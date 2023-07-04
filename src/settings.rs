use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub enabled: bool,

    #[serde(rename = "autoLaunch")]
    pub auto_launch: bool,

    #[serde(rename = "mediaKeys")]
    pub media_keys: bool,

    #[serde(rename = "titleMenubar")]
    pub title_menubar: bool,
}

// TODO: should we save settings (or at least provide the option to)?
