import { useMutation } from "@tanstack/react-query";
import { getConnectionStore } from "../../store/Store";
import { DatabaseConnection } from "./useConnectToDatabase";

const saveConnection = async (connectionInfo: DatabaseConnection) => {
    try{
        const store = await getConnectionStore();
        const currentConnections = (await store.get('connections')) || [];
        let connectionArr = [];
        console.log("Current connections: ",currentConnections);
        console.log("Type of current connections: ",typeof currentConnections);
        if(typeof currentConnections === 'object'){
            console.log("Idáig?");
            connectionArr = Object.values(currentConnections);
            const updatedConnections = [...connectionArr, connectionInfo];
            let data = await store.set('connections', updatedConnections);
            await store.save();
            return data
        }

    }catch(e){
        console.error(e);
    }}

export const useConnectionInfoStorageSave = () => {
    const {mutate} = useMutation({        
        mutationFn:(connectionInfo: DatabaseConnection) => saveConnection(connectionInfo),
    });
    return {saveData: mutate};
}