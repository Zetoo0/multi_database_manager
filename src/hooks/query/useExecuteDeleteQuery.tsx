

import {useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { useMetadata } from "../../providers/MetadataProvider";
import { Column } from "../../Types/DatabaseTypes";

/*
type EditTableColumn = {
    new_cols:Column,
    old_cols:Column
}*/

const deleteTableColumn = async (column_name:string,db_name:string,table_name:string):Promise<Column|null> => {
    try{
        //console.log("Edit table column info: new: ",Object.values(new_cols));
        await invoke("delete_table_column",{columnName:column_name,dbName:db_name,tableName:table_name});
        let retCol:Column = {
            name: column_name,
           // type: "column",
            data_type: "",
            is_nullable: false,
            default_value: "",
            is_primary_key: false,
            table_name: table_name,
            db_name: db_name,
            type_: "column",
            schema_name: ""//TODO
        }
        return retCol;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        await message(e["E"], {title:"Delet Column Problem", kind:'error'})
        return null;
    }
}

const deleteTable = async (db_name:string,table_name:string) => {
    try{
        await invoke("delete_table",{dbName:db_name,tableName:table_name});
        return "Success";
    }catch(e){
        console.error("Error fetching metadatas",e);
        return [];
    }
}

const deleteSequence = async (db_name:string,sequence_name:string) => {
    try{
      //  console.log("Edit sequence info: new: ",Object.values(new_sequence));
        await invoke("delete_sequence",{sequenceName:sequence_name,dbName:db_name});
        return "Success";
    }catch(e){
        console.error("Error fetching metadatas",e);
        return [];
    }
}

const deleteFunction = async (db_name:string,function_name:string,_database_type:string) => {
    try{
       // console.log("Edit function info: new: ",Object.values(new_function));
        await invoke("edit_function",{functionName:function_name,dbName:db_name});
        return "Success";
    }catch(e){
        console.error("Error fetching metadatas",e);
        return [];
    }
}

const deleteBaseObject = async (db_name:string,delete_to_name:string,object_name:string,database_type:string) => {
    try{
        console.log("Trying toDelete base object: ",db_name,delete_to_name,object_name);
        let res = await invoke("base_delete", {dbName:db_name,deleteToName:delete_to_name,objectName:object_name,databaseType:database_type})
        console.log("Result: ",res);
        await message("Delete Success", {title:"Delete Success", kind:'info'})
        return "Success";
    }catch(e:any){
        await message(e["E"], {title:"Delet Proble", kind:'error'})
        //console.error("Error while deleting: ",e["E"]);
       // message("Error while deleting", {title:"Delet Proble", kind:'error'})
        return [];
    }
}


export const useExecuteDeleteBaseObject = () => {
     //   const {databaseMetadata} = useMetadata();
    console.log("Trying to delete base object");
    const {mutate,isSuccess} = useMutation({
        mutationFn:({db_name, delete_to_name,object_name,database_type } : {db_name:string,delete_to_name:string,object_name:string,database_type:string}) => deleteBaseObject(db_name,delete_to_name,object_name,database_type),
        onError:(_error) => {
            console.error("Deleting problem");
            message("Error while deleting", {title:"Delet Problem", kind:'error'})
        }
    });
    return {saveBaseObjectData:mutate, isSuccessBase: isSuccess};
}

export const useExecuteDeleteTableColumn = () => {
        const {databaseMetadata} = useMetadata();
    const {mutate, isSuccess} = useMutation({        
        mutationFn:({column_name,db_name,table_name} : {column_name:string,db_name:string,table_name:string}) => deleteTableColumn(column_name,db_name,table_name),
        onSuccess:(data:Column|null) => {
            if(data == null) return;
            delete databaseMetadata[data.db_name].schemas[data.schema_name].tables[data.table_name].columns[data.name];
        }
    });
    return {saveTableData: mutate, isSuccessTable: isSuccess};
}

export const useExecuteDeleteTable = () => {
    //TODO
    //const {databaseMetadata} = useMetadata();

    const {mutate, isSuccess} = useMutation({        
        mutationFn:({db_name,table_name} : {db_name:string,table_name:string}) => deleteTable(db_name,table_name),
        onSuccess:(_data) => {
            //delete databaseMetadata[_data.].schemas[data.schema_name].tables[data.name];            
        }
    });
    return {saveTableData: mutate, isSuccessTable: isSuccess};
}

export const useExecuteDeleteSequence = () => {
    //const {_databaseMetadata} = useMetadata();
    const {mutate, isSuccess} = useMutation({        
        mutationFn:({db_name,sequence_name} : {db_name:string,sequence_name:string}) => deleteSequence(db_name,sequence_name),
        onSuccess:(_data) => {
           // databaseMetadata["dvdrental"].schemas[data.schema_name].sequences[data.name] = [];
        }
    });
    return {saveSequenceData: mutate,isSuccessSequence: isSuccess};
}

export const useExecuteDeleteFunction = () => {
    //const {databaseMetadata} = useMetadata();
    const {mutate, isSuccess} = useMutation({        
        mutationFn:({db_name,function_name,database_type} : {db_name:string,function_name:string,database_type:string}) => deleteFunction(db_name,function_name,database_type),
        onSuccess:(_data) => {
           // databaseMetadata["dvdrental"].schemas[data.schema_name].functions[data.name] = [];
        }
    });
    return {saveFunctionData: mutate, isSuccessFunction: isSuccess};
}

