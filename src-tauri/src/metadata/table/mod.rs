use std::collections::HashMap;

use crate::metadata::column::Column;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Table{
pub name: String,
   pub columns: Option<HashMap<String, Column>>,
}