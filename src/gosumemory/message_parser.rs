use tungstenite::Message;

use crate::{
    api::np_api::{send_update_reqeust, NowPlaying},
    error::np_error::OsuNPError,
};

use super::status_message::StatusMessage;

pub fn handle_update_message(message: Message, token: &str) -> Result<(), OsuNPError> {
    let message: StatusMessage = serde_json::from_str(message.to_text()?)?;

    let currently_playing = NowPlaying {
        token: token.to_string(),
        title: message.menu.bm.metadata.title,
        artist: message.menu.bm.metadata.artist,
        mode: message.menu.game_mode,
        current_time: message.menu.bm.time.current,
        full_time: message.menu.bm.time.full,
        difficulty_id: message.menu.bm.id.to_string(),
        beatmap_id: message.menu.bm.set.to_string(),
        difficulty: message.menu.bm.metadata.difficulty,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis(),
    };
    send_update_reqeust(currently_playing)?;

    Ok(())
}
