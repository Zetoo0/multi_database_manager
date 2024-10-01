use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct RlsPolicy{
    pub name: String,
    pub command: String,
}