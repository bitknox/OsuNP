const CREATE_NO_WINDOW: u32 = 0x08000000;
use std::process::{Child, Command, Stdio};
#[cfg(target_os = "windows")]
pub fn launch() -> Child {
    use std::os::windows::process::CommandExt;

    Command::new("./osu_memory/gosumemory.exe")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .unwrap()
    //let _guard = ChildGuard(child);
}

//TODO: Create mac and linux version (maybe) :)
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn launch() {
    let _command = Command::new("./osu_memory/gosumemory.exe")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}
