import { invoke } from "@tauri-apps/api/core";
import { QueryResult } from '../../Types/query/QueryResult';
import { QueryInfo } from '../../Types/query/QueryInfo';
import { message } from "@tauri-apps/plugin-dialog";


export const executeQuery = async (query: QueryInfo):Promise<QueryResult> => {
    try{
        return await invoke("query", { queryInfo : query });    
    }catch(err:any){
        message(err["E"], {title:"Error execute query", kind:'error'})
        return Promise.reject(err);
    }
};


/*const useExecuteQuery = (query: QueryInfo) => {
    const{
        status,
        data:queryData,
        error
    }
    = useQuery({
        queryKey: ["queryData"],
        queryFn: () => executeQuery(query),
        enabled: true,
        staleTime:1000*60*60, //1000 * 60 * 60,
        refetchOnWindowFocus: false
    });
    return {status,queryData,error};
};*/