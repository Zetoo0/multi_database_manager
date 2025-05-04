use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    #[serde(default, rename(serialize = "themeField", deserialize = "themeField"))]
    pub theme_field: String,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            theme_field: "default_theme_field_value".to_string(),
        }
    }
}
