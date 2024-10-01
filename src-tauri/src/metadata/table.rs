use std::collections::HashMap;

use crate::metadata::column::Column;
use crate::metadata::trigger::Trigger;
use crate::metadata::constraint::Constraint;
use crate::metadata::index::Index;
use crate::metadata::rls_policy::RlsPolicy;
use crate::metadata::rule::Rule;
use dashmap::DashMap;
use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Table{
   pub name: String,
   pub columns: Option<HashMap<String, Column>>,
   pub constraints: Option<HashMap<String, Constraint>>,
   pub indexes: Option<HashMap<String, Index>>,
   pub triggers: Option<HashMap<String, Trigger>>,
   pub rules: Option<HashMap<String, Rule>>,
   pub rls_policies: Option<HashMap<String, RlsPolicy>>,
}