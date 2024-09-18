use crate::column::Column;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Table{
    pub name: String,
    pub columns: Option<Vec<Column>>,
}