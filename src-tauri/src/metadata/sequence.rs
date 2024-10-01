use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Sequence{
    pub name: String,
}