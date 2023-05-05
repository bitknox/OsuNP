use std::{
    net::TcpStream,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use error::np_error::OsuNPError;

use tungstenite::{stream::MaybeTlsStream, WebSocket};

use crate::{logging::logger::init_logger, systray::np_tray::TrayState};

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

    let pair = systray::np_tray::start_sys_tray()?;
    let sender_handle = Arc::new(Mutex::new(pair));
    let last_updated = Arc::new(AtomicI64::new(0));

    let updater_last_updated = last_updated.clone();
    let updater_thread_sender = sender_handle.clone();

    std::thread::sleep(std::time::Duration::from_secs(5));
    log::info!("[REPORTER] Connecting to websocket...");
    let mut client = connect_websocket();

    log::info!("[REPORTER] Websocket connected!");
    std::thread::spawn(move || {
        let mut last_updated_internal = std::time::Instant::now();

        loop {
            if let Ok(msg) = client.read_message() {
                if !msg.is_text() || last_updated_internal.elapsed().as_secs() < 5 {
                    continue;
                };
                if let Ok(p) = sender_handle.try_lock() {
                    let _ = p.0.send(TrayState::Active);
                }

                match gosumemory::message_parser::handle_update_message(msg, &token) {
                    Ok(_) => {
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards");
                        last_updated
                            .clone()
                            .store(since_the_epoch.as_secs() as i64, Ordering::SeqCst);
                        last_updated_internal = std::time::Instant::now();
                    }
                    Err(e) => log::error!("{}", e),
                }
            } else {
                client = connect_websocket();
            }
        }
    });
    std::thread::spawn(move || loop {
        let i = updater_last_updated.clone().load(Ordering::SeqCst);
        std::thread::sleep(Duration::from_secs(5));

        if let Ok(p) = updater_thread_sender.try_lock() {
            match SystemTime::now().duration_since(UNIX_EPOCH + Duration::from_secs(i as u64)) {
                Ok(n) if n.as_secs() > 10 => {
                    let _ = p.0.send(TrayState::Waiting);
                }
                _ => (),
            };
        };
    })
    .join()
    .unwrap();

    Ok(())
}

fn connect_websocket() -> WebSocket<MaybeTlsStream<TcpStream>> {
    loop {
        match tungstenite::client::connect("ws://localhost:24050/ws") {
            Ok((c, _)) => break c,
            Err(_) => {
                log::debug!(
                    "Could not connect to gosumemory websocket... Retrying in 10 seconds..."
                );
                std::thread::sleep(std::time::Duration::from_secs(10))
            }
        }
    }
}

fn read_token() -> Result<String, OsuNPError> {
    Ok(std::fs::read_to_string("token.txt")?)
}
