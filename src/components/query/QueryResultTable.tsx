import { Table } from "../ui/Table";
import { useQuery } from "../../providers/QueryProvider";
import { useEffect } from "react";


export const QueryResultTable = () => {
    const {queryResult, queryResultHeaders } = useQuery();

    useEffect(() => {
        console.log("Query result headers: ",queryResultHeaders);
    },[queryResult,queryResultHeaders])

    return(
        <div>
            <Table headers={queryResultHeaders ?? []} rows={queryResult?.rows ?? []} isObject={true}/>
        </div>
    )

};

