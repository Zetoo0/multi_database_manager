//import { FormField } from "../../Types/object/FormField";
import { dataTypeOptions } from "../../Types/object/DataTypeOption";
//import { Button } from "../ui/Button";
//import { Input } from "../ui/Input";
//import { Select } from "../ui/Select";
//import { Table as UITable } from "../ui/Table";
import {  formSchemas } from "./FormSchema";
import {  Constraint, Function,  /*Functions,*/  Index,  Role, View } from "../../Types/DatabaseTypes";
import { useExecuteCreateSequence,CreateSequenceInfo, CreateTableInfo, useExecuteCreateTableQuery, useExecuteCreateRole, useExecuteCreateFunction, useExecuteCreateView, useExecuteCreateIndex, useExecuteCreateTrigger, useExecuteCreateDatabaseQuery, useExecuteCreateConstraint } from "../../hooks/query/useExecuteCreateQuery";
//import { useExecuteEditConstraint, useExecuteEditFunction, useExecuteEditIndex, useExecuteEditSequence, useExecuteEditTableColumn, useExecuteEditView } from "../../hooks/query/useExecuteEditQuery";
import { useMetadata } from "../../providers/MetadataProvider";
//import { SqlEditor } from "../query/Editor";
import { useConnection } from "../../providers/ConnectionProvider";
import { GenericForm } from "./GenericObjectForm";
import { useQueryClient } from "@tanstack/react-query";
//import { Translation, useTranslation } from "react-i18next";
//import { useFileOpenDialogCreateDatabase } from "../../hooks/system/useFileOpenDialog";
//import { useGetTablesOfDatabase } from "../../hooks/system/databaseMetadataQuery";
//import { useGetColumnsOfTable } from "../../hooks/system/databaseMetadataQuery";



/*
<Select onValueChange={(e) => handleChange(field.key, e)} options={field.options || dynamicOptions?.["PostgreSQL"]}} />
              /*<select
                className="border rounded p-2 color-gray-700"
                required={field.required}
                onChange={(e) => handleChange(field.key, e.target.value)}
              >
                <option value="" className="text-gray-700">Select...</option>
                {(field.options || dynamicOptions?.["PostgreSQL"])?.map((option) => (  
                  <option key={option} value={option} className="text-gray-700">
                    {option}
                  </option>
                ))}
              </select>*/
export const CreateEntityForm = ({ type, databaseName, schemaName }: { type: string, databaseName: string, schemaName: string}) => {

                const { saveSequenceData, /*sequenceData,isSuccessSequence*/} = useExecuteCreateSequence();
               // const {saveTableData} = useExecuteCreateTableQuery();
                const {saveTableData/*,isSuccessTable, tableData*/} = useExecuteCreateTableQuery();
                const {saveFunctionData, /*isSuccessFunction, functionData*/} = useExecuteCreateFunction();
                const {saveRoleData,roleData,/*isSuccessRole,*/isPending} = useExecuteCreateRole();
                const {saveViewData, /*viewData, isSuccessView*/} = useExecuteCreateView();
                const {saveIndexData,/*indexData,isSuccessIndex*/} = useExecuteCreateIndex();
                const {saveTriggerData,/*triggerData,isSuccessTrigger*/} = useExecuteCreateTrigger();
                const {saveDatabaseData} = useExecuteCreateDatabaseQuery();
                const {saveConstraintData} = useExecuteCreateConstraint();
                const {currentDatabaseType} = useConnection();
                const {databaseMetadata} = useMetadata();
                const queryClient = useQueryClient();
                const editorState = queryClient.getQueryData<string>(['editorState']) || "";
                  const handleSubmit = (data: Record<string, any>) => {
                    console.log("Handle submit type: ", type)
                      switch (type) {
                        case "database":
                        console.log("Submitted Database Data:", data);
                        saveDatabaseData({
                          file_content: data.file_content, database_name: data.database_name,
                          database_type: currentDatabaseType
                        });  
                        break;
                        case "table":
                          console.log("Submitted Table Data:", data);
                          console.log("Schema name: ", schemaName);
                          let c_tbl:CreateTableInfo = {
                              table_name: data.name,
                              columns: data.columns,
                              db_name: databaseName,
                              schema_name: schemaName 
                          }
                          saveTableData({create_info:c_tbl,database_type:currentDatabaseType});
                          console.log("Submitted Table Data:", data);
                          break;
                        case "column":
                          console.log("Submitted Column Data:", data);
                          break;
                        case "view":
                          let view:View = {
                            name: data.view_name || "asd",
                            definition: editorState || "",
                            type: "view",
                            type_: "view",
                            schema_name: data.schema_name
                          }
                          console.log("Submitted View Data:", view);
                          saveViewData({database_type:currentDatabaseType,view});
                          break;
                        case "procedure":
                          console.log("Submitted Procedure Data:", data);
                          break;
                        case "function":
                          console.log("Submitted function definition: ", editorState);
                          let func:Function = {
                            name: data.function_name,
                            definition: data.definition || "",
                            type: "function",
                            db_name: "dvdrental",
                            parameters: data.parameters || [],
                            schema_name: data.schema_name,
                            return_type: "",
                            full_function: data.full_function,
                          };
                          saveFunctionData({create_info:func, database_type:currentDatabaseType, table_name:data.table_name});
                          break;
                        case "sequence":
                          console.log("Sequence create data: ", data);
                          let seq:CreateSequenceInfo = {
                              sequence_name: data.name,
                              start_val: data.start,
                              minimum_val: data.minimum,
                              maximum_val: data.maximum,
                              increment: data.increment,
                              cycle: false
                          }
                          saveSequenceData({
                            database_type: currentDatabaseType, create_info: seq, table_name: "nemmi", schema_name: data.schema_name,
                            database_name: ""
                          });
                          break;
                        case "role":
                          let role:Role = {
                              name: data.name,
                              type: "role",
                              is_super : data.is_super || false,
                              is_insherit : data.is_insherit || false,
                              can_login : data.can_login || false,
                              is_replication : data.is_replication || false,    
                              is_create_role : data.is_create_role || false,
                              is_create_db : data.is_create_db || false,
                              db_name: data.db_name,
                              connection_limit: data.connection_limit || 999,
                              password: data.password || "",
                              valid_until: data.valid_until || "",
                              schema_name:data.schema_name,
                              type_: "role"
                          }
                          console.log("Submitted Role Data:", role);
                          saveRoleData({role:role,database_type:currentDatabaseType});
                          if(!isPending && roleData){
                            console.log("Not pending and Roledata: ", roleData);
                            databaseMetadata["dvdrental"].schemas["public"].roles[roleData.name] = roleData
                          }
                         /*if(!isPending && roleData){
                            console.log("Roledata: ", roleData);
                            console.log("Rolédata: ", roleData);
                            console.log("DatabaseMetadata: ", databaseMetadata["dvdrental"].schemas["public"].roles[roleData.name] = roleData);
                            console.log("newMetadata: ",databaseMetadata["dvdrental"].schemas["public"].roles);
                          }*/
                          /*if(isSuccessRole){
                              if(roleData){
                                console.log("Rolédata: ", roleData);
                                console.log("DatabaseMetadata: ", databaseMetadata["dvdrental"].schemas["public"].roles[roleData.name] = roleData);
                                console.log("newMetadata: ",databaseMetadata["dvdrental"].schemas["public"].roles);
                              }
                          }*/
                        //  databaseMetadata[role.db_name].roles[role.name] = role;
                          break;
                        case "index":
                          let index:Index = {
                            name: data.index_name,
                            type: "index",
                            definition: data.definition || "",
                            column_name: data.column_name || [],
                            non_unique: data.non_unique || false,
                            table_name: data.table_name || "",
                            type_: "index",
                            schema_name:data.schema_name,
                            db_name: ""
                          }
                          console.log("Submitted Index Data:", index);
                          saveIndexData({database_type:currentDatabaseType,index});
                          break;
                        case "trigger":
                          /*let trigger:Trigger = {
                              name: data.trigger_name,
                              definition: data.definition || "",
                              type: "trigger",
                              table_name: data.table_name || "",
                              type_: "trigger"
                          }*/
                          console.log("Submitted Trigger Data:", data);
                          let name = data.name;
                          let when = data.when;
                          let type_ = data.type;
                          let table_name = data.table_name;
                          let function_name = data.function_name;
                         // let _schema_name = data.schema_name;
                          saveTriggerData({database_type:currentDatabaseType,name:name,when:when,type:type_,table_name:table_name,function_name:function_name/*,schema_name:schema_name*/});
                          break;
                        case "constraint":
                          console.log("Submitted Constraint Data:", data);
                          let constraint:Constraint = {
                            name: data.name,
                            type: "constraint",
                            // definition: data.definition || "",
                            table_name: data.table_name || "",
                            type_: "constraint",
                            column_name: "",
                            c_type: data.c_type,
                            schema_name:data.schema_name,
                            db_name: databaseName
                          }
                          console.log("Submitted Constraint Data:", constraint);
                          saveConstraintData({database_type:currentDatabaseType,constraint});
                          break;
                        default:
                          console.log("Submitted Data:", data);
                      }
                    };
              
                  
                    return (
                      <div className="max-w-md mx-auto">
                        <h2 className="text-xl font-bold mb-4">Create {type}</h2>
                        <GenericForm schema={formSchemas[type]} onSubmit={handleSubmit} dynamicOptions={dataTypeOptions} type_={type} isCreate={true} databaseName={databaseName} schemaName={schemaName}/>
                      </div>
                    );
              }
              
              