import { TreeNode } from "../../ui/TreeNode";
import { Tree } from "../../ui/Tree";
import { useMetadata } from "../../../providers/MetadataProvider";
import { convertDatabaseMetadataToTreeNode } from "../../ui/Tree";
import { useNavigate } from "@tanstack/react-router";
import { useConnection } from "../../../providers/ConnectionProvider";
import { useObject } from "../../../providers/ObjectProvider";
import { useExecuteDeleteBaseObject, useExecuteDeleteTableColumn } from "../../../hooks/query/useExecuteDeleteQuery";
import { Column } from "../../../Types/DatabaseTypes";
import { useEffect } from "react";


export const DatabaseTree = () => {
    const navigate = useNavigate();
    const {isConnected} = useConnection();
    const {setObjectCreationInfo, editObject, setEditObject} = useObject();
    const { databaseMetadata } = useMetadata();
    const {saveBaseObjectData} = useExecuteDeleteBaseObject()
    const { saveTableData} = useExecuteDeleteTableColumn();
    const {currentDatabaseType} = useConnection();
   // const {databaseMetadata} = useMetadata();

    //const {databaseMetadata,status} = useDatabaseMetadata(isConnected);
    // const metadatas = data || {}; 
    
    let databases = databaseMetadata ? Object.values(databaseMetadata).map((db) => convertDatabaseMetadataToTreeNode(db)) : [];
    console.log("Databases: ",databases);

    useEffect(() => {
        console.log("Database metadata: ",databaseMetadata);
       // databases  = databaseMetadata ? Object.values(databaseMetadata).map((db) => convertDatabaseMetadataToTreeNode(db));
    },[databaseMetadata])

    /*const nodePathMap = {
        table: (node) => `schemas[public].tables[${node.name}]`,
        view: (node) => `schemas[public].views[${node.name}]`,
        role: (node) => `schemas[public].roles[${node.name}]`,
        function: (node) => `schemas[public].functions[${node.name}]`,
        sequence: (node) => `schemas[public].sequences[${node.name}]`,
        index: (node) => `schemas[public].tables[${node.table_name}].indexes[${node.name}]`,
        constraint: (node) => `schemas[public].tables[${node.table_name}].constraints[${node.name}]`,
        trigger: (node) => `schemas[public].tables[${node.table_name}].triggers[${node.name}]`,
    };*/

    const handleContextMenuAction = (node: Tree,action:string) => {

        if(action.includes("create")){
            if(node.type == "schema" || node.type == "table"){
                let converted_type = action.split("-")[1];
                console.log("Splitted action and unsplitted: ", action,converted_type);
                console.log("Create converted type: " ,converted_type);
                console.log("Creating node: ", node);
                console.log("Creating node schema name: ", node.object?.schema_name ?? "public");
                setObjectCreationInfo({type: converted_type,database_name: node.root_name, schema_name: node.object?.schema_name ?? "public"});
                navigate({to:"/object/definitions"});//to create
                return;
            }
            console.log("Creating node: ", node);
            console.log("Creating node schema name: ", node.object?.schema_name);
            setObjectCreationInfo({type: node.type,database_name: node.root_name, schema_name: node.object?.schema_name ?? "public"});
            console.log("Create type: ",node.type);
            navigate({to:"/object/definitions"});//to create
            return;
        }else if(action.includes("edit")){
            console.log("I got a node to edit: ", node);
            if(node.object){
                setEditObject({
                    type: node.type, editObject: node.object,
                    database_name: node.root_name,
                    schema_name: node.object.schema_name
                });
            }
            console.log("Edit dat type: ",node.type);
            console.log("Edit object: ", editObject);
            navigate({to:"/object/ObjectProperties"});//to edit    
            //console.log("nodeinfo: ",node);
            return;
        }else if(action.includes("delete")){
            switch(node.type){  
                case "column":
                    console.log("Deleting column: ", node);
                    let obj = node.object as Column
                    saveTableData({table_name:obj.table_name,column_name: node.name,db_name:node.root_name})
                    break;
                default: 
                    console.log("Delete object base: ", node);
                    saveBaseObjectData({delete_to_name:node.type,object_name:node.name,db_name:node.root_name,database_type:currentDatabaseType});
                    switch(node.type){
                        case "table":
                            delete databaseMetadata["dvdrental"].schemas["public"].tables[node.name];
                            break;
                        case "role":
                            delete databaseMetadata["dvdrental"].schemas["public"].roles[node.name];
                            break;
                        case "sequence":
                            delete databaseMetadata["dvdrental"].schemas["public"].sequences[node.name];
                            break;
                        case "function":
                           delete databaseMetadata["dvdrental"].schemas["public"].functions[node.name]
                            break;
                        case "index":
                            delete databaseMetadata["dvdrental"].schemas["public"].tables[node.table_name ?? ""].indexes[node.name];
                            break;
                        case "constraint":
                            delete databaseMetadata["dvdrental"].schemas["public"].tables[node.table_name ?? ""].constraints[node.name]
                            break;
                    }
                    console.log("New metadatas after delete: ", databaseMetadata);
                    break;
            }
            return;
        }

     /*   switch(action){
            case "create":
                if(node.type == "schema"){
                    let converted_type = action.split("-")[1];
                    console.log("Creating node: ", node);
                    setObjectCreationInfo({type: converted_type,database_name: node.root_name});
                    console.log("Create type: " ,converted_type);
                    navigate({to:"/object/table"});//to create
                    break;
                }
                console.log("Creating node: ", node);
                setObjectCreationInfo({type: node.type,database_name: node.root_name});
                console.log("Create type: ",node.type);
                navigate({to:"/object/table"});//to create
                break;
            case "edit": 
                if(node.object){
                    setEditObject({
                        type: node.type, editObject: node.object,
                        database_name: node.root_name
                    });
                }
                console.log("Edit dat type: ",node.type);
                console.log("Edit object: ", editObject);
                navigate({to:"/object/ObjectProperties"});//to edit    
                console.log("nodeinfo: ",node);
        }*/
    }

    /*useEffect(() => {
        console.log("isConnected: ",isConnected);
        console.log("Metadata status: ",status);
        console.log("Database Metadata: ",databaseMetadata);
    },[databaseMetadata,isConnected,status]);
*/
    return (
        <div>
            {
            isConnected  &&  databases.map((node,index) => {
                    return <TreeNode key={index} node={node} onAction={(action,node) => handleContextMenuAction(node,action)}/>
                })
            }
        </div>
    )
}