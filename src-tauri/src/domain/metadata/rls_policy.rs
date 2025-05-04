use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RlsPolicy {
    pub name: String,
    pub command: String,
}
