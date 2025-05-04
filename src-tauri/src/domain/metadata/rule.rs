use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub definition: String,
}
