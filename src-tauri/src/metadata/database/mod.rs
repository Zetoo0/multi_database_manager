use std::collections::HashMap;

use serde::{Serialize,Deserialize};
use crate::metadata::{column::Column,constraint::Constraint,foreign_data_wrapper::ForeignDataWrapper,function::Function,lock::Lock,materalized_view::MateralizedView,procedure::Procedure,table::Table,trigger::Trigger,user::User,utype::Type,view::View};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Database{
    pub name: String,
    pub functions: Option<HashMap<String,Function>>,
    pub procedures: Option<HashMap<String,Procedure>>,
    pub tables: Option<HashMap<String, Table>>,
    pub views: Option<HashMap<String, View>>,
    pub constraints: Option<HashMap<String, Constraint>>,
    pub foreign_data_wrappers: Option<HashMap<String, ForeignDataWrapper>>,
    pub locks: Option<HashMap<String, Lock>>,
    pub triggers: Option<HashMap<String, Trigger>>,
    pub types: Option<HashMap<String, Type>>
}