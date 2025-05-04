import { invoke } from "@tauri-apps/api/core";
import { Database } from "../../Types/DatabaseTypes";
import { message } from "@tauri-apps/plugin-dialog";
import { useConnection } from "../../providers/ConnectionProvider";


export const getMetadatas = async (): Promise<Database[]> => {
    const {currentDatabaseType} = useConnection();
    try{
        const metadatas = await invoke("get_metadatas",{databaseType:currentDatabaseType});
        return metadatas as Database[]
    }catch(error:any){
        console.error("Error fetching metadatas",error);
        message(error["E"], {title:"Error fetching metadatas", kind:'error'})
        return [];
    }
};

/*/export const useMetadatas = async (dbConnection: DatabaseConnection) => {
    const [metadatas,setMetadatas] = useState<Database[]>([]);
    
    useEffect(() => {

        const connectAndFetchMetadatas = async () => {
            await useConnectToDatabase(dbConnection);
            const metadatas = await getMetadatas();
            setMetadatas(metadatas);
        };

        /*useConnectToDatabase(dbConnection).then(
            getMetadatas().then((metadata) => {
                setMetadatas(metadata);
            })
        );*/
        /*getMetadatas()
            .then((metadata) => {
                setMetadatas(metadata);
                setIsConnected(true);
            });x
    }, []);

    return {
        metadatas,
    };
};
*/