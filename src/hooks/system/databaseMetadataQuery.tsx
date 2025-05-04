import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { Database } from "../../Types/DatabaseTypes";
import { useMetadata } from "../../providers/MetadataProvider";
    


const fetchDatabaseMetadata = async (databaseType:string) => {
    try{
        console.log("Current Database type:");
        const metadatas = await invoke("get_metadatas",{databaseType:databaseType});
        console.log("Elv good, metadatas: ",metadatas);
        return metadatas as Database[]
    }catch(e){
        console.error("Error fetching metadatas",e);
        return [];
    }
}


export const useDatabaseMetadata = (enabled: boolean, databaseType:string) => {
    const {
        status,
        data:databaseMetadata,
        error,
    } = useQuery({
        queryKey: ["databaseMetadata",databaseType],
        queryFn: async () =>{
            const metadata = await fetchDatabaseMetadata(databaseType);
            return metadata as Database[];
        }, //fetchDatabaseMetadata(databaseType),
        enabled,
        staleTime:1000*60*60, //1000 * 60 * 60,
        refetchOnWindowFocus: false
    });
    return {status,databaseMetadata: JSON.parse(JSON.stringify(databaseMetadata??[])) as Record<string,Database>, /*Database[],*/error};
    //return 
}

//export const getTableColumns()

/*export const useGetColumnsOfTable = (tableName: string, databaseName: string) => {
    const {databaseMetadata} = useMetadata();
}*/


export const useGetTablesOfDatabase = (databaseName: string, schemaName: string) => {
    const {databaseMetadata} = useMetadata();
    console.log("DAtagase jetada: ",databaseMetadata);
    console.log("Database name: ", databaseName);
    console.log("schema name: ", schemaName);
    const tableNames:string[] = Object.keys(databaseMetadata[databaseName].schemas[schemaName].tables);
    console.log("Table names: ",tableNames);
    return tableNames;
    //databaseMetadata[databaseName].schemas["public"].tables;
}

export const useGetColumnsOfTable = (tableName: string, databaseName: string,schemaName: string) => {
    const {databaseMetadata} = useMetadata();
    const table = databaseMetadata[databaseName].schemas[schemaName].tables[tableName];
    const columnNames:string[] = Object.keys(table.columns);
    console.log("Column names: ",columnNames);
    return columnNames;
    //databaseMetadata[databaseName].schemas["public"].tables;
}