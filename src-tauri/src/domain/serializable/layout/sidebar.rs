use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sidebar {
    #[serde(
        default,
        rename(serialize = "sidebarField", deserialize = "sidebarField")
    )]
    pub sidebar_field: String,
}

impl Default for Sidebar {
    fn default() -> Self {
        Sidebar {
            sidebar_field: "default_sidebar_field_value".to_string(),
        }
    }
}
