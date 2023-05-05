custom_error! {
    pub OsuNPError
    TokenRead{source:std::io::Error}      = "Could not read token or gosumemory",
    Image{source: image::ImageError} = "Failed to parse icon image",
    Icon{source: tray_icon::icon::BadIcon} = "Failed to create icon from image",
    JsonRead{token:String} = "Could not read JSON path: {token}",
    API{source: reqwest::Error} = "Could not reach backend",
    ConfigLocation = "Unable to get config location for this system",
    Tray{source: tray_icon::Error} = "Application encountered an error while creation tray icon",
    Websocket{source: tungstenite::Error} = "Could not connect to websocket",
    Serilization{source: serde_json::Error} = "Could not deserialize/serialize json message"
}
