custom_error! {
    pub OsuNPError
    TokenReadError{source:std::io::Error}      = "Could not read token or gosumemory",
    ImageError{source: image::ImageError} = "Failed to parse icon image",
    IconError{source: tray_icon::icon::BadIcon} = "Failed to create icon from image",
    JsonReadError{token:String} = "Could not read JSON path: {token}",
    APIError{source: reqwest::Error} = "Could not reach backend",
    ConfigLocationError = "Unable to get config location for this system",
    TrayError{source: tray_icon::Error} = "Application encountered an error while creation tray icon"
}
