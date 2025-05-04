use crate::domain::repository::file_repository::get_config_dir;
use crate::domain::serializable::response::{DataType, ErrorBody, ResponseBody};
use crate::domain::serializable::user_settings::UserSettings;
use log::debug;
use std::error::Error;

#[tauri::command]
pub fn get_user_settings() -> ResponseBody {
    debug!("Getting user settings");
    let user_settings = match read() {
        Ok(user_settings) => user_settings,
        Err(_) => serde_json::Value::Null,
    };

    return ResponseBody::new(DataType::Object(
        serde_json::to_value(user_settings).unwrap(),
    ));
}

#[tauri::command]
pub fn get_default_user_settings() -> ResponseBody {
    let default_user_settings = UserSettings::default();
    return ResponseBody::new(DataType::Object(
        serde_json::to_value(default_user_settings).unwrap(),
    ));
}

#[tauri::command]
pub fn save_user_settings(settings: serde_json::Value) -> ResponseBody {
    debug!("Saving user settings");
    match save(settings) {
        Ok(_) => ResponseBody::new(DataType::Object(serde_json::Value::String(
            "User settings saved".to_string(),
        ))),
        Err(_) => ResponseBody::new_with_error(ErrorBody::new(
            500,
            "Failed to save user settings".to_string(),
        )),
    }
}

pub fn read() -> Result<serde_json::Value, Box<dyn Error>> {
    let path = get_config_dir().join("usersettings.json");
    debug!("reading file: {:?}", path);
    let contents = std::fs::read_to_string(path)?;
    return Ok(serde_json::from_str(&contents)?);
}

pub fn save(user_settings: serde_json::Value) -> Result<(), Box<dyn Error>> {
    let path = get_config_dir().join("usersettings.json");
    debug!("writing file: {:?}", path);
    std::fs::write(path, serde_json::to_string_pretty(&user_settings)?)?;
    return Ok(());
}
