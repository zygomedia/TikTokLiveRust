use protobuf::Message;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use tokio_tungstenite::tungstenite::connect;
use tokio_tungstenite::tungstenite::handshake::client::Request;

use crate::core::live_client::TikTokLiveClient;
use crate::core::live_client_mapper::TikTokLiveMessageMapper;
use crate::data::live_common::ConnectionState::CONNECTED;
use crate::errors::LibError;
use crate::generated::events::{TikTokConnectedEvent, TikTokLiveEvent};
use crate::generated::messages::webcast::{WebcastPushFrame, WebcastResponse};
use crate::http::http_data::LiveConnectionDataResponse;

pub struct TikTokLiveWebsocketClient {
    pub(crate) message_mapper: TikTokLiveMessageMapper,
    pub(crate) running: Arc<AtomicBool>,
}

impl TikTokLiveWebsocketClient {
    pub fn new(message_mapper: TikTokLiveMessageMapper) -> Self {
        TikTokLiveWebsocketClient {
            message_mapper,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn start(
        &self,
        response: LiveConnectionDataResponse,
        client: Arc<TikTokLiveClient>,
    ) -> Result<(), LibError> {
        let host = response
            .web_socket_url
            .host_str()
            .ok_or(LibError::InvalidHost)?;

        let request = Request::builder()
            .method("GET")
            .uri(response.web_socket_url.to_string())
            .header("Host", host)
            .header("Upgrade", "websocket")
            .header("Connection", "keep-alive")
            .header("Cache-Control", "max-age=0")
            .header("Accept", "text/html,application/json,application/protobuf")
            .header("Sec-Websocket-Key", "asd")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36")
            .header("Referer", "https://www.tiktok.com/")
            .header("Origin", "https://www.tiktok.com")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Accept-Encoding", "gzip, deflater")
            .header("Cookie", response.web_socket_cookies)
            .header("Sec-Websocket-Version", "13")
            .body(())
            .map_err(|_| LibError::ParamsError)?;

        let (mut socket, _) = connect(request).map_err(|_| LibError::WebSocketConnectFailed)?;

        client.set_connection_state(CONNECTED);
        client.publish_event(TikTokLiveEvent::OnConnected(TikTokConnectedEvent {}));

        let running = self.running.clone();
        running.store(true, Ordering::SeqCst);

        let message_mapper = self.message_mapper.clone();

        thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                let optional_message = socket.read_message();

                if let Ok(message) = optional_message {
                    let buffer = message.into_data();

                    let mut push_frame = match WebcastPushFrame::parse_from_bytes(buffer.as_slice())
                    {
                        Ok(frame) => frame,
                        Err(_) => continue,
                    };

                    let webcast_response = match WebcastResponse::parse_from_bytes(
                        push_frame.Payload.as_mut_slice(),
                    ) {
                        Ok(response) => response,
                        Err(_) => continue,
                    };

                    if webcast_response.needsAck {
                        let mut push_frame_ack = WebcastPushFrame::new();
                        push_frame_ack.PayloadType = "ack".to_string();
                        push_frame_ack.LogId = push_frame.LogId;
                        push_frame_ack.Payload = webcast_response.internalExt.clone().into_bytes();

                        let binary = match push_frame_ack.write_to_bytes() {
                            Ok(bytes) => bytes,
                            Err(_) => continue,
                        };

                        let message = tungstenite::protocol::Message::binary(binary);
                        if socket.write_message(message).is_err() {
                            continue;
                        }
                    }

                    message_mapper.handle_webcast_response(webcast_response, client.as_ref());
                }
            }
        });

        Ok(())
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}
