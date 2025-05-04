use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForeignDataWrapper {
    pub name: String,
}
