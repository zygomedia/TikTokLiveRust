use crate::generated::messages::webcast::*;
use crate::generated::messages::webcast::webcast_response::Message;
///
/// Generated file but i deleted like 90% of it
///
pub struct TikTokConnectedEvent {}
pub struct TikTokDisconnectedEvent {}

pub struct TikTokGiftEvent {
    pub raw_data: WebcastGiftMessage,
}
pub struct TikTokWebsocketResponseEvent {
    pub raw_data: WebcastResponse,
}
pub struct TikTokWebsocketUnknownMessageEvent {
    pub message_name: String,
    pub raw_data: Message,
}

pub struct TikTokSubNotifyEvent {
    pub raw_data: WebcastSubNotifyMessage,
}

pub enum TikTokLiveEvent {
    OnConnected(TikTokConnectedEvent),
    OnDisconnected(TikTokDisconnectedEvent),
    OnGift(TikTokGiftEvent),
    OnSubNotify(TikTokSubNotifyEvent),
	OnWebsocketResponse(TikTokWebsocketResponseEvent),
    OnWebsocketUnknownMessage(TikTokWebsocketUnknownMessageEvent),
}
