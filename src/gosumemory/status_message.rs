use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusMessage {
    pub settings: Settings,
    pub menu: Menu,
    pub gameplay: Gameplay,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub show_interface: bool,
    pub folders: Folders,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folders {
    pub game: String,
    pub skin: String,
    pub songs: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub main_menu: MainMenu,
    pub state: i64,
    pub game_mode: i64,
    pub is_chat_enabled: i64,
    pub bm: Bm,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainMenu {
    pub bass_density: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bm {
    pub time: Time,
    pub id: i64,
    pub set: i64,
    pub md5: String,
    pub ranked_status: i64,
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    pub current: i64,
    pub full: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub artist: String,
    pub artist_original: String,
    pub title: String,
    pub title_original: String,
    pub mapper: String,
    pub difficulty: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gameplay {
    pub game_mode: i64,
    pub name: String,
}
