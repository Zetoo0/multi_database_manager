import { useMutation } from "@tanstack/react-query";
import { Column, Constraint, Function, Index, Role, Sequence, Table, TriggerFunction, View } from "../../Types/DatabaseTypes";
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { useMetadata } from "../../providers/MetadataProvider";
export type CreateTableInfo = {
    table_name: string,
    columns: Column[],
    db_name: string,
    schema_name:string
}

export type CreateSequenceInfo = {
    sequence_name: string,
    start_val: string,
    minimum_val: string,
    maximum_val: string,
    increment: string,
    cycle: boolean
}

const createTable = async (create_info: CreateTableInfo, database_type: string) : Promise<Table|null> => {
    try{
        console.log("Creating table: ",create_info);
        console.log("create table type: ", Object.values(create_info.columns));
        create_info.table_name = create_info.table_name.toLowerCase();
        let columns = Object.values(create_info.columns);
        create_info.columns = columns;
        let res = await invoke("create_table",{tableInfo:create_info, databaseType:database_type});
        await message("Success Table Create", {title:"Table Create Success", kind:'info'})
        return res as Table;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        await message(e["E"], {title:"Delet Column Problem", kind:'error'})
        return null;
    }
}

const createSequence = async (create_info: CreateSequenceInfo, database_type: string,table_name:string,database_name:string,_schema_name:string) : Promise<Sequence|null> => {
    try{
        console.log("Create Sequence INfo: ", create_info);
        let res = await invoke("create_sequence",{createSeqInfo:create_info, databaseType:database_type,tableName:table_name,databaseName:database_name});
        await message("Success Sequence Create", {title:"Table Create Success", kind:'info'})
        return res as Sequence;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        await message(e["E"], {title:"Delet Column Problem", kind:'error'})
        return null;
    }
}

const createFunction = async (create_info: Function, database_type: string,table_name:string) : Promise<Function|null> => {
    try{
        let res = await invoke("create_function",{functionInfo:create_info, databaseType:database_type,tableName:table_name});
        await message("Success Function Create", {title:"Table Create Success", kind:'info'})
        return res as Function;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        await message(e["E"], {title:"Delet Column Problem", kind:'error'})
        return null;
    }
}

const createView = async (view:View, _database_type: string) : Promise<View|null> => {
    try{
        let res = await invoke("create_view",{view:view,dbName:"dvdrental"});
       await message("Success View Create", {title:"Table Create Success", kind:'info'})
        return res as View;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        await message(e["E"], {title:"Delet Column Problem", kind:'error'})
        return null;
    }
}

const createIndex = async (index:Index, database_type: string): Promise<Index|null> => {
    try{
        let res = await invoke("create_index",{index:index, databaseType:database_type,dbName:index.db_name});
        await message("Success Index Create", {title:"Table Create Success", kind:'info'})
        return res as Index;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        await message(e["E"], {title:"Delet Column Problem", kind:'error'})
        return null;
    }
}

const createRole = async (role:Role, database_type: string) : Promise<Role|null> => {
    try{
        let res = await invoke("create_role",{role:role, databaseType:database_type,dbName:role.db_name})
        await message("Success Role Create", {title:"Table Create Success", kind:'info'})
        return res as Role;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        await message(e["E"], {title:"Delet Column Problem", kind:'error'})
        return null;
    }
}

const createTriger = async (database_type: string, name: string,when:string,type:string,table_name:string,function_name:string) : Promise<TriggerFunction|null> => {
    try{
        console.log("Trigger info: ",{name:name,when:when,type:type,tableName:table_name,functionName:function_name, databaseType:database_type,dbName:"dvdrental"})
        let res = await invoke("create_trigger",{name:name,when:when,type:type,tableName:table_name,functionName:function_name, databaseType:database_type,databaseName:"dvdrental"});
        await message("Success Trigger Create", {title:"Table Create Success", kind:'info'})
        return res as TriggerFunction;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        await message(e["E"], {title:"Delete Column Problem", kind:'error'})
        return null;
    }
}

const createDatabase = async (file_content:string, _database_name:string,database_type:string): Promise<string|null> => {
    try{
            /*
                database_info: String,
                db_type: String,
                file: String,
            */
        let res = await invoke("create_database",{file:file_content,databaseInfo:"a",dbType:database_type});
        await message("Success Database Create", {title:"Database Create Success", kind:'info'})
        return res as string;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        await message(e["E"], {title:"Create Database Error", kind:'error'})
        return null;
    }
}

const createConstraint = async (constraint:Constraint, database_type: string): Promise<Constraint|null> => {
    try{
        console.log("Constraint info: ",{constraint:constraint, databaseType:database_type,dbName:"dvdrental"})
        let res = await invoke("create_constraint",{constraint:constraint, databaseType:database_type,dbName:"dvdrental"});
        await message("Success Constraint Create", {title:"Table Create Success", kind:'info'})
        return res as Constraint;
    }catch(e:any){
        console.error("Error fetching metadatas",e);
        await message(e["E"], {title:"Create Error", kind:'error'})
        return null;
    }
}

export const useExecuteCreateDatabaseQuery = () => {
    const {mutate, data} = useMutation({
        mutationFn:({ file_content, database_name, database_type }: {file_content:string , database_name: string, database_type:string }) => createDatabase(file_content, database_name,database_type),
    })
    return {saveDatabaseData: mutate, databaseData: data};
}

export const useExecuteCreateTableQuery = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate, data} = useMutation({        
        mutationFn:({create_info,database_type}: { create_info: CreateTableInfo, database_type: string }) => createTable(create_info,database_type),
        onSuccess: (data:Table|null) => {
            if(data == null) return;
            databaseMetadata[data.db_name].schemas[data.schema_name].tables[data.name] = data;
        }
    });
    return {saveTableData: mutate, tableData: data};
}

export const useExecuteCreateSequence = () => {
    const {databaseMetadata,/*refetchDatabaseMetadata, setDatabaseMetadatas*/} = useMetadata();
    const {mutate,data} = useMutation({
        mutationFn:({ create_info, database_type, table_name,database_name,schema_name }: { create_info: CreateSequenceInfo, database_type: string, table_name: string, database_name:string,schema_name:string }) => createSequence(create_info, database_type, table_name,database_name,schema_name),
        onSuccess: (data:Sequence|null) => {
            console.log("Returned szekvenc: ", data);
            if(data == null) return;
            console.log("Not null");
           /* setDatabaseMetadata((prev) => ({
                ...prev,
                ["dvdrental"]: {
                    ...prev["dvdrental"],
                    schemas: {
                        ...prev["dvdrental"].schemas,
                        ["public"]: {
                            ...prev["dvdrental"].schemas["public"],
                            sequences: {
                                ...prev["dvdrental"].schemas["public"].sequences,
                                [data.name]: data
                            }
                        }
                    }
                }
            }))*/
            console.log("setDatabaseMetadata: ", databaseMetadata);
            databaseMetadata["dvdrental"].schemas["public"].sequences[data.name] = data;
            //refetchDatabaseMetadata();
        }
    })
    return {saveSequenceData: mutate, sequenceData: data};
}

export const useExecuteCreateFunction = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate,data} = useMutation({
        mutationFn:({ create_info, database_type, table_name }: { create_info: Function, database_type: string, table_name: string }) => createFunction(create_info, database_type, table_name),
        onSuccess: (data:Function|null) => {
            if(data == null) return;
            databaseMetadata[data.db_name].schemas[data.schema_name].functions[data.name] = {...data, children:[]};
        }
    })
    return {saveFunctionData: mutate, functionData: data};
}

export const useExecuteCreateView = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate,data} = useMutation({
        mutationFn:({ view, database_type }: { view: View, database_type: string }) => createView(view, database_type),
        onSuccess: (data:View|null) => {
            if(data == null) return;
            //TODO
            databaseMetadata[data.name].schemas[/*data.schema_name*/"public"].views[data.name] = data;
        }
    })
    return {saveViewData: mutate, viewData: data};
}

export const useExecuteCreateIndex = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate,data} = useMutation({
        mutationFn:({ index, database_type }: { index: Index, database_type: string }) => createIndex(index, database_type),
        onSuccess: (data:Index|null) => {
            if(data == null) return;
            databaseMetadata[data.db_name].schemas[data.schema_name].tables[data.table_name].indexes[data.name] = data;
        }
    })
    return {saveIndexData: mutate, indexData: data};
}

export const useExecuteCreateRole = () => {
    const {databaseMetadata/*,setDatabaseMetadata*/     } = useMetadata();
    const {mutate, data, isSuccess, isPending} = useMutation({
        mutationFn:({ role, database_type }: { role: Role, database_type: string }) => createRole(role, database_type),
        onSuccess: (data:Role|null) => {
            console.log("Role data in successSS: ", data);
            if(data == null) return;
            console.log("Role data in success: ", data);
            databaseMetadata[data.db_name].schemas["public"].roles[data.name] = data;
           
            console.log("After setDatabaseMetadata: ", databaseMetadata);
            //databaseMetadata[data.db_name].schemas[data.schema_name].roles[data.name] = data;
        }
    })
    return {saveRoleData: mutate, roleData: data, isSuccessRole: isSuccess,isPending};
}

export const useExecuteCreateTrigger = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate, data} = useMutation({
        mutationFn:({ database_type, name,when,type,table_name,function_name }: { database_type: string, name: string, when: string, type: string, table_name: string, function_name: string }) => createTriger(database_type, name,when,type,table_name,function_name),
        onSuccess: (data:TriggerFunction|null) => {
            if(data == null) return;
            //TODO
            databaseMetadata[data.name].schemas[/*data.schema_name*/"public"].trigger_functions[data.name] = data;
        }
    })
    return {saveTriggerData: mutate, triggerData: data};
}

export const useExecuteCreateConstraint = () => {
    const {databaseMetadata} = useMetadata();
    const {mutate,data} = useMutation({
        mutationFn:({ constraint, database_type }: { constraint: Constraint, database_type: string }) => createConstraint(constraint, database_type),
        onSuccess: (data:Constraint|null) => {
            if(data == null) return;
            databaseMetadata[data.db_name].schemas[data.schema_name].tables[data.table_name].constraints[data.name] = data;
        }
    })
    return {saveConstraintData: mutate, constraintData: data};
}