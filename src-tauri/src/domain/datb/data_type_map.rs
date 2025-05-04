use crate::domain::datb::database_type::DatabaseType;
use regex::Regex;
pub fn data_type_map(data_type: &str, target_db_type: DatabaseType) -> String {
    let re = Regex::new(r"\(.+\)").unwrap(); // Matches the parentheses and anything inside
    let data_type_cleaned = re.replace(data_type, "").to_uppercase(); // Remove length
    let dtype = data_type_cleaned.as_str();
    match (dtype, target_db_type) {
        // PostgreSQL Type Mappings
        ("SMALLINT", DatabaseType::Postgres) => "SMALLINT".to_string(),
        ("INTEGER", DatabaseType::Postgres) => "INT".to_string(),
        ("BIGINT", DatabaseType::Postgres) => "BIGINT".to_string(),
        ("DECIMAL", DatabaseType::Postgres) => "NUMERIC".to_string(),
        ("REAL", DatabaseType::Postgres) => "FLOAT".to_string(),
        ("DOUBLE PRECISION", DatabaseType::Postgres) => "FLOAT8".to_string(),

        // MySQL Type Mappings
        ("SMALLINT", DatabaseType::MySql) => "SMALLINT".to_string(),
        ("INTEGER", DatabaseType::MySql) => "INT".to_string(),
        ("BIGINT", DatabaseType::MySql) => "BIGINT".to_string(),
        ("DECIMAL", DatabaseType::MySql) => "DECIMAL".to_string(),
        ("REAL", DatabaseType::MySql) => "FLOAT".to_string(),
        ("DOUBLE PRECISION", DatabaseType::MySql) => "DOUBLE".to_string(),

        // MSSQL Type Mappings
        ("SMALLINT", DatabaseType::MsSql) => "SMALLINT".to_string(),
        ("INTEGER", DatabaseType::MsSql) => "INT".to_string(),
        ("BIGINT", DatabaseType::MsSql) => "BIGINT".to_string(),
        ("DECIMAL", DatabaseType::MsSql) => "DECIMAL".to_string(),
        ("REAL", DatabaseType::MsSql) => "REAL".to_string(),
        ("FLOAT", DatabaseType::MsSql) => "FLOAT".to_string(),

        // SQLite Type Mappings
        ("SMALLINT", DatabaseType::Sqlite) => "INTEGER".to_string(),
        ("INTEGER", DatabaseType::Sqlite) => "INTEGER".to_string(),
        ("BIGINT", DatabaseType::Sqlite) => "INTEGER".to_string(),
        ("DECIMAL", DatabaseType::Sqlite) => "REAL".to_string(),
        ("REAL", DatabaseType::Sqlite) => "REAL".to_string(),
        ("FLOAT", DatabaseType::Sqlite) => "REAL".to_string(),

        // Oracle Type Mappings
        ("SMALLINT", DatabaseType::Oracle) => "NUMBER(38)".to_string(),
        ("INTEGER", DatabaseType::Oracle) => "NUMBER(38)".to_string(),
        ("BIGINT", DatabaseType::Oracle) => "NUMBER(38)".to_string(),
        ("DECIMAL", DatabaseType::Oracle) => "NUMBER".to_string(),
        ("FLOAT", DatabaseType::Oracle) => "FLOAT".to_string(),
        ("REAL", DatabaseType::Oracle) => "FLOAT".to_string(),

        // String Type Mappings for all DBs
        ("VARCHAR" | "CHARACTER VARYING", DatabaseType::MySql) => "VARCHAR".to_string(),
        ("VARCHAR" | "CHARACTER VARYING", DatabaseType::MsSql) => "NVARCHAR".to_string(),
        ("VARCHAR" | "CHARACTER VARYING", DatabaseType::Sqlite) => "TEXT".to_string(),
        ("VARCHAR" | "CHARACTER VARYING", DatabaseType::Oracle) => "VARCHAR2".to_string(),
        ("VARCHAR" | "CHARACTER VARYING", DatabaseType::Postgres) => "VARCHAR".to_string(),

        ("NVARCHAR" | "CHARACTER VARYING", DatabaseType::MySql) => "VARCHAR".to_string(),
        ("NVARCHAR" | "CHARACTER VARYING", DatabaseType::MsSql) => "NVARCHAR".to_string(),
        ("NVARCHAR" | "CHARACTER VARYING", DatabaseType::Sqlite) => "TEXT".to_string(),
        ("NVARCHAR" | "CHARACTER VARYING", DatabaseType::Oracle) => "VARCHAR2".to_string(),
        ("NVARCHAR" | "CHARACTER VARYING", DatabaseType::Postgres) => "VARCHAR".to_string(),

        ("VARCHAR2" | "CHARACTER VARYING", DatabaseType::MySql) => "VARCHAR".to_string(),
        ("VARCHAR2" | "CHARACTER VARYING", DatabaseType::MsSql) => "NVARCHAR".to_string(),
        ("VARCHAR2" | "CHARACTER VARYING", DatabaseType::Sqlite) => "TEXT".to_string(),
        ("VARCHAR2" | "CHARACTER VARYING", DatabaseType::Oracle) => "VARCHAR2".to_string(),
        ("VARCHAR2" | "CHARACTER VARYING", DatabaseType::Postgres) => "VARCHAR".to_string(),

        ("TEXT", DatabaseType::MySql) => "TEXT".to_string(),
        ("TEXT", DatabaseType::MsSql) => "NVARCHAR(MAX)".to_string(),
        ("TEXT", DatabaseType::Sqlite) => "TEXT".to_string(),
        ("TEXT", DatabaseType::Oracle) => "CLOB".to_string(),
        ("TEXT", DatabaseType::Postgres) => "TEXT".to_string(),

        ("CHAR" | "CHARACTER", DatabaseType::MySql) => "CHAR".to_string(),
        ("CHAR" | "CHARACTER", DatabaseType::MsSql) => "NCHAR".to_string(),
        ("CHAR" | "CHARACTER", DatabaseType::Sqlite) => "TEXT".to_string(),
        ("CHAR" | "CHARACTER", DatabaseType::Oracle) => "CHAR".to_string(),
        ("CHAR" | "CHARACTER", DatabaseType::Postgres) => "CHAR".to_string(),

        ("BYTEA", DatabaseType::MySql) => "BLOB".to_string(),
        ("BYTEA", DatabaseType::MsSql) => "VARBINARY(MAX)".to_string(),
        ("BYTEA", DatabaseType::Sqlite) => "BLOB".to_string(),
        ("BYTEA", DatabaseType::Oracle) => "BLOB".to_string(),
        ("BYTEA", DatabaseType::Postgres) => "BYTEA".to_string(),

        // Date/Time Type Mappings
        ("DATE", DatabaseType::MySql) => "DATE".to_string(),
        ("TIMESTAMP", DatabaseType::MySql) => "TIMESTAMP".to_string(),
        ("TIME", DatabaseType::MySql) => "TIME".to_string(),
        ("TIMESTAMP WITHOUT TIME ZONE", DatabaseType::MySql) => "TIMESTAMP".to_string(),
        ("TIMESTAMP WITH TIME ZONE", DatabaseType::MySql) => "TIMESTAMP".to_string(),

        ("DATE", DatabaseType::MsSql) => "DATETIME".to_string(),
        ("TIMESTAMP", DatabaseType::MsSql) => "DATETIME2".to_string(),
        ("TIME", DatabaseType::MsSql) => "TIME".to_string(),
        ("TIMESTAMP WITHOUT TIME ZONE", DatabaseType::MsSql) => "DATETIME2".to_string(),
        ("TIMESTAMP WITH TIME ZONE", DatabaseType::MsSql) => "DATETIME2".to_string(),

        ("DATE", DatabaseType::Sqlite) => "TEXT".to_string(),
        ("TIMESTAMP", DatabaseType::Sqlite) => "TEXT".to_string(),
        ("TIME", DatabaseType::Sqlite) => "TEXT".to_string(),
        ("TIMESTAMP WITHOUT TIME ZONE", DatabaseType::Sqlite) => "TEXT".to_string(),
        ("TIMESTAMP WITH TIME ZONE", DatabaseType::Sqlite) => "TEXT".to_string(),

        ("DATE", DatabaseType::Oracle) => "DATE".to_string(),
        ("TIMESTAMP", DatabaseType::Oracle) => "TIMESTAMP".to_string(),
        ("TIME", DatabaseType::Oracle) => "TIMESTAMP".to_string(),
        ("TIMESTAMP WITHOUT TIME ZONE", DatabaseType::MySql) => "TIMESTAMP".to_string(),
        ("TIMESTAMP WITH TIME ZONE", DatabaseType::MySql) => "TIMESTAMP".to_string(),

        ("DATE", DatabaseType::Postgres) => "DATE".to_string(),
        ("TIMESTAMP", DatabaseType::Postgres) => "TIMESTAMP".to_string(),
        ("TIME", DatabaseType::Postgres) => "TIME".to_string(),

        // Boolean Types
        ("BOOLEAN", DatabaseType::MySql) => "TINYINT(1)".to_string(),
        ("BOOLEAN", DatabaseType::MsSql) => "BIT".to_string(),
        ("BOOLEAN", DatabaseType::Sqlite) => "INTEGER".to_string(),
        ("BOOLEAN", DatabaseType::Oracle) => "NUMBER(1)".to_string(),
        ("BIT", DatabaseType::Oracle) => "NUMBER(1)".to_string(),
        ("TINYINT(1)", DatabaseType::Oracle) => "NUMBER(1)".to_string(),
        ("BOOLEAN", DatabaseType::Oracle) => "NUMBER(1)".to_string(),

        ("IN", DatabaseType::MsSql) => "".to_string(),
        ("OUT", DatabaseType::MsSql) => "OUTPUT".to_string(),

        _ => {
            return data_type.to_string();
        }
    }
}

pub fn map_full_postgres_to_mysql(body: &str) -> String {
    let mut _body = body.to_string();
    for word in body.split_whitespace() {
        _body = body.replace(word, &data_type_map(word, DatabaseType::MySql).as_str());
    }

    _body
}
