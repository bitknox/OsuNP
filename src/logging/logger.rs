use directories::ProjectDirs;
use log::LevelFilter;

use crate::error::error::OsuNPError;

pub fn init_logger(level_filter: LevelFilter) -> Result<(), OsuNPError> {
    if let Some(proj_dirs) = ProjectDirs::from("me", "bitknox", "osu") {
        let log_path = proj_dirs
            .config_dir()
            .parent()
            .map(|p| p.join("log.txt"))
            .ok_or(OsuNPError::ConfigLocationError)?;
        std::fs::create_dir_all(log_path.parent().ok_or(OsuNPError::ConfigLocationError)?)?;
        Ok(simple_logging::log_to_file(log_path, level_filter).unwrap())
    } else {
        Err(OsuNPError::ConfigLocationError)
    }
}
