use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MateralizedView {
    pub name: String,
    pub definition: String,
}
