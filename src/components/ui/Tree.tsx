import { Column, Table, Functions, Sequence, View, Procedure, Schema, Constraint, Index, Role, TriggerFunction } from "../../Types/DatabaseTypes";

export interface Tree{
    children: Tree[];
    name:string;
    type:string;
    root_name:string;
    object?:Column | Table | Functions | Sequence | View | Procedure | Schema | Constraint | Index | TriggerFunction | Role
    table_name?:string;
    schema_name?:string;
}

const createTreeNode = (name:string, type:string, object:any, root_name:string, objectObJ?:Column | Table | Functions | Sequence | View | Procedure | Constraint | Index | Schema | TriggerFunction | Role, _table_name?:string,_schema_name?:string):Tree => {
    const children:Tree[] = [];
    for(const key in object){
        if(object[key] && typeof object[key] == 'object'){
         // console.log("Object tree node type: ",object[key].type_);
          if(object[key].type_ == "column" || object[key].type_ == "table" || object[key].type_ == "function" || object[key].type_ == "sequence" || object[key].type_ == "view" || object[key].type_ == "procedure" || object[key].type_ == "schema" || object[key].type_ == "index" || object[key].type_ == "constraint"){  
           // console.log("Is there any table name: ", object[key]);
           children.push(createTreeNode(key,object[key].type_,object[key], root_name,object[key],object[key].table_name,object[key].schema_name));
          }else{
            if(object[key].type_ == "triggerfunction"){
              console.log("Trigger function: ",object[key]);
            }
            children.push(createTreeNode(key,object[key].type_,object[key],root_name,object[key]));
          //  children.push(createTreeNode(key,name,object[key].type_,))
          }
        }
      }
      if(objectObJ){
        return { name, children,type:type, object:objectObJ,root_name:root_name}
      }
      return { name, children,type:type,root_name:root_name};
    
} 

export function convertDatabaseMetadataToTreeNode(database: any) {
    console.log("Kapott nagyhatalmi adatbázis: ", database.schemas);
  // console.log("Dat da type: ",database.type_);
     return createTreeNode(database.name,database.type_,{
       schemas: database.schemas,
       foreign_data_wrappers: database.foreign_data_wrappers,
       catalogs: database.catalogs 
     },database.name);
   }