use std::io;
use std::time::Duration;

use env_logger::Env;
use log::info;
use tiktoklive::core::live_client::TikTokLiveClient;
use tiktoklive::data::live_common::TikTokLiveSettings;
use tiktoklive::generated::events::TikTokLiveEvent;

use tiktoklive::TikTokLive;

#[tokio::main]
async fn main() {
    let env = Env::default().filter_or("tiktoklive", "info");
    env_logger::init_from_env(env);

    let user_name = "username";
    let client = TikTokLive::new_client(user_name)
        .configure(configure)
        .on_event(handle_event)
        .build();

    client.connect().await;

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() && input.trim() == "stop"
    {
        // client.disconnect();
    }
}

fn handle_event(client: &TikTokLiveClient, event: &TikTokLiveEvent)
{
    match event {
        TikTokLiveEvent::OnMember(joinEvent) =>
            {
                println!("user: {}  joined", joinEvent.raw_data.user.nickname);
            }
        TikTokLiveEvent::OnChat(chatEvent) =>
            {
                println!("user: {} -> {} ", chatEvent.raw_data.user.nickname, chatEvent.raw_data.content);
            }
        TikTokLiveEvent::OnGift(giftEvent) =>
            {
                let nick = &giftEvent.raw_data.user.nickname;
                let gift_name = &giftEvent.raw_data.gift.name;
                let gifts_amount = giftEvent.raw_data.gift.combo;

                println!("user: {} sends gift: {} x {}", nick, gift_name, gifts_amount);
            }
        _ => {}
    }
}

fn configure(settings: &mut TikTokLiveSettings)
{
    settings.http_data.time_out = Duration::from_secs(12);
}