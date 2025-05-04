use crate::domain::datb::data_type_map::{self, data_type_map};
use crate::domain::datb::database_type::DatabaseType;
use crate::domain::datb::ddl_generate::GenerateDDL;
use regex;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ProcedureStatement {
    Declare(String, String),
    Assignment(String, String),
    Sql(String),
    If(String, Vec<ProcedureStatement>),
    Loop(Vec<ProcedureStatement>),
    Exception(Vec<(String, Vec<ProcedureStatement>)>),
    Raise(String, String),
}

impl ProcedureStatement {
    fn add_handler(&mut self, condition: String, body: Vec<ProcedureStatement>) {
        if let ProcedureStatement::Exception(ref mut handlers) = self {
            handlers.push((condition, body));
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Procedure {
    pub name: String,
    pub definition: String,
    pub parameters: Option<Vec<String>>,
    pub source_db: DatabaseType,
    pub db_name: String,
    pub schema_name:Option<String>,
    pub type_: String,
}

impl GenerateDDL for Procedure {
    fn to_create_function(&self, target_db_type: DatabaseType) -> Result<String, String> {
        let converted_procedure_def = self.definition.clone().to_string();
        match target_db_type {
            DatabaseType::MySql => {
                let mut converted_param = String::new();
                for param in self.parameters.as_ref().unwrap() {
                    let mut param_split = param.split(' ').collect::<Vec<&str>>();
                    if param_split.len() == 4 {
                        param_split.remove(0);
                    }
                    match param_split.len() {
                        2 => {
                            let data_name = param_split[0].replace("@", "");
                            let data_type = data_type_map(param_split[1], target_db_type);

                            converted_param
                                .push_str(format!("{} {}", data_name, data_type).as_str());
                        }
                        3 => {
                            let mut data_inout = param_split
                                .iter()
                                .find(|&&w| w == "IN" || w == "OUT")
                                .unwrap()
                                .to_uppercase();
                            let mut data_name = String::new();
                            if self.source_db == DatabaseType::Oracle {
                                data_name = param_split[0].to_string().replace("@", "");
                            } else {
                                data_name = param_split[1].to_string().replace("@", "");
                            }
                            let data_type = data_type_map(param_split[2], target_db_type);

                            data_inout = data_inout.replace("OUTPUT", "OUT").to_string();
                            converted_param.push_str(
                                format!(" {} {} {}, ", data_name, data_type, data_inout).as_str(),
                            );
                        }
                        _ => {
                            return Err("Invalid parameter".to_string());
                        }
                    }
                }
                let ddl = format!(
                    "CREATE OR REPLACE PROCEDURE {} ({}) AS ",
                    self.name, converted_param
                );
                /*let mut converted_body = self.definition.clone();
                println!("original body: {:?}", self.definition.clone());
                converted_body = postgres_to_mysql(&converted_body,target_db_type);
                let statements:Vec<ProcedureStatement> = parse_postgres_body(&converted_body);
                println!("statements: {:?}",statements);
                let converted_stmts:Vec<_> = statements.iter().map(convert_procedure_statement).collect();
                println!("converted ASD: {:?}",converted_stmts);
                println!("ASD: {:?}",converted_body);*/

                Ok(ddl)
            }
            DatabaseType::Postgres => {
                let mut converted_param = String::new();
                for param in self.parameters.as_ref().unwrap() {
                    let mut param_split = param.split(' ').collect::<Vec<&str>>();
                    if param_split.len() == 4 {
                        param_split.remove(0);
                    }
                    match param_split.len() {
                        2 => {
                            let data_name = param_split[0].replace("@", "");
                            let data_type = data_type_map(param_split[1], target_db_type);

                            converted_param
                                .push_str(format!("{} {}", data_name, data_type).as_str());
                        }
                        3 => {
                            let mut data_inout = param_split
                                .iter()
                                .find(|&&w| w == "IN" || w == "OUT")
                                .unwrap()
                                .to_uppercase();
                            let mut data_name: String = String::new();
                            if self.source_db == DatabaseType::Oracle {
                                data_name = param_split[0].to_string().replace("@", "");
                            } else {
                                data_name = param_split[1].to_string().replace("@", "");
                            }
                            let data_type = data_type_map(param_split[2], target_db_type);

                            data_inout = data_inout.replace("OUTPUT", "OUT").to_string();

                            converted_param.push_str(
                                format!(" {} {} {}, ", data_name, data_type, data_inout).as_str(),
                            );
                        }
                        _ => {
                            return Err("Invalid parameter".to_string());
                        }
                    }
                }
                let ddl = format!(
                    "CREATE OR REPLACE PROCEDURE {} ({}) AS {}",
                    self.name, converted_param, self.definition
                );
                Ok(ddl)
            }
            DatabaseType::MsSql => {
                let mut converted_param = String::new();
                for param in self.parameters.as_ref().unwrap() {
                    let mut param_split = param.split(' ').collect::<Vec<&str>>();
                    if param_split.len() == 4 {
                        param_split.remove(0);
                    }
                    match param_split.len() {
                        2 => {
                            let data_name = param_split[0];
                            let data_type = data_type_map(param_split[1], target_db_type);
                            converted_param
                                .push_str(format!("@{} {}", data_name, data_type).as_str());
                        }
                        3 => {
                            let mut data_inout = param_split
                                .iter()
                                .find(|&&w| w == "IN" || w == "OUT")
                                .unwrap()
                                .to_uppercase()
                                .to_string();
                            let mut data_name = String::new();
                            let data_type = data_type_map(param_split[2], target_db_type);

                            if self.source_db == DatabaseType::Oracle {
                                data_name = param_split[0].to_string();
                            } else {
                                data_name = param_split[1].to_string();
                            }

                            data_inout = data_inout.replace("IN", "").replace("OUT", "OUTPUT");

                            converted_param.push_str(
                                format!("@{} {} {}, ", data_name, data_type, data_inout).as_str(),
                            );
                        }
                        _ => {
                            return Err("Invalid parameter".to_string());
                        }
                    }
                }
                let ddl = format!(
                    "CREATE PROCEDURE {} {} AS {}",
                    self.name, converted_param, self.definition
                );
                Ok(ddl)
            }
            DatabaseType::Oracle => {
                let mut converted_param = String::new();
                for param in self.parameters.as_ref().unwrap() {
                    let mut param_split = param.split(' ').collect::<Vec<&str>>();
                    if param_split.len() == 4 {
                        param_split.remove(0);
                    }
                    match param_split.len() {
                        2 => {
                            let data_name = param_split[0].replace("@", "");
                            let data_type = data_type_map(param_split[1], target_db_type);

                            converted_param
                                .push_str(format!("{} {}", data_name, data_type).as_str());
                        }
                        3 => {
                            let mut data_inout = param_split
                                .iter()
                                .find(|&&w| w == "IN" || w == "OUT")
                                .unwrap()
                                .to_uppercase();
                            let data_name = param_split[1].replace("@", "");
                            let data_type = data_type_map(param_split[2], target_db_type);

                            data_inout = data_inout.replace("OUTPUT", "OUT").to_string();

                            converted_param.push_str(
                                format!("{} {} {}, ", data_name, data_inout, data_type).as_str(),
                            );
                        }
                        _ => {
                            return Err("Invalid parameter".to_string());
                        }
                    }
                }
                let ddl = format!(
                    "CREATE PROCEDURE {} {} AS {}",
                    self.name, converted_param, self.definition
                );
                Ok(ddl)
            }
            DatabaseType::Sqlite => Err("Sqlite does not support procedure".to_string()),
        }
    }
}

pub fn postgres_to_mysql(body: &str, target_db: DatabaseType) -> String {
    let mut converted_body = body.to_string();

    // Convert variable declaration from PostgreSQL to MySQL
    // PostgreSQL: DECLARE var_name INTEGER DEFAULT 0;
    // MySQL: DECLARE var_name INT DEFAULT 0;
    //converted_body = converted_body.replace("INTEGER", "INT");
    //map_variable_types_to_postgres(body, target_db);

    //Convert data types to MySQL
    converted_body = data_type_map::map_full_postgres_to_mysql(&converted_body);

    // 2. Convert assignment from PostgreSQL's := to MySQL's SET
    // PostgreSQL: var_name := value;
    // MySQL: SET var_name = value;
    converted_body = converted_body.replace(":=", "=");

    // Convert ELSIF to ELSEIF
    // PostgreSQL: ELSIF
    // MySQL: ELSEIF
    converted_body = converted_body.replace("ELSIF", "ELSEIF");
    println!("daconverted body: {:?}", converted_body);
    converted_body
}

pub fn map_variable_types_to_postgres(body: &str, target_db: DatabaseType) {
    // Matches patterns like: DECLARE var_name datatype [DEFAULT ...]
    let re = regex::Regex::new(r"(?s)DECLARE\s+((?:\s*\w+\s+\w+;\s*)+)\s*BEGIN").unwrap();

    // Check if the DECLARE block is found
    if let Some(caps) = re.captures(body) {
        let declare_block = caps.get(1).map_or("", |m| m.as_str());
        println!("DECLARE Block:\n{}", declare_block);

        let var_pattern = regex::Regex::new(r"(\w+)\s+(\w+);").unwrap();
        for var_caps in var_pattern.captures_iter(declare_block) {
            let var_name = &var_caps[1];
            let mut var_type = &var_caps[2];

            // Map PostgreSQL type to MySQL type
            let var_type_str = data_type_map(var_type, target_db);
            var_type = var_type_str.as_str();
            println!("Variable: {}, Type: {}", var_name, var_type);
            //returnString.push_str(format!("{} {}\n", var_name, var_type).as_str());
            println!("SET {} = {}", var_name, var_type);
        }
    }
}

pub fn parse_postgres_body(body: &str) -> Vec<ProcedureStatement> {
    let mut statements: Vec<ProcedureStatement> = Vec::new();

    // Regex patterns for parsing different constructs
    let declare_re = Regex::new(r"DECLARE\s+(?P<name>\w+)\s+(?P<type>\w+);").unwrap();
    let assignment_re = Regex::new(r"(?<var>\w+)\s*=\s*(?<value>.+);").unwrap();
    let if_re = Regex::new(r"IF\s+(?<condition>[\s\S]+?)THEN(?<body>[\s\S]+?)END IF;").unwrap();
    let loop_re = Regex::new(r"LOOP\s*(?<body>[\s\S]+?)END\s+LOOP;").unwrap();
    let sql_re = Regex::new(r"^\s*(SELECT|INSERT|UPDATE|DELETE).+;$").unwrap();
    let exception_re = Regex::new(r"EXCEPTION\s*(?P<handlers>[\s\S]+?)\s*(?:END;|;|$)").unwrap();
    let handler_re = Regex::new(r"WHEN\s+(?P<condition>.+?)\s+THEN\s*(?P<body>.+?);").unwrap();

    if_re.captures_iter(body).for_each(|c| {
        let condition = c.name("condition").unwrap().as_str().to_string();
        let body = c.name("body").unwrap().as_str();
        let inner_statements = parse_postgres_body(body); // Recursive call
        println!("condition: {}, body: {}", condition, body);
        statements.push(ProcedureStatement::If(condition, inner_statements));
    });
    loop_re.captures_iter(body).for_each(|c| {
        let body = c.name("body").unwrap().as_str();
        let inner_statements = parse_postgres_body(body); // Recursive call
        println!("body: {}", body);
        statements.push(ProcedureStatement::Loop(inner_statements));
    });
    assignment_re.captures_iter(body).for_each(|c| {
        let var = c.name("var").unwrap().as_str();
        let value = c.name("value").unwrap().as_str();
        statements.push(ProcedureStatement::Assignment(
            var.to_string(),
            value.to_string(),
        ));
    });
    println!("body: {}", body);

    statements
}

pub fn convert_procedure_statement(
    statement: &ProcedureStatement, /* , target_db: DatabaseType*/
) -> String {
    match statement {
        ProcedureStatement::Declare(name, typ) => format!("DECLARE {} {}", name, typ),
        ProcedureStatement::Assignment(var, value) => format!("SET {} = {}", var, value),
        ProcedureStatement::Sql(sql) => sql.clone(),
        ProcedureStatement::If(condition, body) => {
            let body_statements: Vec<String> =
                body.iter().map(convert_procedure_statement).collect();
            format!(
                "IF {} THEN {} END IF;",
                condition,
                body_statements.join(" ")
            )
        }
        ProcedureStatement::Loop(body) => {
            let body_statements: Vec<String> =
                body.iter().map(convert_procedure_statement).collect();
            format!("LOOP {} END LOOP;", body_statements.join(" "))
        }
        ProcedureStatement::Exception(handlers) => {
            let mut handler_statements = Vec::new();
            for (condition, body) in handlers {
                let body_statements: Vec<String> =
                    body.iter().map(convert_procedure_statement).collect();
                handler_statements.push(format!(
                    "WHEN {} THEN {}",
                    condition,
                    body_statements.join(" ")
                ));
            }
            format!(
                "DECLARE CONTINUE HANDLER FOR SQLEXCEPTION BEGIN {} END;",
                handler_statements.join(" ")
            )
        }
        ProcedureStatement::Raise(var, typ) => format!("RAISE {} {}", var, typ),
    }
}
