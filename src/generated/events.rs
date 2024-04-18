use crate::generated::messages::webcast::*;
use crate::generated::messages::webcast::webcast_response::Message;
///
/// Generated file
///
pub struct TikTokLinkLayerEvent {
    pub raw_data: WebcastLinkLayerMessage,
}
pub struct TikTokRoomVerifyEvent {
    pub raw_data: RoomVerifyMessage,
}
pub struct TikTokBarrageEvent {
    pub raw_data: WebcastBarrageMessage,
}
pub struct TikTokEnvelopeEvent {
    pub raw_data: WebcastEnvelopeMessage,
}
pub struct TikTokPollEvent {
    pub raw_data: WebcastPollMessage,
}
pub struct TikTokChatEvent {
    pub raw_data: WebcastChatMessage,
}
pub struct TikTokSubNotifyEvent {
    pub raw_data: WebcastSubNotifyMessage,
}
pub struct TikTokInRoomBannerEvent {
    pub raw_data: WebcastInRoomBannerMessage,
}
pub struct TikTokLinkMicFanTicketMethodEvent {
    pub raw_data: WebcastLinkMicFanTicketMethod,
}
pub struct TikTokConnectedEvent {}
pub struct TikTokHourlyRankEvent {
    pub raw_data: WebcastHourlyRankMessage,
}
pub struct TikTokPushFrameEvent {
    pub raw_data: WebcastPushFrame,
}
pub struct TikTokResponseEvent {
    pub raw_data: WebcastResponse,
}
pub struct TikTokControlEvent {
    pub raw_data: WebcastControlMessage,
}
pub struct TikTokLinkMicBattleEvent {
    pub raw_data: WebcastLinkMicBattle,
}
pub struct TikTokLinkMicMethodEvent {
    pub raw_data: WebcastLinkMicMethod,
}
pub struct TikTokGiftEvent {
    pub raw_data: WebcastGiftMessage,
}
pub struct TikTokRoomUserSeqEvent {
    pub raw_data: WebcastRoomUserSeqMessage,
}
pub struct TikTokWebsocketUnknownMessageEvent {
    pub message_name: String,
    pub raw_data: Message,
}
pub struct TikTokLikeEvent {
    pub raw_data: WebcastLikeMessage,
}
pub struct TikTokRankUpdateEvent {
    pub raw_data: WebcastRankUpdateMessage,
}
pub struct TikTokUnauthorizedMemberEvent {
    pub raw_data: WebcastUnauthorizedMemberMessage,
}
pub struct TikTokWebsocketResponseEvent {
    pub raw_data: WebcastResponse,
}
pub struct TikTokMsgDetectEvent {
    pub raw_data: WebcastMsgDetectMessage,
}
pub struct TikTokLinkEvent {
    pub raw_data: WebcastLinkMessage,
}
pub struct TikTokRankTextEvent {
    pub raw_data: WebcastRankTextMessage,
}
pub struct TikTokRoomPinEvent {
    pub raw_data: WebcastRoomPinMessage,
}
pub struct TikTokLinkMicBattlePunishFinishEvent {
    pub raw_data: WebcastLinkMicBattlePunishFinish,
}
pub struct TikTokEmoteChatEvent {
    pub raw_data: WebcastEmoteChatMessage,
}
pub struct TikTokLinkmicBattleTaskEvent {
    pub raw_data: WebcastLinkmicBattleTaskMessage,
}
pub struct TikTokMemberEvent {
    pub raw_data: WebcastMemberMessage,
}
pub struct TikTokDisconnectedEvent {}
pub struct TikTokRoomEvent {
    pub raw_data: WebcastRoomMessage,
}
pub struct TikTokImDeleteEvent {
    pub raw_data: WebcastImDeleteMessage,
}
pub struct TikTokQuestionNewEvent {
    pub raw_data: WebcastQuestionNewMessage,
}
pub struct TikTokOecLiveShoppingEvent {
    pub raw_data: WebcastOecLiveShoppingMessage,
}
pub struct TikTokSocialEvent {
    pub raw_data: WebcastSocialMessage,
}
pub struct TikTokSystemEvent {
    pub raw_data: WebcastSystemMessage,
}
pub struct TikTokLinkMicArmiesEvent {
    pub raw_data: WebcastLinkMicArmies,
}
pub struct TikTokLiveIntroEvent {
    pub raw_data: WebcastLiveIntroMessage,
}
pub struct TikTokCaptionEvent {
    pub raw_data: WebcastCaptionMessage,
}
pub struct TikTokGoalUpdateEvent {
    pub raw_data: WebcastGoalUpdateMessage,
}
pub enum TikTokLiveEvent {
    OnLike(TikTokLikeEvent),
    OnRankUpdate(TikTokRankUpdateEvent),
    OnGoalUpdate(TikTokGoalUpdateEvent),
    OnImDelete(TikTokImDeleteEvent),
    OnLinkMicFanTicketMethod(TikTokLinkMicFanTicketMethodEvent),
    OnControl(TikTokControlEvent),
    OnDisconnected(TikTokDisconnectedEvent),
    OnWebsocketUnknownMessage(TikTokWebsocketUnknownMessageEvent),
    OnLinkmicBattleTask(TikTokLinkmicBattleTaskEvent),
    OnRoomPin(TikTokRoomPinEvent),
    OnSystem(TikTokSystemEvent),
    OnSocial(TikTokSocialEvent),
    OnRankText(TikTokRankTextEvent),
    OnLiveIntro(TikTokLiveIntroEvent),
    OnLink(TikTokLinkEvent),
    OnConnected(TikTokConnectedEvent),
    OnLinkLayer(TikTokLinkLayerEvent),
    OnUnauthorizedMember(TikTokUnauthorizedMemberEvent),
    OnLinkMicBattle(TikTokLinkMicBattleEvent),
    OnHourlyRank(TikTokHourlyRankEvent),
    OnPushFrame(TikTokPushFrameEvent),
    OnLinkMicBattlePunishFinish(TikTokLinkMicBattlePunishFinishEvent),
    OnMsgDetect(TikTokMsgDetectEvent),
    OnOecLiveShopping(TikTokOecLiveShoppingEvent),
    OnResponse(TikTokResponseEvent),
    OnRoomVerify(TikTokRoomVerifyEvent),
    OnRoom(TikTokRoomEvent),
    OnLinkMicMethod(TikTokLinkMicMethodEvent),
    OnLinkMicArmies(TikTokLinkMicArmiesEvent),
    OnSubNotify(TikTokSubNotifyEvent),
    OnQuestionNew(TikTokQuestionNewEvent),
    OnGift(TikTokGiftEvent),
    OnInRoomBanner(TikTokInRoomBannerEvent),
    OnRoomUserSeq(TikTokRoomUserSeqEvent),
    OnEmoteChat(TikTokEmoteChatEvent),
    OnMember(TikTokMemberEvent),
    OnWebsocketResponse(TikTokWebsocketResponseEvent),
    OnCaption(TikTokCaptionEvent),
    OnChat(TikTokChatEvent),
    OnBarrage(TikTokBarrageEvent),
    OnEnvelope(TikTokEnvelopeEvent),
    OnPoll(TikTokPollEvent),
}
