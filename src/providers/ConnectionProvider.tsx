import React, { createContext, useState, useContext } from "react";
import { DatabaseConnection, connectToDatabase } from "../hooks/system/useConnectToDatabase";
import { useDatabaseMetadata } from "../hooks/system/databaseMetadataQuery";

type ConnectionProviderType = {
  //  connections:Record<string,Record<string,DatabaseConnection>>;
    selectedConnection: string | null;
    setSelectedConnection: (connection: string | null) => void;
    connections: DatabaseConnection[];
    isConnected:boolean;
    connectedDatabaseNames:string[];
    handleConnection: (connection_info: any) => void;  
    currentDatabaseType:string;
};

const ConnectionContext = createContext<ConnectionProviderType | undefined>(undefined);

interface ConnectionProviderProps {
    children: React.ReactNode;
}


export const ConnectionProvider = ({ children }: ConnectionProviderProps) => {
    //const [connections, setConnections] = useState<Record<string,Record<string,DatabaseConnection>>>({});
    const [selectedConnection, setSelectedConnection] = useState<string | null>(null);
    const [connections, _setConnections] = useState<DatabaseConnection[]>([]);
    const [isConnected, setIsConnected] = useState(false);
    const [currentDatabaseType,setCurrentDatabaseType] = useState<string>("")
    const [connectedDatabaseNames, _setConnectedDatabaseNames] = useState<string[]>([]);
   // const [databasesTableNames, setDatabasesTableNames] = useState<Record<string, string[]>>({});
     const handleConnection = async (connection_info: DatabaseConnection) => {
            await connectToDatabase(connection_info);
            setIsConnected(true);
            setCurrentDatabaseType(connection_info.driver_type.toString())
            console.log("Current database type:", currentDatabaseType)
            const data = useDatabaseMetadata(isConnected,connection_info.driver_type.toString());
            console.log("Database metadataAAAA: ",data);
            console.log("Current Database type: ",currentDatabaseType);
           /* const fetchedMetadatas = await fetchMetadatas().then((metadatas) => {
                console.log("Metadatas:", metadatas);
                setMetadatas(metadatas);
                setIsConnected(true);
            })*/
    }

    return (
        <ConnectionContext.Provider
          value={{
            connections,
            selectedConnection,
            setSelectedConnection,
            isConnected,
            connectedDatabaseNames,
            handleConnection,
            currentDatabaseType
          //  getTable,
           // addTableToDatabase,
          //  addColumnToTable,
          }}
        >
          {children}
        </ConnectionContext.Provider>
      );
}

export const useConnection = ()  => {
    const context = useContext(ConnectionContext);
    if (!context) {
      throw new Error("useConnection must be used within a ConnectionProvider");
    }
    return context;
};