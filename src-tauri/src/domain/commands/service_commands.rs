use crate::domain::datb::database_type;
use crate::domain::datb::migraton_config::MigrationConfig;
use crate::domain::datb::query_info::{QueryInfo, QueryResult};
use crate::domain::metadata::constraint::Constraint;
use crate::domain::metadata::database::Schema;
use crate::domain::metadata::database_metadata::DatabaseMetadata;
use crate::domain::metadata::function::Function;
use crate::domain::metadata::index::Index;
use crate::domain::metadata::role::Role;
use crate::domain::metadata::sequence::Sequence;
use crate::domain::metadata::table::Table;
use crate::domain::metadata::trigger::Trigger;
use crate::domain::metadata::view::View;
use crate::{AppData, DatabaseConnection};
use std::collections::HashMap;
use std::result;
//use fast_log::plugin::console::{self, ConsoleAppender};
use rbdc::db::Row;
use rbs::value::map::ValueMap;
use serde::{Deserialize, Serialize};

use crate::domain::create_info::create_table_info::{CreateSchemaInfo, CreateSequenceInfo, CreateTableInfo, CreateTriggerInfo, CreateViewInfo};
use crate::domain::metadata::column::Column;
use crate::domain::service::database_service::DatabaseService;
use crate::domain::service::mssql_service::MsSqlService;
use crate::domain::service::mysql_service::MySqlService;
use crate::domain::service::oracle_service::OracleService;
use crate::domain::service::postgres_service::PostgresService;
use crate::domain::service::sqlite_service::SqLiteService;
//use fast_log::{init, plugin::file, Config};
use std::sync::Arc;
//use tokio::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn connect<'a>(
    db_connection: DatabaseConnection,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<String, ()> {
    let mut _state = state.lock().await;
    let connection_clone = db_connection.clone();
    if _state.service.contains_key(&db_connection.driver_type) {
        Ok("You have already connected to this database".to_string())
    } else {
        log::info!("Database connection: {:?}\n", db_connection);
        let serv: Arc<dyn DatabaseService> = match db_connection.driver_type.as_str() {
            "postgresql" => Arc::new(PostgresService::new(db_connection)),
            "mysql" => Arc::new(MySqlService::new(db_connection)),
            "mssql" => Arc::new(MsSqlService::new(db_connection)),
            "oracle" => Arc::new(OracleService::new(db_connection)),
            "sqlite" => Arc::new(SqLiteService::new(db_connection)),
            _ => {
                return Err(());
            }
        };

        let _ = serv.init_database().await;
        _state.service.insert(connection_clone.driver_type, serv.clone());
        //println!("Query test: {:?}",_state.service.get("postgres").unwrap()._query(QueryInfo{sql:"select * from product".to_string(),db_type:crate::datb::database_type::DatabaseType::Postgres,db_name:"products".to_string()}).await.unwrap());
        Ok("ok".to_string())
    }
}

#[tauri::command]
pub async fn create_database(database_info: String,
    db_type: String,
    file: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<(), rbdc::Error> {
    log::info!("Database info: {:?}\nFile: {:?}\nDatabase type: {:?}\n", database_info, file, db_type);
    let mut _state = state.lock().await;
    let service = _state.service.get(&db_type).expect("Service not found");
    let result = service.create_database(&database_info,&file).await;
    log::info!("Query result: {:?}", result);
    result
    //Ok(())
}

#[tauri::command]
pub async fn migrate_to(
    migration_config: MigrationConfig,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<(), rbdc::Error> {
    log::info!("Migration Config Info: {:?}", migration_config);
    let mut _state = state.lock().await;
    let service = _state.service.get(&migration_config.db_type.to_string()).expect("Service not found");
    let result_ = service.migrate_to(migration_config).await;
    //Ok("Migration success".to_string())
    result_
}

#[tauri::command]
pub async fn query(
    query_info: QueryInfo,
    state: State<'_, tokio::sync::Mutex<AppData>>,
    //database_type: String
) -> Result<QueryResult, ()> {
    println!("query: {:?}", query_info);
    let mut _state = state.lock().await;
    let service = _state.service.get(query_info.db_type.as_str()).unwrap();
    let result: Vec<ValueMap> = service._query(query_info).await.unwrap();
    Ok(QueryResult { rows: result })
}

#[tauri::command]
pub async fn get_metadatas(
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<HashMap<String, DatabaseMetadata>, ()> {
    let mut _state = state.lock().await;
    log::info!("Database type: {}", database_type);
    let service = _state.service.get(database_type.as_str()).unwrap();
    let result:HashMap<String, DatabaseMetadata> = service
        .get_metadatas(&database_type.as_str())
        .await
        .unwrap()
        .into_iter()
        .collect();
    Ok(result)
}

/// Asynchronously creates a new table in the specified database using the given table information.
/// 
/// # Arguments
/// 
/// * `table_info` - An instance of `CreateTableInfo` containing the details of the table to be created.
/// * `database_type` - A string specifying the type of database (e.g., "postgresql", "mysql").
/// * `state` - A mutable reference to the application's shared state, wrapped in a `State` and `tokio::sync::Mutex`.
/// 
/// # Returns
/// 
/// * `Result<(), rbdc::Error>` - Returns `Ok(())` if the table creation is successful or an `rbdc::Error` if an error occurs.

#[tauri::command] 
pub async fn create_table(
    table_info: CreateTableInfo,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<Table, rbdc::Error> {
    println!("table_info: {:?}", table_info);
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    let db_name = table_info.db_name.clone();
    let result_ = service.create_table(table_info, &db_name).await;
    result_
    //Ok(())
}

#[tauri::command]
pub async fn create_sequence(
    create_seq_info: CreateSequenceInfo,
    database_type: String,
    table_name: String,
    database_name:String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<Sequence, rbdc::Error> {
    //print!("create_seq_info: {:?}", create_seq_info);
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    //Ok("".to_string());
    let result_ = service.create_sequence(create_seq_info,&database_name).await;
    // Ok("".to_string())
    result_
}

//TODO
#[tauri::command]
pub async fn create_trigger(
    trigger_info: CreateTriggerInfo,
    database_name: String,
    database_type:String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<Trigger, rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    //Ok("".to_string());
  //  let result_ = service.create_trigger(name.as_str(), when.as_str(), type_.as_str(), table_name.as_str(), function_name.as_str(), database_name.as_str()).await;
    let result_ = service.create_trigger(&trigger_info.name,&trigger_info.when,&trigger_info.type_,&trigger_info.table_name,&trigger_info.function_name,&database_name).await;
    // Ok("".to_string())
    Ok(result_.unwrap())
}



#[tauri::command]
pub async fn create_function(
    function_info: Function,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<Function, rbdc::Error> {
    log::info!("function_info: {:?}", function_info);
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    let result_ = service.create_function(function_info).await;
    //Ok(result)
    //Ok("".to_string())
    result_
}

//TODO
#[tauri::command]
pub async fn create_schema(
   //schema_info: CreateSchemaInfo,
    schema_name: String,
    db_name: String,    
    user_name: Option<String>,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>
)-> Result<Schema, rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    let result_ = service.create_schema(schema_name.as_str(),db_name.as_str(),user_name).await;
    Ok(result_.unwrap())
   // Ok("dockey".to_string())
}




//TODO
#[tauri::command]
pub async fn create_view(
    view : View,
   // view_info: View,
    db_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<View, rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    let result_ = service.create_view(view,&db_name).await;
    //Ok("dockey".to_string())
    result_
}

#[tauri::command]
pub async fn create_index(
    index: Index,
    db_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<Index, rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    let result_ = service.create_index(index,&db_name).await;
    //Ok("dockey".to_string())
    Ok(result_.unwrap())
}

#[tauri::command]
pub async fn create_role(
    role: Role,
    db_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<Role, rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    let result_ = service.create_role(role,&db_name).await;
    if result_.is_err() {
        return Err(result_.unwrap_err())
    }
    log::info!("Result is ok: {:?}", result_.as_ref().unwrap());
    //Ok(())
    Ok(result_.unwrap())
   // Ok("dockey".to_string())
}

#[tauri::command]
pub async fn create_constraint(
    constraint: Constraint,
    database_type: String,
    //schema_name: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
)-> Result<Constraint, rbdc::Error>{
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    let constraint_clone = constraint.clone();
    let result_ = service.create_constraint(constraint,&constraint_clone.table_name,&constraint_clone.schema_name.unwrap()).await;
    if result_.is_err() {   
        return Err(result_.unwrap_err())
    }
    log::info!("Result is ok: {:?}", result_.as_ref().unwrap());
    result_
}



#[tauri::command]
pub async fn add_column(
    table_name: String,
    column_info: Column,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<Column, ()> {
    let mut _state = state.lock().await;
 //   println!("column_info: {:?}", column_info);
    let service = _state.service.get(database_type.as_str()).unwrap();
    let result = service.add_column(table_name, column_info,database_type/*KIJAVÍTANI */).await.unwrap();
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditTableColumn {
    new_cols: Column,
    old_cols: Column,
}

#[tauri::command]
pub async fn edit_table_column(
    new_col: Column,
    old_col: Column,
    table_name: String,
    db_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<Column, rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    log::info!(
        "Edit table column info: new: {:?}\nold: {:?}",
        new_col,
        old_col
    );
    let result_ = service
        .edit_table_column(table_name, db_name, new_col, old_col)
        .await;
    result_
    //Ok("dockey".to_string())
}

#[tauri::command]
pub async fn edit_sequence(
    old_sequence: Sequence,
    new_sequence: Sequence,
    db_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<Sequence, rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    log::info!(
        "Edit sequence info: new: {:?}\nold: {:?}",
        new_sequence,
        old_sequence
    );
    let result_ = service.edit_sequence(db_name, old_sequence, new_sequence).await;
    result_
    //Ok("dockey".to_string())
}

#[tauri::command]
pub async fn edit_constraint(
    old_constraint: Constraint,
    new_constraint: Constraint,
    db_name: String,
    table_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>
)->Result<Constraint,rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    let result_ = service.edit_constraint(db_name, table_name, old_constraint, new_constraint).await;
    result_
    //Ok("dockey".to_string())
}

#[tauri::command]
pub async fn edit_index(
    old_index: Index,
    new_index: Index,
    db_name: String,
    table_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>
)->Result<Index,rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    let result_ = service.edit_index(db_name, table_name, old_index, new_index).await;
    result_
    //Ok("dockey".to_string())
}

#[tauri::command]
pub async fn edit_function(
    old_function: Function,
    new_function: Function,
    db_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<Function, rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    log::info!(
        "Edit function info: new: {:?}\nold: {:?}",
        new_function,
        old_function
    );
    let result_ = service.edit_function(db_name, old_function, new_function).await;
    result_
    //Ok("dockey".to_string())
}

#[tauri::command]
pub async fn edit_view(
    old_view: View,
    new_view: View,
    db_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<Function, rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    log::info!(
        "Edit view info: new: {:?}\nold: {:?}",
        new_view,
        old_view
    );
   //66 let result_ = service.edit_view(db_name,old_view,new_view).await;
   // result_
    todo!()
    //Ok("dockey".to_string())
}


#[tauri::command]
pub async fn delete_table(
    table_name: String,
    db_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<(), rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
     let result_ = service.delete_table(table_name,db_name).await;
    // result_
    Ok(())
}

#[tauri::command]
pub async fn delete_table_column(
    column_name: String,
    table_name: String,
    db_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<(), rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    log::info!("Delete column data: {:?}\n{:?}\n{:?}", column_name,table_name,db_name);
    let result_delete = service.delete_table_column(column_name,table_name,db_name).await;
    result_delete
}
/* 
#[tauri::command]
pub async fn delete_sequence(
    sequence_name: String,
    db_name: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<(), rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get("postgres").unwrap();
    //let result = service.delete_sequence(sequence_name,db_name).await.unwrap();
    Ok("dockey".to_string())
}

#[tauri::command]
pub async fn delete_function(
    function_name: String,
    db_name: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<(), rbdc::Error> {
    let mut _state = state.lock().await;
    let service = _state.service.get("postgres").unwrap();
    //let result = service.delete_function(function_name,db_name).await.unwrap();
    Ok("dockey".to_string())
}*/

//        await invoke("base_delete", {dbName:db_name,deleteToName:delete_to_name,objectName:object_name})

#[tauri::command]
pub async fn base_delete(
    delete_to_name: String,
    object_name: String,
    db_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<(), rbdc::Error> {
    log::info!(
        "delete_to_name: {:?}\nobject_name: {:?}\ndb_name: {:?} driver_type: {:?}",
        delete_to_name,
        object_name,
        db_name,
        database_type
    );
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    let result_del = service
        .base_delete(delete_to_name, object_name, db_name)
        .await;
    log::info!("Result from the service: {:?}\n", result_del);
    result_del
}

#[tauri::command]
pub async fn get_table(
    table_name: String,
    database_type: String,
    state: State<'_, tokio::sync::Mutex<AppData>>,
) -> Result<Table, ()> {
    let mut _state = state.lock().await;
    let service = _state.service.get(database_type.as_str()).unwrap();
    let result = service.get_table(table_name, database_type/*KIJAVÍTANI */).await.unwrap();
    Ok(result)
}
