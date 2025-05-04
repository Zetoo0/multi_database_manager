use crate::domain::datb::database_type::DatabaseType;
use crate::domain::datb::ddl_generate::GenerateDDL;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    pub name: String,
    pub definition: Option<String>,
    pub column_name: Option<Vec<String>>,
    pub non_unique: Option<bool>,
    pub table_name: Option<String>,
    pub db_name: String,
    pub schema_name: Option<String>,
    pub type_:String,
}

impl GenerateDDL for Index {
    fn to_create_function(&self, target_db_type: DatabaseType) -> Result<String, String> {
        let mut ddl = String::new();
        match target_db_type {
            _ => {
                if let (Some(table_name), Some(non_unique)) = (&self.table_name, self.non_unique) {
                    let unique_clause = if !non_unique { "UNIQUE" } else { "" };

                    // Join the column names for multi-column indexes
                    let columns_list = self.column_name.clone().unwrap().join(", ");

                    // Generate the CREATE INDEX statement
                    ddl.push_str(
                        format!(
                            "CREATE {} INDEX {} ON {} ({});",
                            unique_clause, self.name, table_name, columns_list
                        )
                        .as_str(),
                    );
                }
            }
        }
        Ok(ddl)
    }
}
