use crate::domain::datb::database_type::DatabaseType;
use crate::domain::datb::ddl_generate::GenerateDDL;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct View {
    pub name: String,
    pub definition: String,
    pub type_: String,
    pub db_name: String,
    pub schema_name: String
}

impl GenerateDDL for View {
    fn to_create_function(&self, target_db_type: DatabaseType) -> Result<String, String> {
        let mut ddl = String::new();
        match target_db_type {
            DatabaseType::MsSql => {
                ddl.push_str(
                    format!(
                        "CREATE VIEW {} WITH SCHEMABINDING AS {}",
                        self.name, self.definition
                    )
                    .as_str(),
                );
            }
            DatabaseType::MySql | DatabaseType::Postgres | DatabaseType::Sqlite => {
                ddl.push_str(format!("CREATE VIEW {} AS {}", self.name, self.definition).as_str());
            }
            DatabaseType::Oracle => {
                ddl.push_str(
                    format!(
                        "CREATE OR REPLACE VIEW {} AS {}",
                        self.name, self.definition
                    )
                    .as_str(),
                );
            }
            _ => {
                return Err("Unsupported database type".to_string());
            }
        }
        Ok(ddl)
    }
}
