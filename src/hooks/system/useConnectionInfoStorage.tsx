import { DatabaseConnection } from "./useConnectToDatabase";
import { getConnectionStore } from "../../store/Store";
import { useQuery } from "@tanstack/react-query";

/*export const setupConnectionInfoStorage = async (db_con:DatabaseConnection) => {
    const store = await load('connectionInfos.json');
    console.log("Storage: ",store.values());
    await store.get('connections').then((value) => {
        console.log("Connections: ",value);
    })

    store.close();
}*/

const fetchConnectionInfoList = async () => {
    const store = await getConnectionStore();
    const data = await store.get('connections');
    console.log("Fetch connection data: ",data);
    return data as DatabaseConnection[] || [];
}


export const useConnectionInfoStorage =  () => {
    const {
        status,
        data:connectionInfoList,
        error,
    } = useQuery({
        queryKey: ["connectionInfoList"],
        queryFn: fetchConnectionInfoList,
        staleTime:1000*60*60, //1000 * 60 * 60,
        refetchOnWindowFocus: false
    });
    return {status,connectionInfoList: connectionInfoList as DatabaseConnection[],error};
}