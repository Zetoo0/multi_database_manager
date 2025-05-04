import React, { createContext, useContext, useEffect, useState, /*useEffect, useState*/ } from "react";
import { Database} from "../Types/DatabaseTypes";
import { useDatabaseMetadata } from "../hooks/system/databaseMetadataQuery";
import { useConnection } from "./ConnectionProvider";
import {isEqual} from 'lodash';
//import { use } from "i18next";
type MetadataContextType = {
  databaseMetadata: Record<string, Database>;//Database[];
  setDatabaseMetadata: React.Dispatch<React.SetStateAction<Record<string, Database>>>;
  refetchDatabaseMetadata:() => void;
};

const MetadataContext = createContext<MetadataContextType | undefined>(undefined);

interface MetadataProviderProps {
  children: React.ReactNode;
}


export const MetadataProvider = ({ children }: MetadataProviderProps) => {
  //const [metadatas, setMetadatas] = useState<Database[]>([]);
  const {isConnected,currentDatabaseType} = useConnection();
  const {databaseMetadata:fetchDatabaseMetadata} = useDatabaseMetadata(isConnected,currentDatabaseType);
  const [databaseMetadata,setDatabaseMetadata] = useState<Record<string, Database>>({});
  //const [databasesTableNames, setDatabasesTableNames] = useState<Record<string, string[]>>({});

 /* const tableNamesForDatabase = () => {
    Object.keys(databaseMetadata).map((db) => {
        Object.keys(databaseMetadata[db].schemas["public"].tables).map((table) => {
            const tableNames:string[] = [];
            tableNames.push(databaseMetadata[db].schemas["public"].tables[table].name);
            setDatabasesTableNames((prev) => ({
                ...prev,
                [db]: tableNames
            }))
        })
    });
}


  useEffect(() => {
    tableNamesForDatabase();
  },[databaseMetadata]);

export const refetchMetadatas = () => {
	delete databasseMetadata;
	setDatabaseMetada(fetchDaDatabaseMetadata);	
}*/


  useEffect(() => {
    if(!isEqual(databaseMetadata,fetchDatabaseMetadata)){
      console.log("It is not the same: ", databaseMetadata, fetchDatabaseMetadata)
      setDatabaseMetadata(fetchDatabaseMetadata);
      
    }
  },[fetchDatabaseMetadata,databaseMetadata]);

  const refetchDatabaseMetadata = () => {
    setDatabaseMetadata(fetchDatabaseMetadata);
  }

  return (
    <MetadataContext.Provider
      value={{
        databaseMetadata, 
        setDatabaseMetadata,   
      refetchDatabaseMetadata
      ////  getTable,
       // addTableToDatabase,
      //  addColumnToTable,
      }}
    >
      {children}
    </MetadataContext.Provider>
  );
};

export const useMetadata = ()  => {
  const context = useContext(MetadataContext);
  if (!context) {
    throw new Error("useMetadata must be used within a MetadataProvider");
  }
  return context;
};
