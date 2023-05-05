use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusMessage {
    pub settings: Settings,
    pub menu: Menu,
    pub gameplay: Gameplay,
    pub results_screen: ResultsScreen,
    pub tourney: Tourney,
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
    pub mods: Mods,
    pub pp: Pp,
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
    pub stats: Stats,
    pub path: Path,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    pub first_obj: i64,
    pub current: i64,
    pub full: i64,
    pub mp3: i64,
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
pub struct Stats {
    #[serde(rename = "AR")]
    pub ar: f64,
    #[serde(rename = "CS")]
    pub cs: f64,
    #[serde(rename = "OD")]
    pub od: i64,
    #[serde(rename = "HP")]
    pub hp: i64,
    #[serde(rename = "SR")]
    pub sr: f64,
    #[serde(rename = "BPM")]
    pub bpm: Bpm,
    pub max_combo: i64,
    #[serde(rename = "fullSR")]
    pub full_sr: f64,
    #[serde(rename = "memoryAR")]
    pub memory_ar: f64,
    #[serde(rename = "memoryCS")]
    pub memory_cs: f64,
    #[serde(rename = "memoryOD")]
    pub memory_od: i64,
    #[serde(rename = "memoryHP")]
    pub memory_hp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bpm {
    pub min: i64,
    pub max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub full: String,
    pub folder: String,
    pub file: String,
    pub bg: String,
    pub audio: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mods {
    pub num: i64,
    pub str: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pp {
    #[serde(rename = "100")]
    pub n100: i64,
    #[serde(rename = "99")]
    pub n99: i64,
    #[serde(rename = "98")]
    pub n98: i64,
    #[serde(rename = "97")]
    pub n97: i64,
    #[serde(rename = "96")]
    pub n96: i64,
    #[serde(rename = "95")]
    pub n95: i64,
    pub strains: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gameplay {
    pub game_mode: i64,
    pub name: String,
    pub score: i64,
    pub accuracy: i64,
    pub combo: Combo,
    pub hp: Hp,
    pub hits: Hits,
    pub pp: Pp2,
    pub key_overlay: KeyOverlay,
    pub leaderboard: Leaderboard,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Combo {
    pub current: i64,
    pub max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hp {
    pub normal: i64,
    pub smooth: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hits {
    #[serde(rename = "300")]
    pub n300: i64,
    pub geki: i64,
    #[serde(rename = "100")]
    pub n100: i64,
    pub katu: i64,
    #[serde(rename = "50")]
    pub n50: i64,
    #[serde(rename = "0")]
    pub n0: i64,
    pub slider_breaks: i64,
    pub grade: Grade,
    pub unstable_rate: i64,
    pub hit_error_array: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grade {
    pub current: String,
    pub max_this_play: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pp2 {
    pub current: i64,
    pub fc: i64,
    pub max_this_play: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyOverlay {
    pub k1: K1,
    pub k2: K2,
    pub m1: M1,
    pub m2: M2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct K1 {
    pub is_pressed: bool,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct K2 {
    pub is_pressed: bool,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct M1 {
    pub is_pressed: bool,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct M2 {
    pub is_pressed: bool,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leaderboard {
    pub has_leaderboard: bool,
    pub is_visible: bool,
    pub ourplayer: Ourplayer,
    pub slots: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ourplayer {
    pub name: String,
    pub score: i64,
    pub combo: i64,
    pub max_combo: i64,
    pub mods: String,
    pub h300: i64,
    pub h100: i64,
    pub h50: i64,
    pub h0: i64,
    pub team: i64,
    pub position: i64,
    pub is_passing: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultsScreen {
    pub name: String,
    pub score: i64,
    pub max_combo: i64,
    pub mods: Mods2,
    #[serde(rename = "300")]
    pub n300: i64,
    pub geki: i64,
    #[serde(rename = "100")]
    pub n100: i64,
    pub katu: i64,
    #[serde(rename = "50")]
    pub n50: i64,
    #[serde(rename = "0")]
    pub n0: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mods2 {
    pub num: i64,
    pub str: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tourney {
    pub manager: Manager,
    pub ipc_clients: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manager {
    pub ipc_state: i64,
    #[serde(rename = "bestOF")]
    pub best_of: i64,
    pub team_name: TeamName,
    pub stars: Stars,
    pub bools: Bools,
    pub chat: Value,
    pub gameplay: Gameplay2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamName {
    pub left: String,
    pub right: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stars {
    pub left: i64,
    pub right: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bools {
    pub score_visible: bool,
    pub stars_visible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gameplay2 {
    pub score: Score,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub left: i64,
    pub right: i64,
}
