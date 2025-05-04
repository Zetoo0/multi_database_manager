use crate::domain::create_info::create_table_info::{CreateIndexInfo, CreateSequenceInfo, CreateTableInfo, CreateViewInfo};
use crate::domain::datb::dml_generate::GenerateDML;
use crate::domain::datb::migraton_config::MigrationConfig;
use crate::domain::datb::query_info::QueryInfo;
use crate::domain::metadata::column::Column;
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
use crate::domain::repository::postgres_repository::PostgresRepository;
use crate::domain::service::database_service::DatabaseService;
use crate::DatabaseConnection;
use dashmap::DashMap;
use futures::{Future, TryFutureExt};
use rbs::value::map::ValueMap;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

pub struct PostgresService {
    db_repo: Arc<PostgresRepository>,
}

impl PostgresService {
    pub fn new(connection_info: DatabaseConnection) -> PostgresService {
        let repo = Arc::new(PostgresRepository::new(connection_info));
        PostgresService { db_repo: repo }
    }


}

impl DatabaseService for PostgresService {
    fn get_metadatas(
        &self,
        _db_name: &str,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<DashMap<String, DatabaseMetadata>, rbdc::error::Error>,
                > + Send,
        >,
    > {
        let repo = Arc::clone(&self.db_repo);
        Box::pin(async move { Ok(repo.databases.clone()) })
    }

    fn create_database<'a>(&self, _db_name: &'a str, file_path: &'a str) -> Pin<Box<dyn Future<Output = Result<(), rbdc::error::Error>> + Send + 'a>> {
        let repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let result = repo.create_database(_db_name, file_path).await; 
            result
            }
        )
    }

    fn migrate_to<'a>(
        &self,
        migration_config: MigrationConfig, /*db_name:&'a str,target_db:DatabaseType,limit:Option<DMLLimit>,exclude_columns:Option<Vec<String>>,obfuscations:Option<Obfuscation> ,table_name:&'a str,columns:&'a Vec<String>*/
    ) -> Pin<Box<dyn Future<Output = Result<(), rbdc::error::Error>> + Send + 'a>> {
        let repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let db = repo.get_database_(&migration_config.db_name).unwrap();
            //let rows = repo.get_row_value(table_name, db_name, columns).await.unwrap();
            let _ = db
                .schemas
                .as_ref()
                .unwrap()
                .get(&migration_config.schema_name)
                .unwrap()
                .generate_ddls(migration_config.db_type)
                .await;
            let tables = db
                .schemas
                .as_ref()
                .unwrap()
                .get(&migration_config.schema_name)
                .as_ref()
                .unwrap()
                .tables
                .as_ref()
                .unwrap();
            let exclude_columns_clone = migration_config.exclude_columns.clone().unwrap();
            for (table_name, table) in tables.iter() {
                let cols: HashMap<String, Column> = table
                    .columns
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter(|key| !exclude_columns_clone.contains(&key.0))
                    .map(|(k, v)| (k.to_string(), v.clone()))
                    .collect();
                let rows = repo
                    .get_row_value(
                        &table_name,
                        &migration_config.db_name,
                        &cols.keys().map(|v| v.to_string()).collect::<Vec<_>>(),
                    )
                    .await
                    .unwrap();
                let _ = table
                    .to_insert(
                        migration_config.db_type,
                        rows,
                        cols,
                        &migration_config.db_name,
                        migration_config.limit.clone(),
                        migration_config.exclude_columns.clone(),
                        migration_config.obfuscations.clone(),
                    )
                    .await;
            }
            Ok(())
        })
    }

    fn _query<'a>(
        &self,
        query_info: QueryInfo,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Vec<ValueMap>, rbdc::error::Error>> + Send + 'a,
        >,
    > {
        let db_repo = Arc::clone(&self.db_repo);
        println!("Query: {:?}", &query_info.db_name);
        Box::pin(async move {
            //let query_result = db_repo.rb_map.get("products").unwrap().query(sql,params.to_vec()).await;

            if let Some(rb) = db_repo.rb_map.get(&query_info.db_name) {
                //rb.query(sql, params.to_vec()).await.map_err(|e| rbdc::Error::from(e))
                println!("Query: {}", query_info.sql);
                let help_me: Vec<ValueMap> =
                    rb.query_decode(&query_info.sql, vec![]).await.unwrap();
                println!("query result {:?}", help_me);
                Ok(help_me)
            } else {
                Err(rbdc::Error::from("Database not found"))
            }
        })
    }

    fn init_database<'a>(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), rbdc::error::Error>> + Send + 'a>,
    > {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let _ = db_repo.init_database().await;
            Ok(())
        })
    }

    fn create_table<'a>(
        &self,
        table_info: CreateTableInfo,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Table, rbdc::error::Error>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let new_table = db_repo.create_table(&db_name, &table_info).await;
            new_table
        })
    }

    fn create_function<'a>(
        &self,
        function: Function,
    ) -> Pin<Box<dyn Future<Output = Result<Function, rbdc::error::Error>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let db_name = function.db_name.clone();
            let result_ = db_repo.create_funtion(&db_name, function).await.await;
            result_
           // Ok(())
        })
    }


fn create_index<'a>(
        &self,
        index:crate::domain::metadata::index::Index,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Index, rbdc::error::Error>> + Send + 'a>> {
    let db_repo = Arc::clone(&self.db_repo);
    Box::pin(async move {
        let info = CreateIndexInfo{
            index_name: index.name.clone(),
            table_name: index.table_name.clone().unwrap(),
            columns: index.column_name.clone().unwrap(),
            schema_name: todo!(),
        };
        let result_ = db_repo.create_index(&info, &db_name).await;
        result_
      //  Ok(())
    })
}

fn create_role<'a>(
        &self,
        role:crate::domain::metadata::role::Role,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Role, rbdc::error::Error>> + Send + 'a>> {
    let db_repo = Arc::clone(&self.db_repo);
    Box::pin(async move {
        let result_ = db_repo.create_role(role, &db_name,"public").await.await;
        log::info!("Service create role result: {:?}", result_);
        result_
    })
}

fn create_schema<'a>(
        &self,
        name:&'a str,
        db_name: &'a str,
        user_name:Option<String>
    ) -> Pin<Box<dyn Future<Output = Result<Schema, rbdc::error::Error>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let result_ = db_repo.create_schema(name, &db_name, user_name.as_deref()).await;
            result_
        })
}

fn create_sequence<'a>(
        &self,
        seq: CreateSequenceInfo,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Sequence, rbdc::error::Error>> + Send + 'a>> {
    let db_repo = Arc::clone(&self.db_repo);
    Box::pin(async move {
        let result_ = db_repo.create_sequence("valami",&db_name,"public", &seq).await;
        result_
    })
}

fn create_trigger<'a>(
        &self,
        name:&'a str,
        when:&'a str,
        type_:&'a str,
        table_name:&'a str,
        function_name:&'a str,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Trigger, rbdc::error::Error>> + Send + 'a>> {
    let db_repo = Arc::clone(&self.db_repo);
    Box::pin(async move {
        let result_ = db_repo.create_trigger(name, when, type_, table_name, function_name, &db_name,"public").await;
        result_
       // Ok(())
    })
}
fn create_view<'a>(
        &self,
        view:crate::domain::metadata::view::View,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<View, rbdc::error::Error>> + Send + 'a>> {
    let db_repo = Arc::clone(&self.db_repo);
    Box::pin(async move {
        let info: CreateViewInfo = CreateViewInfo{
            view_name: view.name.clone(),
            stmt: view.definition.clone(),
            columns: todo!(),
            table_name: todo!(),
            schema_name: view.schema_name.clone(),
        };
        let result_ = db_repo.create_view(&info, &db_name,&info.schema_name).await;
        result_
      //  Ok(())
    })
}

    fn add_column<'a>(
        &self,
        table_name: String,
        column_info: Column,
        database_name:String
    ) -> Pin<Box<dyn Future<Output = Result<Column, ()>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let new_column = db_repo
                .add_column(&table_name, column_info, &database_name,"public")
                .await?;
            Ok(new_column)
        })
    }

    fn edit_table_column<'a>(
        &self,
        table_name: String,
        db_name: String,
        new_cols: Column,
        old_cols: Column,
    ) -> Pin<Box<dyn Future<Output = Result<Column, rbdc::error::Error>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            //for i in 0..new_cols.len() {
                let clone = old_cols.clone();
                let new_column = db_repo
                    .alter_table_column(
                        &table_name,
                        new_cols,
                        old_cols,
                        &db_name,
                        clone.schema_name.unwrap().as_str()
                    )
                    .await
                    .await;
            //}

   //         let new_column = db_repo.alter_table_column(&table_name,new_cols.get(0).unwrap().clone(),old_cols.get(0).unwrap().clone(),"huwa").await.await?;

            new_column
        })
    }

    fn edit_constraint<'a>(
            &self,
            db_name: String,
            table_name:String,
            old_constraint: crate::domain::metadata::constraint::Constraint,
            new_constraint: crate::domain::metadata::constraint::Constraint,
        ) -> Pin<Box<dyn Future<Output = Result<Constraint, rbdc::error::Error>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move{
            let result_ = db_repo.edit_constraint(table_name.as_str(), old_constraint, new_constraint, &db_name).await.await;
            result_
        })
    }

    fn edit_index<'a>(
            &self,
            db_name: String,
            table_name: String,    
            old_index: crate::domain::metadata::index::Index,
            new_index: crate::domain::metadata::index::Index,
        ) -> Pin<Box<dyn Future<Output = Result<Index, rbdc::error::Error>> + Send + 'a>> {
            let db_repo = Arc::clone(&self.db_repo);
            Box::pin(async move{
                let result_ = db_repo.edit_index(table_name.as_str(), new_index, old_index, &db_name).await.await;
                result_
            })
    }

    fn edit_sequence<'a>(
        &self,
        db_name: String,
        old_sequence: Sequence,
        new_sequence: Sequence,
    ) -> Pin<Box<dyn Future<Output = Result<Sequence, rbdc::error::Error>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let result_ = db_repo
                .edit_sequence(old_sequence, new_sequence, &db_name)
                .await.await;
            result_
        })
    }

    fn edit_function<'a>(
        &self,
        db_name: String,
        old_function: Function,
        new_function: Function,
    ) -> Pin<Box<dyn Future<Output = Result<Function, rbdc::error::Error>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let result_ = db_repo
                .edit_function(old_function, new_function, &db_name)
                .await.await;
            result_
        })
    }

    fn delete_table_column<'a>(
        &self,
        column_name: String,
        table_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), rbdc::Error>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let result_del = db_repo
                .delete_table_column(&column_name, &table_name, &db_name)
                .await.await;
            result_del
        })
    }

    fn delete_table<'a>(
        &self,
        table_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let _ = db_repo.delete_table(&table_name, &db_name).await;
            Ok(())
        })
    }

    fn delete_function<'a>(
        &self,
        function_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let _ = db_repo.delete_function(&function_name, &db_name).await;
            Ok(())
        })
    }

    fn delete_sequence<'a>(
        &self,
        sequence_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let _ = db_repo.delete_sequence(&sequence_name, &db_name).await;
            Ok(())
        })
    }

    fn base_delete<'a>(
        &self,
        delete_to_name: String,
        object_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), rbdc::Error>> + Send + 'a>> {
        log::info!("Base delete service");
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            log::info!("Base delete service2");
            let result = db_repo
                .base_delete(&delete_to_name, &object_name, &db_name)
                .await
                .await;
            log::info!("Result from the repo: {:?}\n", result);
            result
        })
    }

    fn get_table<'a>(
        &self,
        table_name: String,
        database_name: String
    ) -> Pin<Box<dyn Future<Output = Result<Table, ()>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let table = db_repo.get_table(&table_name, &database_name).await.await?;
            Ok(table)
        })
    }
    
    fn create_constraint<'a>(
        &self,
        constraint: crate::domain::metadata::constraint::Constraint,
        table_name: &'a str,
        schema_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Constraint, rbdc::error::Error>> + Send + 'a>> {
        let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let db_name = constraint.db_name.clone();
            let result_ = db_repo.create_constraint(constraint,&db_name , &table_name, &schema_name).await.await;
            result_
        })
    }
}

/*
mod tests{
    use std::result;

    use crate::domain::datb::dml_generate::DMLLimit;
    use crate::domain::metadata::database::Schema;
    use crate::domain::metadata::sequence::Sequence;
    use crate::domain::metadata::view::View;

    use super::*;
    use mockall::*;
    use mockall::predicate::*;
    use rbatis::dark_std::sync::vec;
    use tokio::test;
    use rbatis::rbatis::RBatis;
    use crate::domain::service::postgres_service::PostgresService;

    pub async fn setup_connection() -> PostgresService{
        let connection_info = DatabaseConnection {
            port: "5432".to_string(), 
            server: "127.0.0.1".to_string(), 
            username: "mzeteny".to_string(),
            password: "".to_string(),
            driver_type: "postgresql".to_string() };
        let repo = Arc::new(PostgresRepository::new(connection_info));
        PostgresService { db_repo: repo }
    }

    #[tokio::test]
    pub async fn test_databases_metadatas_get_is_ok(){
        let service = setup_connection().await;
        let _ = service.init_database().await;
        let _result = service.get_metadatas("test_db").await.unwrap();
        assert!(_result.is_empty());
    }
    #[tokio::test]
    pub async fn test_database_migration_is_ok(){
        let service = setup_connection().await;
        let _ = service.init_database();
        let migration_config = MigrationConfig{ db_name: "test_db".to_string(),
            db_type: crate::domain::datb::database_type::DatabaseType::Postgres,
            limit: None, 
            exclude_columns: None,
            obfuscations: None 
            };
        let _result = service.migrate_to(migration_config).await;
        assert!(_result.is_err());       
    }
    
    #[tokio::test]
    pub async fn test_query_is_ok(){
        let service = setup_connection().await;
        let _ = service.init_database();
        let query_info = QueryInfo { sql: "SELECT * FROM test_table".to_string(),
            db_type: "postgresql".to_string(),
            db_name: "test_db".to_string(), 
        };
        let result = service._query(query_info).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    pub async fn test_create_database_is_ok(){
        let service = setup_connection().await;
        let _ = service.init_database();
        let result = service.create_database("test_database", "").await;
        assert(result.is_err());
    }

    #[tokio::test]
    pub async fn test_create_table_is_ok(){
        let service = setup_connection().await;
        let _ = service.init_database();
        let table_info = CreateTableInfo { table_name: "test_table2".to_string(),
            columns: vec![Column{ name: "testcol".to_string(),
                data_type: Some("VARCHAR".to_string()),
                is_nullable: Some(true),
                default_value: Some("".to_string()), 
                is_primary_key: Some(false), 
                maximum_length: Some(120), 
                schema_name: Some("public".to_string()), 
                table_name: "test_table2".to_string(), 
                db_name: "test_db".to_string(), 
                type_: "column".to_string() }], 
            db_name: "test_db".to_string(),
            schema_name: "public".to_string() 
        };
        let result = service.create_table(table_info, "test_db").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    pub async fn test_create_sequence_is_ok(){
        let service = setup_connection().await;
        let _ = service.init_database();
        let seq = CreateSequenceInfo { 
            sequence_name: "test_seq".to_string(),
            start_val: "0".to_string(),
            minimum_val: "0".to_string(), 
            maximum_val: "100".to_string(), 
            increment: "1".to_string(), 
            cycle: false 
        };
        let result = service.create_sequence(seq, "test_db").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    pub async fn test_create_role_is_ok(){
        let service = setup_connection().await;
        let _ = service.init_database();
        let role = Role{ 
            name: "test_role".to_string(), 
            is_super: Some(false), 
            is_insherit: Some(false), 
            is_create_role: Some(false), 
            is_create_db: Some(false), 
            can_login: Some(true), 
            is_replication: Some(false), 
            connection_limit: Some(10), 
            valid_until: None, 
            password: Some("test".to_string()), 
            db_name: "test_db".to_string(), 
            type_: "role".to_string() 
        };
        let result = service.create_role(role, "test_db").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    pub async fn test_create_view_is_ok(){
        let service = setup_connection().await;
        let _ = service.init_database();
        let view = View{ 
            name: "test_view".to_string(), 
            definition: "Some definition".to_string(),
            type_: "view".to_string() 
        }
        let result = service.create_view(view, "test_db").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    pub async fn test_create_function_is_ok(){
        let service = setup_connection().await;
        let _ = service.init_database();
        let function = Function{ 
            name: "test_function".to_string(), 
            definition: "Some definitoiin".to_string(), 
            parameters: None, 
            return_type: None, 
            type_: Some("function".to_string()), 
            schema_name: Some("public".to_string()), 
            db_name: "test_db".to_string()     
        };
        let result = service.create_function(function).await;
        assert!(result.is_err());
    }
}

 */