use crate::domain::datb::{database_type::DatabaseType, ddl_generate::GenerateDDL};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sequence {
    pub name: String,
    pub start_val: Option<String>,
    pub minimum_val: Option<String>,
    pub maximum_val: Option<String>,
    pub increment: Option<String>,
    pub cycle: Option<String>,
    pub db_name: String,
    pub schema_name: Option<String>,
    pub type_: Option<String>,
}

impl GenerateDDL for Sequence {
    fn to_create_function(&self, target_db_type: DatabaseType) -> Result<String, String> {
        let ddl = format!(
            "CREATE SEQUENCE {}\nSTARTS WITH {}\nINCREMENT BY {}\nMINVALUE {}\nMAXVALUE {};",
            self.name.clone(),
            self.start_val.clone().unwrap(),
            self.increment.clone().unwrap(),
            self.minimum_val.clone().unwrap(),
            self.maximum_val.clone().unwrap()
        );
        Ok(ddl)
    }
}
