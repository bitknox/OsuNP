#![windows_subsystem = "windows"]
use native_dialog::{MessageDialog, MessageType};
fn main() {
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        orig_hook(panic_info);
        show_error_dialog(&panic_info.to_string());
        std::process::exit(1);
    }));
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            if let Err(e) = osu_reporter::init() {
                show_error_dialog(&e.to_string())
            }
        })
}

fn show_error_dialog(content: &str) {
    let _ = MessageDialog::new()
        .set_title("fatal error")
        .set_text(content)
        .set_type(MessageType::Error)
        .show_alert();
}
