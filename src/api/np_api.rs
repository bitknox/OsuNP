use serde::{Deserialize, Serialize};

use crate::error::np_error::OsuNPError;

#[derive(Debug, Serialize)]
struct PostBody {
    token: String,
    data: NowPlaying,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NowPlaying {
    pub token: String,
    pub title: String,
    pub mode: i64,
    pub current_time: i64,
    pub full_time: i64,
    pub beatmap_id: String,
    pub artist: String,
    pub difficulty: String,
    pub difficulty_id: String,
    pub timestamp: u128,
}

pub fn send_update_reqeust(np: NowPlaying) -> Result<reqwest::blocking::Response, OsuNPError> {
    Ok(reqwest::blocking::Client::new()
        .post("https://osu.bitknox.me/playing/update")
        .json(&PostBody {
            token: np.token.to_string(),
            data: np,
        })
        .timeout(std::time::Duration::from_secs(10))
        .send()?)
}
