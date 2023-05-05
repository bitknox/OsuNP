use std::{
    sync::mpsc::{channel, Sender},
    thread::JoinHandle,
    time::Duration,
};

use chrono::Datelike;
use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder,
};

use winit::event_loop::{ControlFlow, EventLoopBuilder};

#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;

#[cfg(target_os = "linux")]
use winit::platform::x11::EventLoopBuilderExtX11;

use crate::error::np_error::OsuNPError;

#[derive(Debug, PartialEq)]
pub enum TrayState {
    Active,
    Waiting,
}

fn load_icon(path: &std::path::Path) -> Result<tray_icon::icon::Icon, OsuNPError> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)?.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Ok(tray_icon::icon::Icon::from_rgba(
        icon_rgba,
        icon_width,
        icon_height,
    )?)
}

pub fn start_sys_tray() -> Result<(Sender<TrayState>, JoinHandle<()>), OsuNPError> {
    let (send, recv) = channel::<TrayState>();
    let handle = std::thread::spawn(move || {
        let waiting_icon = load_icon(std::path::Path::new("./assets/osu_waiting.png")).unwrap();
        let connected_icon = load_icon(std::path::Path::new("./assets/osu_success.png")).unwrap();
        let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
        let tray_menu = Menu::new();
        let quit_i = MenuItem::new("Quit", true, None);
        let current_date = chrono::Utc::now();

        let year = current_date.year();
        tray_menu.append_items(&[
            &PredefinedMenuItem::about(
                None,
                Some(AboutMetadata {
                    name: Some("OsuNowPlaying".to_string()),
                    copyright: Some(format!("©bit_knox - {}", year)),
                    website: Some("https://github.com/bitknox/OsuNP".to_string()),
                    ..Default::default()
                }),
            ),
            &PredefinedMenuItem::separator(),
            &quit_i,
        ]);
        let mut tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("OsuNowPlaying")
            .with_icon(waiting_icon.clone())
            .build()
            .unwrap();

        let menu_channel = MenuEvent::receiver();
        let mut current_state = TrayState::Waiting;
        event_loop.run(move |_event, _, control_flow| {
            *control_flow = ControlFlow::WaitUntil(
                std::time::Instant::now()
                    .checked_add(Duration::from_secs(1))
                    .expect("Expected time to be inside 64bit uint"),
            );

            if let Ok(evt) = recv.try_recv() {
                if evt != current_state {
                    let _ = match evt {
                        TrayState::Active => tray_icon.set_icon(Some(connected_icon.clone())),
                        TrayState::Waiting => tray_icon.set_icon(Some(waiting_icon.clone())),
                    };
                    let _ = tray_icon.set_tooltip(Some(format!("OsuNowPlaying - {:?}", evt)));

                    current_state = evt;
                }
            }

            if let Ok(event) = menu_channel.try_recv() {
                if event.id == quit_i.id() {
                    *control_flow = ControlFlow::Exit;
                }
            }
        })
    });
    Ok((send, handle))
}
