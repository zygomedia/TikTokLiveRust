use crate::core::live_client_builder::TikTokLiveBuilder;

pub mod data;
pub mod core;
pub mod http;
pub mod generated;

pub use protobuf;

///
/// # Entry point of library used to create new instance of TikTokLive client
///
/// ```
/// use tiktoklive::TikTokLive;
///
/// let client = TikTokLive::new_client("some-user");
//         .configure(configure)
//         .on_event(on_event)
//         .build();
///   client.connect().await
/// ```
///
pub struct TikTokLive {}

impl TikTokLive {
    ///
    /// Returns builder for creating new TikTokLiveClient
    ///
    pub fn new_client(user_name: &str) -> TikTokLiveBuilder {
        TikTokLiveBuilder::new(user_name)
    }
}
