use super::layout::layout::Layout;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserSettings {
    #[serde(default)]
    pub layout: Layout,
    #[serde(default = "default_autosave_period", rename = "autosavePeriod")]
    pub autosave_period: u64,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_last_opened_files", rename = "lastOpenedFiles")]
    pub last_opened_files: Vec<String>,
}

fn default_autosave_period() -> u64 {
    5000
}

fn default_language() -> String {
    "en".to_string()
}

fn default_last_opened_files() -> Vec<String> {
    vec![]
}
