import { invoke } from "@tauri-apps/api/core";

export type DatabaseConnection = {
    port:String,
    server:String,
    username:String,
    password:String,
    driver_type:String
}


export const connectToDatabase = async (connection_info: DatabaseConnection) => {
    await invoke("connect",{dbConnection:connection_info });
    console.log("Successfully connected to database");
};

/*export const useConnectToDatabase = async (connection_info: DatabaseConnection) => {
    const [isConnected, setIsConnected] = useState(false);

    await connectToDatabase(connection_info).then(
        (result) => {
            console.log("Successfully connected to database");
            setIsConnected(true);
        },
        (error) => {
            console.error(error);
        }
    )
};*/