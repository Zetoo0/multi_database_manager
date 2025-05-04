//such as add table to database, add column, etc stuffs
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import { QueryResult } from "../Types/query/QueryResult";
import { QueryInfo } from "../Types/query/QueryInfo";
import { executeQuery } from "../hooks/query/useExecuteQuery";


type QueryProviderType = {
    /*addTableToDatabase: (databaseName: string, tableName: string) => void;
    addColumnToTable: (databaseName: string, tableName: string, columnName: string, dataType: string) => void;
    addIndexToTable: (databaseName: string, tableName: string, indexName: string, columnName: string) => void;
    addConstraintToTable: (databaseName: string, tableName: string, constraintName: string, columnName: string) => void;
    addTriggerToTable: (databaseName: string, tableName: string, triggerName: string, triggerType: string) => void;*/
    queryResult: QueryResult | null;
    queryResultHeaders: string[] | null;
    handleQuery: (queryInfo:QueryInfo) => void;
}

const QueryProvider = createContext<QueryProviderType | undefined>(undefined);

export const useQuery = () => {
  const context = useContext(QueryProvider);
  if (!context) {
    throw new Error("useQuery must be used within a QueryProvider");
  }
  return context;
};

interface QueryProviderProps {
    children: ReactNode;
  }

export const QueryProviderContext = ({ children }: QueryProviderProps) => {
    const [queryResult, setQueryResult] = useState<QueryResult | null>(null);
    const [queryResultHeaders, setQueryResultHeaders] = useState<string[] | null>(null);

    const handleQuery = async (queryInfo:QueryInfo) => {
      executeQuery(queryInfo).then((result:QueryResult) => {
          console.log("Query result: ",result);
          setQueryResultHeaders(result?.rows[0] ? Object.keys(result?.rows[0]) : []);   
          setQueryResult(result);
      });
    }

    useEffect(() => {
      console.log("Query result effect: ",queryResult);
    },[queryResult,queryResultHeaders])
    return(
        <QueryProvider.Provider value={{
            /*addTableToDatabase: (databaseName: string, tableName: string) => {},
            addColumnToTable: (databaseName: string, tableName: string, columnName: string, dataType: string) => {},
            addIndexToTable: (databaseName: string, tableName: string, indexName: string, columnName: string) => {},
            addConstraintToTable: (databaseName: string, tableName: string, constraintName: string, columnName: string) => {},
            addTriggerToTable: (databaseName: string, tableName: string, triggerName: string, triggerType: string) => {},*/
            queryResult,
            queryResultHeaders,
            handleQuery
        }}>
            {children}
        </QueryProvider.Provider>
    )
}