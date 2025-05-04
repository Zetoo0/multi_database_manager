use crate::domain::create_info::create_table_info::{CreateIndexInfo, CreateTableInfo, CreateViewInfo};
use crate::domain::metadata::column::Column;
use crate::domain::metadata::constraint::Constraint;
use crate::domain::metadata::database_metadata::DatabaseMetadata;
use crate::domain::metadata::function::Function;
use crate::domain::metadata::trigger::Trigger;
use crate::domain::metadata::{ table:: Table, trigger, view::View, index::Index};
use crate::domain::repository::database_repository::DatabaseRepository;
use dashmap::mapref::one::Ref;
use rbdc_sqlite::*;
use rbs::value::map::ValueMap;
use rbs::Value;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use crate::domain::metadata::database::Schema;
use dashmap::DashMap;


#[derive(Debug,Clone)]
pub struct SqLiteRepository{
    pub rb_map:DashMap<String,Arc<rbatis::RBatis>>,
    base_path:String,
    pub databases:DashMap<String,DatabaseMetadata>,
}

impl SqLiteRepository {
    pub fn new(path:&str)->Self{
        let rb_map = DashMap::new();
        let databases = DashMap::new();
        log::info!("Connecting to path: {}",path);
        let base_path = String::from(format!("sqlite:///{}",path));
        return SqLiteRepository{ rb_map, base_path, databases};
    }

        ///Add the database to the pool if not exists
        ///It's create a new rbatis, initialize it add add to the pool
    async fn connect(&self,name:&str,path:&str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.rb_map.contains_key(name){
            log::info!("new pool adding... database: {:?}",name);
            let rb = Arc::new(rbatis::RBatis::new());
            match rb.init(SqliteDriver {}, &self.base_path) {
                Ok(_) => log::info!("Connection to {} successful", name),
                Err(e) => {
                    log::error!("Failed to initialize rbatis for {}: {:?}", name, e);
                    return Err(Box::new(e)); // Return the error early
                }
            }
            self.rb_map.insert(name.to_string(),rb);
            let db_node = Schema{
                name : name.to_string(),
                tables : Some(HashMap::new()),
                functions : None,                       
                procedures : Some(HashMap::new()),
                views : Some(HashMap::new()),
                constraints : Some(HashMap::new()),
             //   foreign_data_wrappers : None,
                locks : None,
                types : None,
                triggers : Some(HashMap::new()),
                aggregates : None,
                materalized_views : None,
                catalogs : None,
                sequences : None,
                roles : None,
                type_:Some("schema".to_string()),
            };

            let db_metadata = DatabaseMetadata{
                name : name.to_string(),
                schemas : Some(HashMap::new()),
                foreign_data_wrappers : None,
                catalogs : None,
                type_:"database".to_string(),
            };
            self.databases.insert(name.to_string(), db_metadata);
            log::info!("Database node created for {}", name);
        }
        Ok(())
        }

        ///Get the database by its name
        pub async fn get_database_(&self,db_name:&str)->std::option::Option<dashmap::mapref::one::Ref<'_, std::string::String, DatabaseMetadata, >>{
            self.databases.get(db_name)
        }

        ///connect to rbatis if it isnt cached
        async fn rbatis_connect(&self,db_name:&str)->Result<Option<Ref<'_,String,Arc<rbatis::RBatis>>>,rbdc::Error>{
            let cached_rb = self.rb_map.get(db_name);
            if cached_rb.is_some(){
                log::info!("rb cached");
                return Ok(cached_rb);
            }
            log::info!("rb isnt cached");          
            if let Err(err) = self.connect(db_name, &self.base_path).await {
                log::error!("Connection failed for database {}: {:?}", db_name, err);
                return Err(rbdc::Error::from("Failed to connect to database"));
            }
            match self.rb_map.get(db_name) {
                Some(rb) => {
                    log::info!("Connection successfull, rb cached");
                    Ok(Some(rb))
                }
                None => {
                    log::error!("Connection failed to cache rbatis");
                    Err(rbdc::Error::from("Database not found after connection attempt"))
                }
            }
            //Ok(rb)
        }

        pub async fn init_database(&self) {
            let databases = self.get_databases().await.unwrap();
            log::info!("Databases: {:#?}",databases);
            let mut schemas:Vec<ValueMap> = Vec::new();
            let mut tables:Vec<ValueMap> = Vec::new();
            let mut columns:Vec<ValueMap> = Vec::new();
            tables = self.get_tables("main","main").await.unwrap();
            let schema_do = crate::domain::metadata::database::Schema {
                name: "main".to_string(),
                tables: Some(HashMap::new()),
                functions: Some(HashMap::new()),
                procedures: Some(HashMap::new()),
                views: Some(HashMap::new()),
                constraints: Some(HashMap::new()),
                locks: Some(HashMap::new()),
                types: Some(HashMap::new()),
                triggers: Some(HashMap::new()),
                aggregates: Some(HashMap::new()),
                materalized_views: Some(HashMap::new()),
                catalogs: Some(HashMap::new()),
                sequences: Some(HashMap::new()),
                roles : Some(HashMap::new()),
                type_: Some("schema".to_string()),
            };

            self.init_schemas(&schemas, "main").await;
           // let schema_node = db_node.value_mut();
           // self.databases.get_mut("main")unwrap().schemas.
            //tables = self.get_tables("main","main").await.unwrap();
            log::info!("Databases: {:?}",self.databases);
            self.init_tables(&tables,"main","main").await;
            //log::info!("Inited tables: ")
           log::info!("table inited???");
            for table in tables{
                log::info!("Current table: {:?}", table);
              for t in table.0{
                let table_namae = t.1.as_str().unwrap();
                columns = self.get_columns("main",table_namae,"main").await.unwrap();
                self.init_columns(&columns,"main","main",table_namae).await;
                let ind = self.get_indexes("main",table_namae,"main").await.unwrap();
                self.init_indexes(&ind,"main","main",table_namae).await;
               // let constr = self.get_constraints(db_namae,schema_namae,table_namae).await.unwrap();
                //self.init_constraints(&constr,db_namae,schema_namae,table_namae).await;
                //let functions = self.get_functions(db_namae).await.unwrap();
                //self.init_functions(&functions,db_namae).await;
              }
            }

            log::info!("Database info after init: {:?}", self.databases);

          /*   schemas.insert(0, schema_node);
            */
           /*  for dab in databases{
                for db_name in dab.0{
                    let db_namae = "main";
                  schemas = self.get_schemas(db_namae).await.unwrap();
                  self.init_schemas(&schemas,db_namae).await;
                  for schema in schemas{
                    for sch in schema.0{
                        let schema_namae = sch.1.as_str().unwrap();
                      tables = self.get_tables(db_namae,schema_namae).await.unwrap();
                      self.init_tables(&tables,db_namae,schema_namae).await;
                      for table in tables{
                        for t in table.0{
                            let table_namae = t.1.as_str().unwrap();
                          columns = self.get_columns(db_namae,table_namae,schema_namae).await.unwrap();
                          self.init_columns(&columns,db_namae,schema_namae,table_namae).await;
                          let ind = self.get_indexes(db_namae,table_namae,schema_namae).await.unwrap();
                          self.init_indexes(&ind,db_namae,schema_namae,table_namae).await;
                         // let constr = self.get_constraints(db_namae,schema_namae,table_namae).await.unwrap();
                          //self.init_constraints(&constr,db_namae,schema_namae,table_namae).await;
                          //let functions = self.get_functions(db_namae).await.unwrap();
                          //self.init_functions(&functions,db_namae).await;
                        }
                      }
                    let views = self.get_views(db_namae,schema_namae).await.unwrap();
                    self.init_views(&views,db_namae,schema_namae).await;

                    //let stored_procs = self.get_stored_procedures(db_namae,schema_namae).await.unwrap();
                    //self.init_stored_procedures(&stored_procs,db_namae,schema_namae).await;
                      let trigfuncs = self.get_trigger_functions(db_namae).await.unwrap();
                      self.init_trigger_functions(&trigfuncs,db_namae).await;
                      let seqs = self.get_sequences(db_namae,schema_namae).await.unwrap();
                      //self.init_sequences(&seqs,db_namae,schema_namae).await;
                      //let matv = self.get_materalized_views(db_namae).await.unwrap();
                      //self.init_materalized_views(&matv,db_namae,schema_namae).await;
                      //let fdw = self.get_foreign_data_wrappers(db_namae).await.unwrap();
                      //self.init_foreign_data_wrappers(&fdw,db_namae).await;
                      //let typez = self.get_types(db_namae).await.unwrap();
                      //self.init_types(&typez,db_namae,schema_namae).await;
                    }
                  }
                }
            }*/
        }

        async fn init_tables(&self,result2:&Vec<ValueMap>,db_name:&str,schema_name:&str){
            log::info!("init table???");
            if let Some(mut node) = self.databases.get_mut("main"){
                    log::info!("Got the tabée node?");
                if let Some(mut schemas) = node.schemas.as_mut(){
                    log::info!("Schemass: {:?}",schemas);
                    let table_map = schemas.get_mut("main").unwrap().tables.get_or_insert_with(HashMap::new);
                    for table in result2{
                        log::info!("Inside a table result?");
                        let table_name = table.0.get(&Value::String("name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let tb_node = Table{
                            name : table_name.to_string(), 
                            columns : Some(HashMap::new()),
                            constraints: Some(HashMap::new()),
                            indexes: Some(HashMap::new()),
                            triggers: Some(HashMap::new()),
                            rules: Some(HashMap::new()),
                            rls_policies: Some(HashMap::new()),
                            type_: Some("table".to_string()),
                            schema_name:Some(schema_name.to_string()),
                            db_name : db_name.to_string(),
                        };
                        table_map.insert(table_name.to_string(), tb_node);
                    }
                } 
                    log::info!("Table inited?");
            }
        }

        async fn init_columns(&self,result2:&Vec<ValueMap>,db_name:&str,schema_name:&str,table_name:&str){
            if let Some(mut node) = self.databases.get_mut(db_name){
                    if let Some(table_map) = node.schemas.as_mut().unwrap().get_mut(schema_name).unwrap().tables.as_mut().unwrap().get_mut(table_name){
                        let columns_map = table_map.columns.get_or_insert_with(HashMap::new);
                        for col in result2{
                                let col_name = col.0.get(&Value::String("name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                let data_type = col.0.get(&Value::String("type".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                let is_nullable = col.0.get(&Value::String("notnull".to_string())).and_then(|v| v.as_bool()).unwrap_or_default();
                                let column_default = col.0.get(&Value::String("dflt_value".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                
                                let _col_node = crate::domain::metadata::column::Column{
                                    name : String::from(col_name),//col_name.1.to_string(),
                                    data_type: Some(String::from(data_type)),     
                                    is_nullable: Some(is_nullable),
                                    default_value: Some(String::from(column_default)),
                                    is_primary_key: None,
                                    maximum_length: None,
                                    type_: "column".to_string(),
                                    table_name: table_name.to_string(),
                                    schema_name:Some(schema_name.to_string()),
                                    db_name: db_name.to_string()
                                };
                                
                                columns_map.insert(col_name.to_string(), _col_node);
                        }
                    }
                
            }
        }

        async fn init_views(&self,result2:&Vec<ValueMap>,db_name:&str,schema_name:&str){
            if let Some(mut node) = self.databases.get_mut(db_name){
                    let view_map = node.schemas.as_mut().unwrap().get_mut(schema_name).unwrap().views.get_or_insert_with(HashMap::new);//node.value_mut().views.get_or_insert_with(HashMap::new);
                    for view in result2{
                            let view_name = view.0.get(&Value::String("name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let view_definition = view.0.get(&Value::String("sql".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let view_node = View{
                                name : view_name.to_string(),
                                definition : view_definition.to_string(),
                                type_:"view".to_string(),
                                schema_name: schema_name.to_string(),
                                db_name: db_name.to_string(),
                            };
                            view_map.insert(view_name.to_string(), view_node);
                        
                    }
                

            }else{
                log::info!("Node is not OK");
            }
        }

        async fn init_trigger_functions(&self,result2:&Vec<ValueMap>,db_name:&str){
            if let Some(mut node) = self.databases.get_mut(db_name){
                let trigger_map = node.value_mut().schemas.as_mut().unwrap().get_mut("public").unwrap().triggers.get_or_insert_with(HashMap::new);
                for trigger in result2{
                        let trigger_name = trigger.0.get(&Value::String("name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let trigger_definition = trigger
                    .0
                    .get(&Value::String("trigger_definition".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                        let trigger_node = trigger::TriggerFunction{
                            name : trigger_name.to_string(),
                            definition : trigger_definition.to_string(),
                            type_: "trigger".to_string(),
                            db_name: db_name.to_string(),
                            schema_name: "public".to_string(),
                        };
                        trigger_map.insert(trigger_name.to_string(), trigger_node);
                    
                }
            }else{
                log::info!("Node is not OK");
            }
        }


        async fn init_schemas(&self,result2:&Vec<ValueMap>,db_name:&str){
            if let Some(mut node) = self.databases.get_mut("main") {
                let schemamap = node.value_mut().schemas.get_or_insert_with(HashMap::new);
                let schema_node = crate::domain::metadata::database::Schema {
                        name: "main".to_string(),
                        tables: Some(HashMap::new()),
                        functions: Some(HashMap::new()),
                        procedures: Some(HashMap::new()),
                        views: Some(HashMap::new()),
                        constraints: Some(HashMap::new()),  
                        locks: Some(HashMap::new()),
                        types: Some(HashMap::new()),
                        triggers: Some(HashMap::new()),
                        aggregates: Some(HashMap::new()),
                        materalized_views: Some(HashMap::new()),
                        catalogs: Some(HashMap::new()),
                        sequences: Some(HashMap::new()),
                        roles : Some(HashMap::new()),
                        type_: Some("schema".to_string()),
                    };
                schemamap.insert("main".to_string(), schema_node);
            }
        }

        async fn init_indexes(&self,result2:&Vec<ValueMap>,db_name:&str,schema_name:&str,table_name:&str){
            if let Some(mut node) = self.databases.get_mut(db_name){
                    if let Some(table_map) = node.schemas.as_mut().unwrap().get_mut(schema_name).unwrap().tables.as_mut().unwrap().get_mut(table_name){//node.value_mut().tables.clone().unwrap().get_mut(db_name){
                        let index_map = table_map.indexes.get_or_insert_with(HashMap::new);
                        for index in result2{
                            let index_name = index.0.get(&Value::String("index_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            if index_map.contains_key(index_name){
                                index_map.get(index_name).unwrap().column_name.clone().unwrap().push(index_name.to_string());
                            }else{
                                let non_unique = index.0.get(&Value::String("non_unique".to_string())).and_then(|v| v.as_bool()).unwrap_or_default();
                                let table_name = index.0.get(&Value::String("table_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                
                                let index_node = crate::domain::metadata::index::Index{
                                    name : String::from(index_name),
                                    definition: None,     
                                    column_name: None,
                                    non_unique: Some(non_unique),
                                    table_name: Some(String::from(table_name)),
                                    db_name: db_name.to_string(),
                                    schema_name:Some(schema_name.to_string()),
                                    type_:"index".to_string(),
                                    
                                };
                                index_map.insert(index_name.to_string(), index_node);
                            }
                        }
                    }
            }else{
                log::info!("Node is not OK");
            }
        }

        pub fn get_row_value<'a>(&'a self,table_name:&'a str,db_name:&'a str,columns:&'a Vec<String>)->impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send + 'a{
            async move{
                let rb = self.rbatis_connect(db_name).await?.unwrap();
                let _sql = format!("SELECT {} FROM {};",columns.join(","),table_name);
                let result:Vec<ValueMap> = rb.query_decode(&_sql,vec![]).await.unwrap();
                println!("row get result: {:?}",result);
                Ok(result)
            }
        }


    //CREATES 
    pub fn create_table<'a>(
        &'a self,
        db_name: &'a str,
        table_info: &'a CreateTableInfo,
    ) -> impl Future<Output = Result<Table, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(db_name) {
                let mut col_transformed_datas: Vec<String> = Vec::new();
                let mut col_map: HashMap<String, Column> = HashMap::new();
    
                for col in &table_info.columns {
                    let pk = if col.is_primary_key.unwrap() { "PRIMARY KEY" } else { "" };
                    let is_nullable = if !col.is_nullable.unwrap() { "NOT NULL" } else { "" };
                    
                    col_transformed_datas.push(format!(
                        "{} {} {} {}",
                        col.name,
                        col.data_type.as_ref().unwrap(),
                        pk,
                        is_nullable
                    ));
                    col_map.insert(col.name.clone(), col.clone());
                }
                let sql = format!(
                    "CREATE TABLE IF NOT EXISTS {} ({});",
                    table_info.table_name,
                    col_transformed_datas.join(", ")
                );
    
                println!("SQL: {:?}", sql);
                let result: Result<(),rbdc::Error> = rb.query_decode(&sql, vec![]).await;
                
                let new_table = Table {
                    name: table_info.table_name.clone(),
                    columns: Some(col_map),
                    constraints: None,
                    indexes: None,
                    triggers: None,
                    rules: None,
                    rls_policies: None,
                    type_: Some("table".to_string()),
                    schema_name:Some("public".to_string()),
                    db_name: table_info.table_name.clone(),
                };


    
                Ok(new_table)
            } else {
                Err(rbdc::Error::from("Database not found"))
            }
        }
    }

    pub fn create_index<'a>(
        &'a self,
        index_info: &'a CreateIndexInfo,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Index, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let sql = format!(
                    "CREATE INDEX IF NOT EXISTS {} ON {} ({});",
                    index_info.index_name,
                    index_info.table_name,
                    index_info.columns.join(", ")
                );
    
                println!("SQL: {:?}", sql);
                let index = Index {
                    name: index_info.index_name.clone(),
                    definition: todo!(),
                    column_name: Some(index_info.columns.clone()),
                    non_unique: todo!(),
                    table_name: Some(index_info.table_name.clone()),
                    db_name: database_name.to_string(),
                    schema_name: Some("public".to_string()),
                    type_: "index".to_string(),
                };
                
                let result: Result<(), rbdc::Error> = rb.query_decode(&sql, vec![]).await;
                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(&index_info.schema_name)
                    .unwrap()
                    .tables
                    .as_mut()
                    .unwrap()
                    .get_mut(&index_info.table_name)
                    .unwrap()
                    .indexes
                    .as_mut()
                    .unwrap()
                    .insert(index_info.index_name.clone(), index.clone());
                Ok(index)
                //result
            } else {
                Err(rbdc::Error::from("Database not found"))
            }
        }
    }

    pub fn create_view<'a>(
        &'a self,
        view_info: &'a CreateViewInfo,
        database_name: &'a str,
    ) -> impl Future<Output = Result<View, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let sql = format!(
                    "CREATE VIEW IF NOT EXISTS {} AS {};",
                    view_info.view_name, view_info.stmt
                );
    
                println!("SQL: {:?}", sql);
                let result: Result<(), rbdc::Error> = rb.query_decode(&sql, vec![]).await;
                let view: View = View {
                    name: view_info.view_name.clone(),
                    definition: view_info.stmt.clone(),
                    type_: "view".to_string(),
                    db_name: database_name.to_string(),
                    schema_name: database_name.to_string(),
                };
                log::info!("Result: {:?}", result);
                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(database_name)
                    .unwrap()
                    .views
                    .as_mut()
                    .unwrap()
                    .insert(view_info.view_name.clone(), view.clone());
                Ok(view)
                //result
            } else {
                Err(rbdc::Error::from("Database not found"))
            }
        }
    }

    pub fn create_trigger<'a>(
        &'a self,
        name: &'a str,
        when: &'a str,
        type_: &'a str,
        table_name: &'a str,
        function_body: &'a str, // SQLite does not use separate function names
        database_name: &'a str,
    ) -> impl Future<Output = Result<Trigger, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let sql = format!(
                    "CREATE TRIGGER IF NOT EXISTS {} {} {} ON {} BEGIN {}; END;",
                    name, when, type_, table_name, function_body
                );
    
                println!("SQL: {:?}", sql);
                let result: Result<(), rbdc::Error> = rb.query_decode(&sql, vec![]).await;
                let trigger: Trigger = Trigger {
                    name: name.to_string(),
                    definition: function_body.to_string(),
                    type_: type_.to_string(),
                    db_name: database_name.to_string(),
                    schema_name: "public".to_string(),
                };
                log::info!("Result: {:?}", result);
                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(database_name)
                    .unwrap()
                    .tables
                    .as_mut()
                    .unwrap()
                    .get_mut(table_name)
                    .unwrap()
                    .triggers
                    .as_mut()
                    .unwrap()
                    .insert(name.to_string(), trigger.clone());
                Ok(trigger)
               // result
            } else {
                Err(rbdc::Error::from("Database not found"))
            }
        }
    }
    
    //EDITS

    pub async fn alter_table_column<'a>(
        &'a self,
        table_name: &'a str,
        new_col: Column,
        old_col: Column,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Column, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let col_rename = if new_col.name != old_col.name {
                    format!(
                        "ALTER TABLE {} RENAME COLUMN {} TO {};",
                        table_name, old_col.name, new_col.name
                    )
                } else {
                    "".to_string()
                };
    
                // SQLite does not support ALTER COLUMN TYPE, so we create a new table
                let create_new_table = format!(
                    "CREATE TABLE {}_new AS SELECT * FROM {};",
                    table_name, table_name
                );
                let drop_old_table = format!("DROP TABLE {};", table_name);
                let rename_table = format!("ALTER TABLE {}_new RENAME TO {};", table_name, table_name);
    
                let migration_sql = format!(
                    "
                    BEGIN;
                    {};
                    {};
                    {};
                    COMMIT;
                    ",
                    create_new_table, drop_old_table, rename_table
                );
    
                log::info!("Executing migration SQL: {}", migration_sql);
                let result: Result<(), rbdc::Error> = rb.query_decode(&migration_sql, vec![]).await;
                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(database_name)
                    .unwrap()
                    .tables
                    .as_mut()
                    .unwrap()
                    .get_mut(table_name)
                    .unwrap()
                    .columns
                    .as_mut()
                    .unwrap()
                    .insert(new_col.name.clone(), new_col.clone());
                Ok(new_col)
                //result
            
        }else{
            Err(rbdc::Error::from("Database not found"))
        }
    }
    }
    
    pub async fn edit_constraint<'a>(
        &'a self,
        table_name: &'a str,
        new_constraint: Constraint,
        old_constraint: Constraint,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Constraint, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                log::info!("Editing constraint on table: {:?}", table_name);
                let constraint_clone = new_constraint.clone();
                let drop_constraint = format!("PRAGMA foreign_keys=OFF;"); // Turn off constraints
                let create_new_table = format!(
                    "CREATE TABLE {}_new AS SELECT * FROM {};",
                    table_name, table_name
                );
                let drop_old_table = format!("DROP TABLE {};", table_name);
                let rename_table = format!("ALTER TABLE {}_new RENAME TO {};", table_name, table_name);
                let reenable_foreign_keys = format!("PRAGMA foreign_keys=ON;");
    
                let migration_sql = format!(
                    "
                    BEGIN;
                    {};
                    {};
                    {};
                    {};
                    {};
                    COMMIT;
                    ",
                    drop_constraint, create_new_table, drop_old_table, rename_table, reenable_foreign_keys
                );
    
                log::info!("Executing constraint migration SQL: {}", migration_sql);
                let result: Result<(), rbdc::Error> = rb.query_decode(&migration_sql, vec![]).await;
                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(&old_constraint.schema_name.unwrap())
                    .unwrap()
                    .tables
                    .as_mut()
                    .unwrap()
                    .get_mut(table_name)
                    .unwrap()
                    .constraints
                    .as_mut()
                    .unwrap()
                    .insert(new_constraint.name.clone(), constraint_clone);
                Ok(new_constraint)
                //result
            } else {
                Err(rbdc::Error::from("Database not found"))
            }
        }
    }
    
    pub async fn edit_index<'a>(
        &'a self,
        table_name: &'a str,
        new_index: Index,
        old_index: Index,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Index, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let index_clone = new_index.clone();
                let drop_sql = format!("DROP INDEX IF EXISTS {};", old_index.name);
                let create_sql = format!(
                    "CREATE {} INDEX {} ON {} ({});",
                    if new_index.non_unique.unwrap() { "" } else { "UNIQUE" },
                    new_index.name,
                    table_name,
                    new_index.column_name.unwrap().join(", ")
                );
    
                let migration_sql = format!(
                    "
                    BEGIN;
                    {};
                    {};
                    COMMIT;
                    ",
                    drop_sql, create_sql
                );
    
                log::info!("Executing index migration SQL: {}", migration_sql);
                let result: Result<(), rbdc::Error> = rb.query_decode(&migration_sql, vec![]).await;
                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(&old_index.schema_name.unwrap())
                    .unwrap()
                    .tables
                    .as_mut()
                    .unwrap()
                    .get_mut(table_name)
                    .unwrap()
                    .indexes
                    .as_mut()
                    .unwrap()
                    .insert(new_index.name.clone(), index_clone.clone());
                Ok(index_clone)
                //result
            } else {
                Err(rbdc::Error::from("Database not found"))
            }
        }
    }

    pub async fn edit_view<'a>(
        &'a self,
        old_view: View,
        new_view: View,
        database_name: &'a str,
    ) -> impl Future<Output = Result<View, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let view_clone = new_view.clone();
                let drop_sql = format!("DROP VIEW IF EXISTS {};", old_view.name);
                let create_sql = format!(
                    "CREATE VIEW {} AS {};",
                    new_view.name, new_view.definition
                );
    
                let migration_sql = format!(
                    "
                    BEGIN;
                    {};
                    {};
                    COMMIT;
                    ",
                    drop_sql, create_sql
                );
    
                log::info!("Executing view migration SQL: {}", migration_sql);
                let result: Result<(), rbdc::Error> = rb.query_decode(&migration_sql, vec![]).await;
                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(&old_view.schema_name)
                    .unwrap()
                    .views
                    .as_mut()
                    .unwrap()
                    .insert(new_view.name.clone(), view_clone);
                Ok(new_view)
                //result
            } else {
                Err(rbdc::Error::from("Database not found"))
            }
        }
    }
    

    pub async fn edit_function<'a>(
        &'a self,
        old_function: Function,
        new_function: Function,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Function, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                if old_function != new_function {
                    let function_clone = new_function.clone();
                    let drop_sql = format!("DROP FUNCTION IF EXISTS {};", old_function.name);
                    let create_sql = format!(
                        "CREATE FUNCTION {}({:?}) BEGIN {}; END;",
                        new_function.name, new_function.parameters, new_function.definition
                    );
    
                    let migration_sql = format!(
                        "
                        BEGIN;
                        {};
                        {};
                        COMMIT;
                        ",
                        drop_sql, create_sql
                    );
    
                    log::info!("Executing function migration SQL: {}", migration_sql);
                    let result: Result<(), rbdc::Error> = rb.query_decode(&migration_sql, vec![]).await;
                    self.databases
                        .get_mut(database_name)
                        .unwrap()
                        .schemas
                        .as_mut()
                        .unwrap()
                        .get_mut(&old_function.schema_name.unwrap())
                        .unwrap()
                        .functions
                        .as_mut()
                        .unwrap()
                        .insert(new_function.name.clone(), function_clone);
                    Ok(new_function)
                    //result
                } else {
                    Err(rbdc::Error::from("No changes detected"))
                }
            } else {
                Err(rbdc::Error::from("Database not found"))
            }
        }
    }
    
    
    

}

impl DatabaseRepository for SqLiteRepository{
    async fn get_databases(&self)-> Result<Vec<ValueMap>,rbdc::Error> {
        log::info!("Sqlite: Get databases");
        let rb = self.rbatis_connect("sqlite").await?.unwrap();
        log::info!("Connection RB: {:?}", rb.pool);
        let _sql = "SELECT * FROM pragma_database_list;";//"PRAGMA database_list;";
       // let result = rb.query(_sql,vec![]).await.unwrap();
        let result2:Vec<ValueMap> = rb.query_decode(_sql,vec![]).await.unwrap();
        log::info!("Database1s: {:#?}",result2);
       // log::info!("Database2s: {:#?}",result);
        let mut new_v:Vec<ValueMap> = vec![];
        let mut value_map:ValueMap = ValueMap::new();
        value_map.insert(Value::from("name"), Value::from("main"));
        new_v.push(value_map);
        //iterate through databases and insert into the pool and the database map(db.1 = database name)
        /*let db_metadata = DatabaseMetadata{
            name : String::from("main"),
            schemas : Some(HashMap::new()),
            foreign_data_wrappers : None,
            catalogs : None,
            type_:"database".to_string()
        };
        self.databases.insert(String::from("main"), db_metadata);*/
       /* if let Some(databases) = result.as_array(){
            log::info!("Database main: {:?}", databases.get(0));
            for db_val in databases{
                for db in db_val{
                    log::info!("new pool adding... database: {:?}",db);

                   // if !self.rb_map.contains_key(db.1.as_str().unwrap()){
                        let rb = Arc::new(rbatis::RBatis::new());
                        log::info!("new pool adding... database: {:?}",db);
                        self.rb_map.insert(db.1.to_string(),rb);

                        let db_node = Schema{
                            name : db.1.to_string(),
                            tables : Some(HashMap::new()),
                            functions : Some(HashMap::new()),                        
                            procedures : Some(HashMap::new()),
                            views : Some(HashMap::new()),
                            constraints : Some(HashMap::new()),
                           // foreign_data_wrappers : Some(HashMap::new()),
                            locks : Some(HashMap::new()),
                            types : Some(HashMap::new()),
                            triggers : Some(HashMap::new()),
                            aggregates : Some(HashMap::new()),
                            materalized_views : Some(HashMap::new()),
                            catalogs : Some(HashMap::new()),
                            sequences : Some(HashMap::new()),
                            roles : None,
                            type_:Some("schema".to_string()),
                        };

                        let db_metadata = DatabaseMetadata{
                            name : db.1.to_string(),
                            schemas : Some(HashMap::new()),
                            foreign_data_wrappers : None,
                            catalogs : None,
                            type_:"database".to_string()
                        };
                        self.databases.insert(db.1.to_string(), db_metadata);*/
                 //   }
               // }
           // }
       // }
        Ok(new_v)
    }

    async fn get_tables(&self, db_name:&str,schema_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();
        let _sql = "SELECT name
                        FROM sqlite_master
                        WHERE type = 'table';";
        //let result = rb.query(_sql,vec![]).await.unwrap();
        let result2:Vec<ValueMap> = rb.query_decode(_sql,vec![]).await.unwrap();
        log::info!("Tables1: {:#?}",result2);
    //    log::info!("Tables2: {:#?}",result);
        if result2.is_empty(){
            return Ok(Vec::new());
        }
        
        Ok(result2)
    }

    ///Get all columns in the table
    async fn get_columns(&self,db_name:&str,table_name:&str,schema_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = format!("PRAGMA table_info({});",table_name);
        
        let result2:Vec<ValueMap> = rb.query_decode(&_sql, vec![]).await.unwrap();

        Ok(result2)
    }

    async fn get_views(&self,db_name:&str,schema_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();
        
        let _sql = "SELECT name,sql
                        FROM sqlite_master
                        WHERE type = 'view';";
        let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

        Ok(result2)
    }

    async fn get_stored_procedures(&self,db_name:&str,schema_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
       Ok(Vec::new())
    }

    async fn get_functions(&self,db_name:&str,schema_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())
    }

    async fn get_trigger_functions(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql =  "SELECT name 
                        FROM sqlite_master 
                        WHERE type = 'trigger';";
        let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result2)
    }

    async fn get_event_triggers(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())

    }

    async fn get_aggregates(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())
    }

    async fn get_materalized_views(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())
    }

    async fn get_types(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())
    }

    async fn get_languages(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())

    }

    async fn get_catalogs(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())
    }

    async fn get_foreign_data_wrappers(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())
    }

    //TODO SELECT table_name FROM information_schema.tables WHERE table_schema = '?'; catalogobjects(?)
    async fn get_schemas(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())
    }

    async fn get_indexes(&self,db_name:&str,table_name:&str,schema_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT name AS index_name, sql AS definition
                        FROM sqlite_master 
                        WHERE type = 'index';";
        let result = rb.query(_sql, vec![]).await.unwrap();
        let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

        Ok(result2)
    }

    async fn get_constraints(&self,db_name:&str,table_name:&str,schema_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())

    }

    async fn get_sequences(&self,db_name:&str,schema_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())

    }

    async fn get_roles_and_users(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())

    }

    async fn get_table_statistics(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())


    }

    async fn get_active_sessions(&self)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())


    }

    async fn get_locks(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())


    }

    async fn get_partitions(&self,db_name:&str,table_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())


    }

    async fn get_user_privileges(&self,db_name:&str,user_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())

    }

    async fn get_database_settings(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())

    }

    async fn get_foreign_key_relationships(&self,db_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())

    }

    async fn get_triggers_associated_with_table(&self,db_name:&str,table_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())

    }

    async fn get_default_columns_value(&self,db_name:&str,table_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())


    }

    async fn get_rls_policies(&self,db_name:&str,table_name:&str,schema_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())


    }

    async fn get_rules(&self,db_name:&str,table_name:&str)-> Result<Vec<ValueMap>,rbdc::Error> {
        Ok(Vec::new())
    }
}



#[cfg(test)]
mod tests {
    use std::result;

    use crate::domain::metadata::database::Schema;
    use crate::domain::metadata::sequence::Sequence;
    use crate::DatabaseConnection;

    use super::*;
    use mockall::*;
    use mockall::predicate::*;
    use rbatis::dark_std::sync::vec;
    use tokio::test;
    use rbatis::rbatis::RBatis;
    use crate::domain::repository::mysql_repository::MySqlRepository;
    
    /// ✅ **Helper function to setup test database connection**
    async fn setup_test_db() -> SqLiteRepository {
       // let rb = rbatis::RBatis::new();
        //rb.init(rbdc_pg::driver::PgDriver {}, "postgres://test_user:test_password@localhost/test_db")
          //  .unwrap();
        let path = format!("sqlite:///home/zetny/Documents/GitHub/dbr/public/test.db");
        let rb = Arc::new(rbatis::rbatis::RBatis::new());
        rb.init(SqliteDriver {}, &path);
        let repo = SqLiteRepository::new(&path);
        repo.rb_map.insert("test".to_string(),rb.clone());
        repo
    }

    #[tokio::test]
    async fn test_create_table_is_ok() {
        let repo = setup_test_db().await;
        // ✅ **Step 1: Define the table schema**   
      //  let test_table = CreateTableInfo {table_name:"test_table".to_string(),columns:vec![Column{name:"id".to_string(),data_type:Some("SERIAL".to_string()),is_primary_key:Some(true),is_nullable:Some(false),default_value:None,maximum_length:None, table_name: todo!(), db_name: todo!(), type_: todo!() },Column{name:"name".to_string(),data_type:Some("VARCHAR(255)".to_string()),is_primary_key:Some(false),is_nullable:Some(false),default_value:None,maximum_length:Some(255), table_name: todo!(), db_name: todo!(), type_: todo!() },], db_name: todo!() 
      //  };
      let table_t: CreateTableInfo = CreateTableInfo{table_name:"testTable".to_string(),columns: vec![Column{ name: "namajo".to_string(), data_type: Some("INT".to_string()), is_nullable: Some(true), default_value: Some("".to_string()), is_primary_key: Some(false), maximum_length: Some(0), table_name: "test_t".to_string(), db_name: "test_db".to_string(), type_: "column".to_string(), schema_name: Some("test".to_string()) }],db_name:"test".to_string(), schema_name: "test".to_string()};

        println!("Test table: {:?}",table_t);
        let result = repo.create_table("test",&table_t).await;
        println!("Is error? {}",result.is_err());
        assert!(result.is_ok(), "Table creation should succeed");

        // ✅ **Step 4: Check if the table exists in the database**
  //      let check_query = "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'tablaeetest');";
    //    let exists: (bool,) = repo.rb_map.get("test_db").unwrap().query_decode(check_query, vec![]).await.unwrap();
    //    assert!(exists.0, "Table should exist in the database");

        // ✅ **Step 5: Cleanup (Drop the test table)**
        let drop_query = "DROP TABLE IF EXISTS testTable;";
        repo.rb_map.get("test").unwrap().exec(drop_query, vec![]).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_table_database_not_exists(){
        let repo = setup_test_db().await;

        let table_t: CreateTableInfo = CreateTableInfo{table_name:"tableTest".to_string(),columns: vec![Column{name:"namajo".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: Some("test".to_string()) }],db_name:"akela".to_string(), schema_name: "test".to_string() };
        
        let result = repo.create_table
        ("testDb",&table_t).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_index_is_ok() {
        let repo = setup_test_db().await;
        let table_t: CreateTableInfo = CreateTableInfo{table_name:"test_tabletete".to_string(),columns: vec![Column{name:"id".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: Some("test".to_string()) }],db_name:"test".to_string(), schema_name: "test".to_string()};

        let index_info = CreateIndexInfo {
            index_name: "test_idxq".to_string(),
            table_name: "test_tabletete".to_string(),
            columns: vec!["id".to_string()],
            schema_name: todo!()
        };

        let _ = repo.create_table("test",&table_t).await;  
        let result = repo.create_index(&index_info, "test").await;
        
        println!("Is error? {}",result.is_err());
        println!("Result: {:?}",result);
        
        assert!(result.is_ok());

        let drop_query_t = "DROP TABLE IF EXISTS test_tabletete;";
        repo.rb_map.get("test").unwrap().exec(drop_query_t, vec![]).await.unwrap();
        let drop_query = "DROP INDEX IF EXISTS test_idxq;";
        repo.rb_map.get("test").unwrap().exec(drop_query, vec![]).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_index_err_table_not_exist(){
        let repo = setup_test_db().await;
        let index_info = CreateIndexInfo {
            index_name: "test_idxq".to_string(),
            table_name: "test_tabletete".to_string(),
            columns: vec!["id".to_string()],
            schema_name: todo!()
        };
        let result = repo.create_index(&index_info, "test").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_view_is_ok() {
        let repo = setup_test_db().await;
        let table_t: CreateTableInfo = CreateTableInfo{table_name:"testablae".to_string(),columns: vec![Column{ name: "id".to_string(), data_type: Some("INT".to_string()), is_nullable: Some(true), default_value: Some("".to_string()), is_primary_key: Some(false), maximum_length: Some(0), table_name: "test_t".to_string(), db_name: "test_db".to_string(), type_: "column".to_string(), schema_name: Some("test_db".to_string()) }],db_name:"test".to_string(), schema_name: "test".to_string()};

        let view_info = CreateViewInfo {view_name:"test_view".to_string(),stmt:"SELECT * FROM testablae;".to_string(), columns:vec!["id".to_string()], table_name: "testablae".to_string(), schema_name:"test".to_string()};
        let _ = repo.create_table("test",&table_t).await;  

        let result = repo.create_view(&view_info,"test").await;
        println!("result: {:?}",result);
        assert!(result.is_ok());

        let drop_query_t = "DROP TABLE IF EXISTS testablaa;";
        repo.rb_map.get("test").unwrap().exec(drop_query_t, vec![]).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_view_err_table_not_exist(){
        let repo = setup_test_db().await;
        let view_info = CreateViewInfo {view_name:"test_view".to_string(),stmt:"SELECT * FROM testablae;".to_string(), columns:vec!["id".to_string()], table_name: "testablae".to_string(), schema_name:"test".to_string()};
        let result = repo.create_view(&view_info,"test").await;
        assert!(result.is_err());
    }
    /*#[tokio::test]
    async fn test_create_role_is_ok(){
        let repo = setup_test_db().await;

        let role_info:Role = Role { name: "test_rolee".to_string(), is_super: Some(false), is_insherit: Some(false), is_create_role: Some(false), is_create_db: Some(true), can_login: Some(true), is_replication: Some(false), connection_limit: Some(100), valid_until: Some("".to_string()), password: Some("valamike".to_string()), db_name: "test_db".to_string(), type_: "role".to_string() };
        let result = repo.create_role(role_info, "test_db").await.await;

        println!("result: {:?}",result);
        assert!(result.is_ok());

        let drop_query_t = "DROP ROLE IF EXISTS test_role;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query_t, vec![]).await.unwrap();
    }

     #[tokio::test]
    async fn test_create_role_err_database_not_exist(){
        let repo = setup_test_db().await;

        let role_info:Role = Role { name: "test_rolee".to_string(), is_super: Some(false), is_insherit: Some(false), is_create_role: Some(false), is_create_db: Some(true), can_login: Some(true), is_replication: Some(false), connection_limit: Some(100), valid_until: Some("".to_string()), password: Some("valamike".to_string()), db_name: "test_db".to_string(), type_: "role".to_string() };
        let result = repo.create_role(role_info, "akela_db").await.await;
        
        assert!(result.is_err());
    }*/

    #[tokio::test]
    async fn test_edit_view_is_ok(){
        let repo = setup_test_db().await;

        //let table_t: CreateTableInfo = CreateTableInfo{table_name:"testablae".to_string(),columns: vec![Column{ name: "id".to_string(), data_type: Some("INT".to_string()), is_nullable: Some(true), default_value: Some("".to_string()), is_primary_key: Some(false), maximum_length: Some(0), table_name: "test_t".to_string(), db_name: "test_db".to_string(), type_: "column".to_string() }],db_name:"test_db".to_string()};
        let view_info = CreateViewInfo {view_name:"test_view".to_string(),stmt:"SELECT * FROM testablae;".to_string(), columns:vec!["id".to_string()], table_name: "testablae".to_string(), schema_name:"tes_db".to_string()};
        let old_view_info = View {name:"test_view".to_string(),definition:"SELECT * FROM testablae;".to_string(),type_:"view".to_string(), db_name: "test".to_string(), schema_name: "test".to_string()};
        let new_view_info = View {name:"test_view22".to_string(),definition:"SELECT id FROM testablae;".to_string(),type_:"view".to_string(), db_name: "test".to_string(), schema_name: "test".to_string()};
        let _ = repo.create_view(&view_info,"test").await;

        let result = repo.edit_view(old_view_info, new_view_info, "test").await.await;

        println!("result: {:?}",result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_edit_view_err_old_view_not_exists(){
        let repo = setup_test_db().await;

        let old_view_info = View {name:"test_view".to_string(),definition:"SELECT * FROM testablae;".to_string(),type_:"view".to_string(), schema_name: "test".to_string(), db_name: "test".to_string()};
        let new_view_info = View {name:"test_view22".to_string(),definition:"SELECT id FROM testablae;".to_string(),type_:"view".to_string(), schema_name: "test".to_string(), db_name: "test".to_string()};

        let result = repo.edit_view(old_view_info, new_view_info, "test").await.await;
    
        assert!(result.is_err());
        
    }

    #[tokio::test]
    async fn test_edit_index_is_ok(){
        let repo =  setup_test_db().await;

        let create_index_info = CreateIndexInfo {
            index_name: "test_idxq".to_string(),
            table_name: "test_tabletete".to_string(),
            columns: vec!["id".to_string()],
            schema_name: todo!()
        };
        let old_index_info = Index {
            table_name: Some("test_tabletete".to_string()),
            name: "test_idxq".to_string(),
            definition: Some("".to_string()),
            column_name: Some(vec!["id".to_string()]),
            non_unique: Some(false),
            db_name: "test".to_string(),
            type_: "index".to_string(),
            schema_name: Some("test".to_string())
        };
        let new_index_info = Index {
            table_name: Some("test_tabletete".to_string()),
            name: "test_idxqcq".to_string(),
            definition: Some("".to_string()),
            column_name: Some(vec!["id".to_string()]),
            non_unique: Some(false),
            db_name: "test".to_string(),
            type_: "index".to_string(),
            schema_name: Some("test".to_string())
        };
        let _ = repo.create_index(&create_index_info, "test");
        let result = repo.edit_index("test_tabletete", new_index_info, old_index_info, "test").await.await;
    
        println!("result: {:?}", result);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_edit_index_err_old_index_not_exists(){
        let repo =  setup_test_db().await;

        let create_index_info = CreateIndexInfo {
            index_name: "test_idxq".to_string(),
            table_name: "test_tabletete".to_string(),
            columns: vec!["id".to_string()],
            schema_name: todo!()
        };
        let old_index_info = Index {
            table_name: Some("test_tabletete".to_string()),
            name: "test_idxq".to_string(),
            definition: Some("".to_string()),
            column_name: Some(vec!["id".to_string()]),
            non_unique: Some(false),
            db_name: "test_db".to_string(),
            type_: "index".to_string(),
            schema_name: Some("test_db".to_string())
        };

        let new_index_info = Index {
            table_name: Some("test_tabletete".to_string()),
            name: "test_idxqcq".to_string(),
            definition: Some("".to_string()),
            column_name: Some(vec!["id".to_string()]),
            non_unique: Some(false),
            db_name: "test".to_string(),
            type_: "index".to_string(),
            schema_name: Some("test".to_string())
        };
        let _ = repo.create_index(&create_index_info, "test");
        let result = repo.edit_index("test_tabletete", new_index_info, old_index_info, "test").await.await;
    
        assert!(result.is_err());
    }

}