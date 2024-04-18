use std::time::Duration;

use log::info;
use tiktoklive::core::live_client::TikTokLiveClient;
use tiktoklive::data::live_common::TikTokLiveSettings;
use tiktoklive::generated::events::TikTokLiveEvent;

use tiktoklive::TikTokLive;

#[tokio::main]
async fn main() {
	env_logger::Builder::from_default_env()
		.filter(Some("tiktoklive"), log::LevelFilter::Debug)
		.init();

    let user_name = "username";
    let client = TikTokLive::new_client(user_name)
        .configure(configure)
        .on_event(handle_event)
        .build();

    client.connect().await;
}

fn handle_event(_client: &TikTokLiveClient, event: &TikTokLiveEvent) {
    match event {
        // TikTokLiveEvent::OnMember(join_event) => {
			// info!("user: {}  joined", join_event.raw_data.user.nickname);
		// },
        // TikTokLiveEvent::OnChat(chat_event) => {
			// info!("user: {} -> {} ", chat_event.raw_data.user.nickname, chat_event.raw_data.content);
        // },
        TikTokLiveEvent::OnGift(gift_event) => {
			let nick = &gift_event.raw_data.user.nickname;
			let gift_name = &gift_event.raw_data.gift.name;
			let gifts_amount = gift_event.raw_data.gift.combo;

			info!("user: {} sends gift: {} x {}", nick, gift_name, gifts_amount);
		},
        _ => {},
    }
}

fn configure(settings: &mut TikTokLiveSettings) {
    settings.http_data.time_out = Duration::from_secs(12);
}
