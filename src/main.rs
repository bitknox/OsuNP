use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs::{self},
    process::Command,
    thread,
};
use tungstenite::Message;

fn main() {
    let token = read_token();
    Command::new("./osu_memory/gosumemory.exe").spawn().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("[REPORTER] Connecting to websocket...");
    let (mut client, _) = tungstenite::client::connect("ws://localhost:24050/ws").unwrap();
    println!("[REPORTER] Websocket connected!");
    thread::spawn(move || {
        let mut last_updated = std::time::Instant::now();

        loop {
            let msg = client.read_message().unwrap();
            if !msg.is_text() || last_updated.elapsed().as_secs() < 5 {
                continue;
            };
            match handle_update_message(msg, &token) {
                Ok(_) => last_updated = std::time::Instant::now(),
                Err(e) => println!("Error: {:?}", e),
            }
        }
    })
    .join()
    .unwrap();

    println!("[REPORTER] Websocket connected!");
}

fn handle_update_message(message: Message, token: &str) -> Result<(), HandleError> {
    let message: Value = serde_json::from_str(message.to_text().unwrap()).unwrap();

    let beatmap_info: &Value = message
        .get("menu")
        .and_then(|v| v.get("bm"))
        .ok_or(HandleError::new("bm"))?;

    let time: &Value = beatmap_info.get("time").ok_or(HandleError::new("time"))?;
    let curr_time = get_i64(&time, "current")?;

    let max_time = get_i64(&time, "mp3")?;

    let meta_data = beatmap_info
        .get("metadata")
        .ok_or(HandleError::new("metadata"))?;

    let currently_playing = NowPlaying {
        token: token.to_string(),
        title: get_string(&meta_data, "title")?,
        artist: get_string(&meta_data, "artist")?,
        current_time: curr_time,
        full_time: max_time,
        beatmap_id: get_i64(&beatmap_info, "set")?.to_string(),
        difficulty: get_string(&meta_data, "difficulty")?,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis(),
    };
    send_update_reqeust(currently_playing);

    Ok(())
}

fn send_update_reqeust(np: NowPlaying) -> bool {
    reqwest::blocking::Client::new()
        .post("https://osu.bitknox.me/playing/update")
        .json(&PostBody {
            token: np.token.to_string(),
            data: np,
        })
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .is_ok()
}

fn get_i64(value: &Value, key: &str) -> Result<i64, HandleError> {
    value
        .get(key)
        .and_then(|v| v.as_i64())
        .ok_or(HandleError::new(key))
}

fn get_string(val: &Value, key: &str) -> Result<String, HandleError> {
    Ok(val
        .get(key)
        .ok_or(HandleError::new(key))?
        .as_str()
        .ok_or(HandleError::new(key))?
        .to_string())
}

fn read_token() -> String {
    fs::read_to_string("token.txt").unwrap().to_string()
}

#[derive(Debug, Clone)]
enum HandleError {
    Token(String),
}
impl HandleError {
    fn new(token: &str) -> HandleError {
        HandleError::Token(token.to_string())
    }
}

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
    #[serde(rename(serialize = "currentTime"))]
    pub current_time: i64,
    #[serde(rename(serialize = "fullTime"))]
    pub full_time: i64,
    #[serde(rename(serialize = "beatmapId"))]
    pub beatmap_id: String,
    pub artist: String,
    pub difficulty: String,
    pub timestamp: u128,
}
