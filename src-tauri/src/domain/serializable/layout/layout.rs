use super::{sidebar::Sidebar, theme::Theme};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Layout {
    #[serde(default)]
    pub sidebar: Sidebar,
    #[serde(default)]
    pub theme: Theme,
}
