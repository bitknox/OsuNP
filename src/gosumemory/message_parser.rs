use serde_json::Value;
use tungstenite::Message;

use crate::{
    api::np_api::{send_update_reqeust, NowPlaying},
    error::error::OsuNPError,
};

pub fn handle_update_message(message: Message, token: &str) -> Result<(), OsuNPError> {
    let message: Value = serde_json::from_str(message.to_text().unwrap()).unwrap();

    let menu = message.get("menu").ok_or(OsuNPError::JsonReadError {
        token: "menu".to_string(),
    })?;

    let beatmap_info: &Value = menu.get("bm").ok_or(OsuNPError::JsonReadError {
        token: "bm".to_string(),
    })?;

    let time: &Value = beatmap_info.get("time").ok_or(OsuNPError::JsonReadError {
        token: "time".to_string(),
    })?;
    let curr_time = get_i64(&time, "current")?;

    let max_time = get_i64(&time, "mp3")?;

    let meta_data = beatmap_info
        .get("metadata")
        .ok_or(OsuNPError::JsonReadError {
            token: "metadata".to_string(),
        })?;

    let currently_playing = NowPlaying {
        token: token.to_string(),
        title: get_string(&meta_data, "title")?,
        artist: get_string(&meta_data, "artist")?,
        mode: get_i64(&menu, "gameMode")?,
        current_time: curr_time,
        full_time: max_time,
        difficulty_id: get_i64(&beatmap_info, "id")?.to_string(),
        beatmap_id: get_i64(&beatmap_info, "set")?.to_string(),
        difficulty: get_string(&meta_data, "difficulty")?,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis(),
    };
    send_update_reqeust(currently_playing)?;
    Ok(())
}

fn get_i64(value: &Value, key: &str) -> Result<i64, OsuNPError> {
    value
        .get(key)
        .and_then(|v| v.as_i64())
        .ok_or(OsuNPError::JsonReadError {
            token: key.to_string(),
        })
}

fn get_string(val: &Value, key: &str) -> Result<String, OsuNPError> {
    Ok(val
        .get(key)
        .ok_or(OsuNPError::JsonReadError {
            token: key.to_string(),
        })?
        .as_str()
        .ok_or(OsuNPError::JsonReadError {
            token: key.to_string(),
        })?
        .to_string())
}
