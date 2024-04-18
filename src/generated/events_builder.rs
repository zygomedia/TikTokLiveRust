use crate::core::live_client::TikTokLiveClient;
use crate::core::live_client_builder::TikTokLiveBuilder;
use crate::generated::events::*;
///
///  Generated code
///
///
impl TikTokLiveBuilder {
    pub fn on_gift(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokGiftEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| {
            match incoming_event {
                TikTokLiveEvent::OnGift(event_instance) => {
                    on_event(client, event_instance);
                }
                _ => {}
            }
        })
    }
    pub fn on_sub_notify(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokSubNotifyEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| {
            match incoming_event {
                TikTokLiveEvent::OnSubNotify(event_instance) => {
                    on_event(client, event_instance);
                }
                _ => {}
            }
        })
    }
}
