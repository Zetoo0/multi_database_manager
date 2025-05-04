export const dataTypeOptions: {[key: string]: string[]} = {
    "postgresql": [
      // Numeric Types
      'SMALLINT', 'INTEGER', 'BIGINT', 'DECIMAL', 'NUMERIC', 'REAL', 
      'DOUBLE PRECISION', 'SMALLSERIAL', 'SERIAL', 'BIGSERIAL', 'MONEY',
  
      // Character Types
      'CHAR', 'VARCHAR', 'TEXT', 'CHARACTER', 'CHARACTER VARYING',
  
      'BYTEA',
  
      // Date/Time Types
      'DATE', 'TIME', 'TIMESTAMP', 'TIMESTAMPTZ', 'TIMETZ', 'INTERVAL',,'TIMESTAMP WITH TIME ZONE','TIME WITHOUT TIME ZONE',
  
      'BOOLEAN',
  
      'ENUM',
  
      // Geometric Types
      'POINT', 'LINE', 'LSEG', 'BOX', 'PATH', 'POLYGON', 'CIRCLE',
  
      // Network Address Types
      'CIDR', 'INET', 'MACADDR',
  
      // Bit String Types
      'BIT', 'BIT VARYING',
  
      // Text Search Types
      'TSVECTOR', 'TSQUERY',
  
      'UUID',
  
      'XML',
  
      // JSON Types
      'JSON', 'JSONB'

    ] as string[],
    "mssql": [
      // Numeric Types
      "TINYINT", "SMALLINT", "INT", "BIGINT", "DECIMAL", "NUMERIC", "REAL", "FLOAT(n)", "MONEY", "SMALLMONEY",
  
      // Character Types
      "CHAR", "VARCHAR", "NCHAR", "NVARCHAR", "VARCHAR(MAX)", "NVARCHAR(MAX)",
  
      // Binary Types
      "BINARY", "VARBINARY", "IMAGE",
  
      // Date/Time Types
      "DATE", "TIME", "DATETIME", "DATETIME2", "SMALLDATETIME", "DATETIMEOFFSET",
  
      // Boolean Equivalent
      "BIT",
  
      // GUID (UUID Equivalent)
      "UNIQUEIDENTIFIER",
  
      // XML
      "XML"
    ],
    "mysql": [
      // Numeric Types
      "TINYINT", "SMALLINT", "MEDIUMINT", "INT", "BIGINT", "DECIMAL", "FLOAT", "DOUBLE", "BIT",
  
      // Character Types
      "CHAR", "VARCHAR", "TEXT", "TINYTEXT", "MEDIUMTEXT", "LONGTEXT",
  
      // Binary Types
      "BINARY", "VARBINARY", "TINYBLOB", "BLOB", "MEDIUMBLOB", "LONGBLOB",
  
      // Date/Time Types
      "DATE", "DATETIME", "TIMESTAMP", "TIME", "YEAR",
  
      // ENUM & SET
      "ENUM", "SET",
  
      // JSON
      "JSON"
    ],
    "sqlite": [
      // Storage Classes (SQLite uses these instead of strict data types)
      "NULL", "INTEGER", "REAL", "TEXT", "BLOB","NUMERIC",
  
      // Date/Time Types (Stored as TEXT, INTEGER, or REAL)
      "DATE", "DATETIME", "TIME",
  
      // JSON (Stored as TEXT but officially recognized)
      "JSON"
    ]
}; 
  