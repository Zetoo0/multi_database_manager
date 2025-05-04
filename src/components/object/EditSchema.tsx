//import { FormField } from "../../Types/object/FormField";
import { dataTypeOptions } from "../../Types/object/DataTypeOption";
import { useEffect,  useMemo } from "react";
//import { Button } from "../ui/Button";
//import { Input } from "../ui/Input";
//import { Select } from "../ui/Select";
//import { Table as UITable } from "../ui/Table";
//import {  formSchemas } from "./FormSchema";
import { Column,Constraint,/* Constraint, Function,*/ Functions, Index, /*Index,*/  /*Role,*/ Sequence, Table, View } from "../../Types/DatabaseTypes";
//import { useExecuteCreateSequence,CreateSequenceInfo, CreateTableInfo, useExecuteCreateTableQuery, useExecuteCreateRole, useExecuteCreateFunction, useExecuteCreateView, useExecuteCreateIndex, useExecuteCreateTrigger, useExecuteCreateDatabaseQuery } from "../../hooks/query/useExecuteCreateQuery";
//import { useExecuteEditConstraint, useExecuteEditFunction, useExecuteEditIndex, useExecuteEditSequence, useExecuteEditTableColumn, useExecuteEditView } from "../../hooks/query/useExecuteEditQuery";
//import { useMetadata } from "../../providers/MetadataProvider";
//import { SqlEditor } from "../query/Editor";
import { useConnection } from "../../providers/ConnectionProvider";
import { useExecuteEditTableColumn, useExecuteEditFunction, useExecuteEditView, useExecuteEditIndex, useExecuteEditSequence, useExecuteEditConstraint, useExecuteEditTable } from "../../hooks/query/useExecuteEditQuery";
import { useMetadata } from "../../providers/MetadataProvider";
import { formSchemas } from "./FormSchema";
import { GenericForm } from "./GenericObjectForm";
import { useQueryClient } from "@tanstack/react-query";
//import { Translation, useTranslation } from "react-i18next";
//import { useFileOpenDialogCreateDatabase } from "../../hooks/system/useFileOpenDialog";
//import { useGetTablesOfDatabase } from "../../hooks/system/databaseMetadataQuery";
//import { useGetColumnsOfTable } from "../../hooks/system/databaseMetadataQuery";

export const EditEntityForm = ({type,data,databaseName,schemaName} : {type: string, data: any, databaseName: string, schemaName: string}) => {
    const dataMemo = useMemo(() => data, [data]);
    const {databaseMetadata} = useMetadata();
    const {saveTableData, /*isSuccessTable*/} = useExecuteEditTable();
    const {saveFunctionData,isSuccessFunction} = useExecuteEditFunction();
    const {saveViewData, /*isSuccessView*/} = useExecuteEditView();
   // const {saveRoleData, isSuccessRole} = useExecuteEditRole();
    const {saveIndexData, /*isSuccessIndex*/} = useExecuteEditIndex();
  //  const {saveTriggerData, isSuccessTrigger} = useExecuteEditTrigger();
    const {saveSequenceData, /*isSuccessSequence*/} = useExecuteEditSequence();
    const {saveConstraintData, /*isSuccessConstraint*/} = useExecuteEditConstraint();
    const {saveTableColumnData} = useExecuteEditTableColumn();
    const {currentDatabaseType} = useConnection();
    const queryClient = useQueryClient();
    const editorState = queryClient.getQueryData<string>(['editorState']) || "";
    const handleSubmit = (newData: Record<string, any>, oldColumn?: any) => {
      console.log("Handle submit type: ", type)
      switch (type) {
        case "table":
          console.log("Submitted Table Or Table Column Data:", newData);
          if(newData["type_"] === "column"){
            console.log("Submitted Table Table Column Data:", newData);
            handleEditColumn(newData, oldColumn);
            break;
          }
          handleEditTable(newData);
          break;
        case "column":
          handleEditColumn(newData);
          break;
        case "view":
          handleEditView(newData);
          console.log("Submitted View Data:", newData);
          break;
        case "procedure":
          console.log("Submitted Procedure Data:", newData);
          break;
        case "function":
          handleEditFunction(newData);
          console.log("Submitted Function Data:", newData);
          break;
        case "sequence":
          handleEditSequence(newData);
          console.log("Submitted Sequence Data:", newData);
          break;
        case "index":
          console.log("Submitted Index Data: ",newData);
          handleEditIndex(newData);
          break;
        case "constraint":
          handleEditConstraint(newData);
          break;
        default:
          console.log("Submitted Data:", newData);
      }    
    };

    const handleEditConstraint = (newData:Record<string,any>) => {
      console.log("Edit constraint: ", newData);
      console.log("Edit constraint old: ", dataMemo.editObject);
      const oldConstraint = newData;
      const newConstraint:Constraint = dataMemo.editObject;
      const db_name = dataMemo.editObject.db_name;
      saveConstraintData({db_name:db_name, old_constraint:oldConstraint as Constraint,new_constraint:newConstraint,table_name:oldConstraint.table_name,database_type:currentDatabaseType});
    }

    const handleEditIndex = (newData:Record<string,any>) => {
      console.log("edit index: ", newData);
      console.log("edit old_data: ", dataMemo.editObject);
      let newIndex = newData;
      let oldIndex:Index = dataMemo.editObject;
      const db_name = dataMemo.editObject.db_name;
      if(db_name){
        saveIndexData({db_name:db_name,new_index:newIndex as Index,old_index:oldIndex,table_name:oldIndex.table_name,database_type:currentDatabaseType});
      }
    }

    const handleEditView = (newData: Record<string,any>) => {
      console.log("edit view: ",newData);
      let newView = newData;
      let oldView:View = dataMemo.editObject;
      const db_name = dataMemo.editObject.db_name;
      if(db_name){
        saveViewData({db_name:db_name,old_view:oldView,new_view:newView as View,database_type:currentDatabaseType});
      } 
    }

    const handleEditSequence = (newData: Record<string,any>) => {
      console.log("edit seq: ",newData);
      let newSeq = newData;
      let oldSeq: Sequence = dataMemo.editObject;
      const db_name = dataMemo.editObject.db_name;
      if(db_name){
        saveSequenceData({db_name:db_name,old_sequence:oldSeq,new_sequence:newSeq as Sequence,database_type:currentDatabaseType});
      }
    }

    const handleEditTable = (newData: Record<string, any>) => {
      let new_cols = newData;
        let old_cols: Column = dataMemo.editObject
        console.log("Submitted Data converting to columns:", new_cols);
        console.log("Old data: ",old_cols);
       // console.log("Parent database name: ",Object.values(databaseMetadata).map((db) => db).filter);
      const db_name = dataMemo.editObject.db_name;
      /* const db_name = Object.values(databaseMetadata).find((db) => {
        const found_table = Object.values(db.schemas["public"].tables).find((table) => {
            if(table.name === dataMemo.editObject.name){
                console.log("Found table: ",table);
                console.log("Table database name: ",db.name);
                return true;
            }
          })
          return found_table ? db : undefined;
        });*/
        console.log("Editing table database name: ", db_name.name);
        saveTableData({
          db_name: db_name.name, database_type: currentDatabaseType,
          old_table: dataMemo.editObject as Table,
          new_table: newData as Table
        });
      //  saveTableData({new_cols:new_cols as Column,old_cols:old_cols,db_name:db_name,table_name:newData.table_name,database_type:currentDatabaseType});
        //saveTableData({new_cols,old_cols,db_name:db_name.name,table_name:dataMemo.editObject.name});
    }

    const handleEditColumn = (newData:Record<string,any>, oldColumn?: any) => {
      console.log("Edit column: ",newData);
      let new_col = newData;
      let old_col:Column = oldColumn as Column;
      console.log("Old column: ",old_col);
      const db_name = dataMemo.editObject.db_name;
      saveTableColumnData({db_name:db_name,old_cols:old_col,new_cols:new_col as Column,table_name:new_col.table_name,database_type:currentDatabaseType});
    }

    const handleEditFunction = (newData:Record<string,any>) => {
      console.log("Edit func: ",newData);
      let new_function = newData;
      const db_name = dataMemo.editObject.db_name;
      /*const db_name = Object.values(databaseMetadata).find((db) => {
        const found_table = Object.values(db.schemas["public"].functions).find((table) => {
            if(table.name === dataMemo.editObject.name){
                console.log("Found function: ",table);
                console.log("Table database name: ",db.name);
                return true;
            }
          })
          return found_table ? db : undefined;
      });*/
      if(db_name){
        console.log("Editing table database name: ", db_name.name);
        console.log("EDiting function code: ", editorState);
        saveFunctionData({db_name:db_name,old_function:dataMemo.editObject,new_function:new_function as Functions,database_type:currentDatabaseType});
        if(isSuccessFunction){
            console.log("Successfull edited table: ",dataMemo.editObject.name);
            databaseMetadata[db_name?.name].schemas[dataMemo.editObject.schema_name].functions[dataMemo.editObject.name] = new_function as Functions;
           // console.log("New columns: ",databaseMetadata[db_name?.name].schemas["public"].tables[dataMemo.editObject.name].columns);  
        }else{
          console.log("Succession: ",isSuccessFunction);
        }
      }    }
      


    useEffect(() => {
        console.log("Editing data: ",data.editObject); 
        console.log("Formschema tyoe: ",type);
      }, [data]);
    
    return (
        <div className="max-w-md mx-auto">
          <div>
            <div>
              <h2 className="text-xl font-bold mb-4">Edit {type}</h2>
              <GenericForm schema={formSchemas[type]} onSubmit={handleSubmit} data={dataMemo.editObject} dynamicOptions={dataTypeOptions} type_={type} isCreate={false} databaseName={databaseName} schemaName={schemaName}/>
            </div>
          </div>          
        </div>
      );
}
