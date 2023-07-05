use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Presence {
    /// Client ID of presence
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// Tray title to be shown in Mac OS tray
    #[serde(rename = "trayTitle")]
    pub tray_title: String,
    /// Determines if the service is currently playing something back or not, if false it will automatically hide after 1 minute
    pub playback: bool,
    /// Discord Presence which gets sent directly to Discord app
    #[serde(rename = "presenceData")]
    pub presence_data: PresenceData,
    /// Determines if the service should be hidden (clearActivity)
    pub hidden: Option<bool>,
    /// Determines if the service is mediaKey able / uses them
    #[serde(rename = "mediaKeys")]
    pub media_keys: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PresenceData {
    pub state: Option<String>,

    pub details: Option<String>,

    #[serde(rename = "startTimestamp")]
    pub start_timestamp: Option<i64>,

    #[serde(rename = "endTimestamp")]
    pub end_timestamp: Option<i64>,

    #[serde(rename = "largeImageKey")]
    pub large_image_key: Option<String>,

    #[serde(rename = "largeImageText")]
    pub large_image_text: Option<String>,

    #[serde(rename = "smallImageKey")]
    pub small_image_key: Option<String>,

    #[serde(rename = "smallImageText")]
    pub small_image_text: Option<String>,

    pub instance: Option<bool>,

    #[serde(rename = "partyId")]
    pub party_id: Option<String>,

    #[serde(rename = "partySize")]
    pub party_size: Option<i32>,

    #[serde(rename = "partyMax")]
    pub party_max: Option<i32>,

    #[serde(rename = "matchSecret")]
    pub match_secret: Option<String>,

    #[serde(rename = "spectateSecret")]
    pub spectate_secret: Option<String>,

    #[serde(rename = "joinSecret")]
    pub join_secret: Option<String>,

    pub buttons: Option<Vec<Button>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Button {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscordUser {
    avatar: String,
    bot: bool,
    discriminator: String,
    flags: u32,
    id: String,
    premium_type: u8,
    username: String,
}
