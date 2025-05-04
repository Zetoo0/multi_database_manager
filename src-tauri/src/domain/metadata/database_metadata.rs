use crate::domain::metadata::catalog::Catalog;
use crate::domain::metadata::database::Schema;
use crate::domain::metadata::foreign_data_wrapper::ForeignDataWrapper;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseMetadata {
    pub name: String,
    pub schemas: Option<HashMap<String, Schema>>,
    pub foreign_data_wrappers: Option<HashMap<String, ForeignDataWrapper>>,
    pub catalogs: Option<HashMap<String, Catalog>>,
    pub type_: String,
}

/*
impl Serialize for DatabaseMetadata {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        {
        let mut state = serializer.serialize_struct("DatabaseMetadata", 3)?;
        state.serialize_field("name", &self.name)?;
        let children = vec![
            MetadataNode::new("schemas", self.schemas.iter()),
            MetadataNode::new("foreign_data_wrappers", &self.foreign_data_wrappers),
            MetadataNode::new("catalogs", &self.catalogs),
        ];
        state.serialize_field("children", &children)?;
        state.end()
    }
} */
