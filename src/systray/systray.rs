use chrono::Datelike;
use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

use crate::error::error::OsuNPError;
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

pub fn start_sys_tray() -> Result<(), OsuNPError> {
    let icon = load_icon(std::path::Path::new("./assets/icon.png"))?;
    let event_loop = EventLoopBuilder::new().build();
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
    let _not_unused = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("OsuNowPlaying")
        .with_icon(icon)
        .build()?;

    let menu_channel = MenuEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_i.id() {
                // tray_icon.take();
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}
