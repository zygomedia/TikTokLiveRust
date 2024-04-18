use protobuf::Message;
use crate::core::live_client::TikTokLiveClient;
use crate::generated::messages::webcast::*;
use crate::core::live_client_mapper::TikTokLiveMessageMapper;
use crate::generated::events::*;
impl TikTokLiveMessageMapper {
    pub fn handle_single_message(
        &self,
        message: crate::generated::messages::webcast::webcast_response::Message,
        client: &TikTokLiveClient,
    ) {
        let proto_message_name = &message.method;
        let proto_message_content = &message.payload;
        match proto_message_name.as_str() {
            "WebcastGiftMessage" => {
                let raw_data = WebcastGiftMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokGiftEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnGift(event));
            },
            "WebcastSubNotifyMessage" => {
                let raw_data = WebcastSubNotifyMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokSubNotifyEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnSubNotify(event));
            },
            _ => {
                client
                    .publish_event(
                        TikTokLiveEvent::OnWebsocketUnknownMessage(TikTokWebsocketUnknownMessageEvent {
                            message_name: message.method.clone(),
                            raw_data: message,
                        }),
                    );
            }
        }
    }
}
