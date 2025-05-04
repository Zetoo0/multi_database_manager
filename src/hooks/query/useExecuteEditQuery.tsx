

import { useMutation } from "@tanstack/react-query";
import { Column,Function, Sequence, View, Index, Constraint, Table } from "../../Types/DatabaseTypes";
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { useMetadata } from "../../providers/MetadataProvider";


/*type EditTableColumn = {
    new_cols:Column,
    old_cols:Column
}*/

const editTableColumn = async (new_cols:Column,old_cols:Column,db_name:string,table_name:string,database_type:string) : Promise<Column|null> => {
    try{
        /*
    new_col: Column,
    old_col: Column,
    table_name: String,
    db_name: String,
    database_type: String,
        */
        console.log("Edit table column info: new: ",new_cols,"old: ",old_cols,"table_name: ",table_name,"db_name: ",db_name,"database_type: ",database_type);
        await invoke("edit_table_column",{newCol:new_cols,oldCol:old_cols,tableName:old_cols.table_name,dbName:old_cols.db_name,databaseType:database_type});
        message("Success Edit Column", {title:"Edit Table Column", kind:'info'})
        return new_cols;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        message(e["E"], {title:"Edit Table Error", kind:'error'})
        return null;
    }
}

const editSequence = async (db_name:string,old_sequence:Sequence,new_sequence:Sequence,database_type:string) :Promise<Sequence|null> => {
    try{
        console.log("Edit sequence info old: ",Object.values(old_sequence));
        await invoke("edit_sequence",{newSequence:new_sequence,oldSequence:old_sequence,dbName:db_name,databaseType:database_type});
        await message("Success Edit Sequence", {title:"Edit Table Column", kind:'info'})
        return new_sequence;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        message(e["E"], {title:"Edit Table Error", kind:'error'})
        return null;
    }
}

const editFunction = async (db_name:string,old_function:Function,new_function:Function,database_type:string) : Promise<Function|null> => {
    try{
        console.log("Edit function info: new: ",Object.values(new_function));
        console.log("Edit function info: old: ",Object.values(old_function));
        console.log("datas: ",db_name,database_type);
        await invoke("edit_function",{newFunction:new_function,oldFunction:old_function,dbName:"dvdrental",databaseType:database_type});
        message("Success Edit Function", {title:"Edit Table Column", kind:'info'})
        return new_function;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        message(e["E"], {title:"Edit Table Error", kind:'error'})
        return null;
    }

}

const editConstraint = async (_db_name:string,_table_name:string,old_constraint:Constraint,new_constraint:Constraint,database_type:string) : Promise<Constraint|null> => {
    try{
        console.log("Edit constraint info: new: ",Object.values(new_constraint));
        await invoke("edit_constraint",{newConstraint:new_constraint,oldConstraint:old_constraint,dbName:old_constraint.db_name,tableName:old_constraint.table_name,databaseType:database_type});
        message("Success", {title:"Edit Table Column", kind:'info'})
        return new_constraint;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        message(e["E"], {title:"Edit Table Error", kind:'error'})
        return null;
    }
}

const editIndex = async (db_name:string,table_name:string,old_index:Index,new_index:Index,database_type:string) : Promise<Index|null> => {
    try{
        console.log("Edit index info: new: ",Object.values(new_index));
        await invoke("edit_index",{newIndex:new_index,oldIndex:old_index,dbName:db_name,tableName:table_name,databaseType:database_type});
        message("Success Edit Index", {title:"Edit Table Column", kind:'info'})
        return new_index;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        message(e["E"], {title:"Edit Table Error", kind:'error'})
        return null;
    }
}

const editView = async (db_name:string,old_view:View,new_view:View,database_type:string) : Promise<View|null> => {
    try{
        console.log("Edit view info: new: ",Object.values(new_view));
        await invoke("edit_view",{newView:new_view,oldView:old_view,dbName:db_name,databaseType:database_type});
        message("Success Edit View", {title:"Edit Table Column", kind:'info'})
        return new_view;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        message(e["E"], {title:"Edit Table Error", kind:'error'})
        return null;
    }
}

const editTable = async ( db_name:string,old_table:Table,new_table:Table,database_type:string): Promise<Table|null> => {
    try{
        console.log("Edit table info: new: ",Object.values(new_table));
        await invoke("edit_table",{newTable:new_table,oldTable:old_table,dbName:db_name,databaseType:database_type});
        message("Success Edit Table", {title:"Edit Table Column", kind:'info'})
        return new_table;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        message(e["E"], {title:"Edit Table Error", kind:'error'})
        return null;
    }
}


export const useExecuteEditTableColumn = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate, isSuccess} = useMutation({        
        mutationFn:({new_cols,old_cols,db_name,table_name,database_type} : {new_cols:Column,old_cols:Column,db_name:string,table_name:string,database_type:string}) => editTableColumn(new_cols,old_cols,db_name,table_name,database_type),
        onSuccess: (data:Column|null) => {
            if(data == null) return;
            console.log("data: ",data);
            databaseMetadata[data.db_name].schemas[data.schema_name].tables[data.table_name].columns[data.name] = data;
        }
    });
    return {saveTableColumnData: mutate, isSuccessTable: isSuccess};
}

export const useExecuteEditTable = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate, isSuccess} = useMutation({
        mutationFn:({db_name,old_table,new_table,database_type} : {db_name:string,old_table:Table,new_table:Table,database_type:string}) => editTable(db_name,old_table,new_table,database_type),
        onSuccess: (data:Table|null) => {
            if(data == null) return;
            databaseMetadata[data.db_name].schemas[data.schema_name].tables[data.name] = data;
        }
    });
    return {saveTableData: mutate, isSuccessTable: isSuccess};
}

export const useExecuteEditSequence = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate, isSuccess} = useMutation({        
        mutationFn:({db_name,old_sequence,new_sequence,database_type} : {db_name:string,old_sequence:Sequence,new_sequence:Sequence,database_type:string}) => editSequence(db_name,old_sequence,new_sequence,database_type),
        onSuccess: (data:Sequence|null) => {
            if(data == null) return;
            databaseMetadata[data.db_name].schemas[data.schema_name].sequences[data.name] = data;
        }    
    });
    return {saveSequenceData: mutate,isSuccessSequence: isSuccess};
}

export const useExecuteEditFunction = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate, isSuccess} = useMutation({        
        mutationFn:({db_name,old_function,new_function,database_type} : {db_name:string,old_function:Function,new_function:Function,database_type:string}) => editFunction(db_name,old_function,new_function,database_type),
        onSuccess: (data:Function|null) => {
            if(data == null) return;
            databaseMetadata[data.db_name].schemas[data.schema_name].functions[data.name] = {...data, children:[]};
        }    
    });
    return {saveFunctionData: mutate, isSuccessFunction: isSuccess};
}

export const useExecuteEditConstraint = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate, isSuccess} = useMutation({
        mutationFn:({db_name,table_name,old_constraint,new_constraint,database_type} : {db_name:string,table_name:string,old_constraint:Constraint,new_constraint:Constraint,database_type:string}) => editConstraint(db_name,table_name,old_constraint,new_constraint,database_type),
        onSuccess: (data:Constraint|null) => {
            if(data == null) return;
            databaseMetadata[data.db_name].schemas[data.schema_name].tables[data.table_name].constraints[data.name] = data;
        }
    });
    return {saveConstraintData: mutate, isSuccessConstraint: isSuccess};
}

export const useExecuteEditIndex = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate, isSuccess} = useMutation({
        mutationFn:({db_name,table_name,old_index,new_index,database_type} : {db_name:string,table_name:string,old_index:Index,new_index:Index,database_type:string}) => editIndex(db_name,table_name,old_index,new_index,database_type),
        onSuccess: (data:Index | null) => {
            if(data == null) return;
            databaseMetadata[data.db_name].schemas[data.schema_name].tables[data.table_name].indexes[data.name] = data;
            
        }
    });
    return {saveIndexData: mutate, isSuccessIndex: isSuccess};
}

export const useExecuteEditView = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate, isSuccess} = useMutation({
        mutationFn:({db_name,old_view,new_view,database_type} : {db_name:string,old_view:View,new_view:View,database_type:string}) => editView(db_name,old_view,new_view,database_type),
        onSuccess: (data:View|null) => {
            if(data == null) return;
            //TODO
            databaseMetadata[data.name].schemas[/*data.schema_name*/"public"].views[data.name] = data;
        }
    });
    return {saveViewData: mutate, isSuccessView: isSuccess};
} 