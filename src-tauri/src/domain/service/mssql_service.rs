use crate::domain::create_info::create_table_info::{CreateSequenceInfo, CreateTableInfo, CreateViewInfo};
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
use crate::domain::repository::mssql_repository::MsSqlRepository;
use crate::domain::service::database_service::DatabaseService;
use crate::DatabaseConnection;
use dashmap::DashMap;
use futures::Future;
use rbs::value::map::ValueMap;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

pub struct MsSqlService {
    db_repo: Arc<MsSqlRepository>,
}

impl MsSqlService {
    pub fn new(connection_info: DatabaseConnection) -> MsSqlService {
        let repo = Arc::new(MsSqlRepository::new(connection_info));
        MsSqlService { db_repo: repo }
    }
}

impl DatabaseService for MsSqlService {
    fn get_metadatas(
        &self,
        db_name: &str,
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

    fn migrate_to<'a>(
        &self,
        migration_config: MigrationConfig, /*db_name:&'a str,target_db:DatabaseType,limit:Option<DMLLimit>,exclude_columns:Option<Vec<String>>,obfuscations:Option<Obfuscation> ,table_name:&'a str,columns:&'a Vec<String>*/
    ) -> Pin<Box<dyn Future<Output = Result<(), rbdc::error::Error>> + Send + 'a>> {
        let repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let db = repo.get_database_(&migration_config.db_name).await.unwrap();
            //let rows = repo.get_row_value(table_name, db_name, columns);
            let _ = db
                .schemas
                .as_ref()
                .unwrap()
                .get("public")
                .unwrap()
                .generate_ddls(migration_config.db_type)
                .await;
            let tables = db
                .schemas
                .as_ref()
                .unwrap()
                .get("public")
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
        Box::pin(async move {
            //let query_result = db_repo.rb_map.get("products").unwrap().query(sql,params.to_vec()).await;

            if let Some(rb) = db_repo.rb_map.get("products") {
                //rb.query(sql, params.to_vec()).await.map_err(|e| rbdc::Error::from(e))
                let help_me: Vec<ValueMap> =
                    rb.query_decode(&query_info.sql, vec![]).await.unwrap();
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
            let new_table = db_repo.create_table(&table_info).await?;
            Ok(new_table)
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
            //todo!()
        })
    }


fn create_index<'a>(
        &self,
        index:crate::domain::metadata::index::Index,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Index, rbdc::error::Error>> + Send + 'a>> {
    let db_repo = Arc::clone(&self.db_repo);
    Box::pin(async move {
        let index_info = crate::domain::create_info::create_table_info::CreateIndexInfo{
            index_name: index.name.clone(),
            table_name: index.table_name.clone().unwrap(),
            columns: index.column_name.clone().unwrap(),
            schema_name: index.schema_name.clone().unwrap(),
        };
        let result_ = db_repo.create_index(&index_info, &db_name).await;
        result_
        //Ok(())
        //todo!()
    })
}

fn create_role<'a>(
        &self,
        role:crate::domain::metadata::role::Role,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Role, rbdc::error::Error>> + Send + 'a>> {
    /*let db_repo = Arc::clone(&self.db_repo);
    Box::pin(async move {
        let result_ = db_repo.create_role(role, &db_name).await.await;
        result_
    })*/
    todo!()
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
           //todo!()
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
        let result_ = db_repo.create_sequence("table_name", &db_name, &seq.schema_name,&seq).await;
        result_
        //todo!()
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
        let result_ = db_repo.create_trigger(name, when, type_, table_name, function_name, &db_name,"schema_name").await;
        result_
       //todo!()
    })
}
fn create_view<'a>(
        &self,
        view:crate::domain::metadata::view::View,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<View, rbdc::error::Error>> + Send + 'a>> {
    let db_repo = Arc::clone(&self.db_repo);
    Box::pin(async move {
        let view_info = CreateViewInfo{
            view_name: view.name,
            columns: todo!(),
            table_name: todo!(),
            stmt: view.definition,
            schema_name: view.schema_name
        };
        let result_ = db_repo.create_view(&view_info, &db_name).await;
        result_
        //Ok(())
       // todo!()
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
                .add_column(&table_name, column_info, &database_name)
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
                let new_column = db_repo
                    .alter_table_column(
                        &table_name,
                        new_cols,
                        old_cols,
                        &db_name,
                    )
                    .await
                    .await;
            //}

   //         let new_column = db_repo.alter_table_column(&table_name,new_cols.get(0).unwrap().clone(),old_cols.get(0).unwrap().clone(),"huwa").await.await?;
                new_column
            //Ok(())
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
            let result_ = db_repo.edit_constraint(table_name.as_str(), new_constraint, old_constraint, &db_name).await.await;
            //todo!()
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
                //todo!()
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
           // todo!()
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
           //todo!()
            result_
        })
    }

    fn create_database<'a>(&self, _db_name: &'a str, file_path: &'a str) -> Pin<Box<dyn Future<Output = Result<(), rbdc::error::Error>> + Send + 'a>> {
        todo!()
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
        todo!()
    }
    
    fn delete_table<'a>(
        &self,
        table_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
        todo!()
    }
    
    fn delete_sequence<'a>(
        &self,
        sequence_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
        todo!()
    }
    
    fn delete_function<'a>(
        &self,
        function_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
        todo!()
    }
    
    fn create_constraint<'a>(
        &self,
        constraint: crate::domain::metadata::constraint::Constraint,
        table_name: &'a str,
        schema_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Constraint, rbdc::error::Error>> + Send + 'a>> {
        /*let db_repo = Arc::clone(&self.db_repo);
        Box::pin(async move {
            let result_ = db_repo.create_c
        })*/
        todo!()
    }
}

mod tests{
    
}