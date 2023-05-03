use error::error::OsuNPError;

use crate::logging::logger::init_logger;

#[macro_use]
extern crate custom_error;

mod api;
mod error;
mod gosumemory;
mod logging;
mod systray;

pub fn init() -> Result<(), OsuNPError> {
    let token = read_token()?;
    match init_logger(log::LevelFilter::Info) {
        Ok(_) => log::info!("logger initialized"),
        Err(_) => println!("Unable to initialize logger"),
    };
    gosumemory::launcher::launch();

    std::thread::sleep(std::time::Duration::from_secs(5));
    log::info!("[REPORTER] Connecting to websocket...");
    let mut client;
    loop {
        match tungstenite::client::connect("ws://localhost:24050/ws") {
            Ok((c, _)) => {
                client = c;
                break;
            }
            Err(_) => {
                log::debug!(
                    "Could not connect to gosumemory websocket... Retrying in 10 seconds..."
                );
                std::thread::sleep(std::time::Duration::from_secs(10))
            }
        }
    }

    log::info!("[REPORTER] Websocket connected!");
    std::thread::spawn(move || {
        let mut last_updated = std::time::Instant::now();
        loop {
            let msg = client.read_message().unwrap();
            if !msg.is_text() || last_updated.elapsed().as_secs() < 5 {
                continue;
            };
            match gosumemory::message_parser::handle_update_message(msg, &token) {
                Ok(_) => last_updated = std::time::Instant::now(),
                Err(e) => log::error!("{}", e),
            }
        }
    });
    match systray::systray::start_sys_tray() {
        Ok(()) => (),
        Err(e) => log::error!("{}", e),
    };
    Ok(())
}

fn read_token() -> Result<String, OsuNPError> {
    Ok(std::fs::read_to_string("token.txt")?)
}
