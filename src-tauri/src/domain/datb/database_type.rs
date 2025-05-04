use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DatabaseType {
    MySql,
    Postgres,
    MsSql,
    Sqlite,
    Oracle,
}

impl std::fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseType::MySql => write!(f, "MySql"),
            DatabaseType::Postgres => write!(f, "Postgres"),
            DatabaseType::MsSql => write!(f, "MsSql"),
            DatabaseType::Sqlite => write!(f, "Sqlite"),
            DatabaseType::Oracle => write!(f, "Oracle"),
        }
    }
}
