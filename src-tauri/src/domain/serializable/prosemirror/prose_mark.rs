use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProseMirrorMark {
    #[serde(rename = "type")]
    pub mark_type: String,
    pub attrs: Option<HashMap<String, String>>, // For link marks (href, title, etc.)
}
