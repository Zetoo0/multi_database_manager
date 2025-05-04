import { FormField } from "../../Types/object/FormField";

export type FormSchema = Record<string, FormField[]>;

export const formSchemas:FormSchema = {
    table : [
        {
            label:"Table Name",
            key:"table_name",
            type:"text",
            required:true
        },
        {
            label:"Columns",
            key:"columns",
            type:"columns",
            required:true
        },
        
    ],
    column : [
        {
            label:"Column Name",
            key:"name",
            type:"text",
            required:true
        },
        {
            label:"Data Type",
            key:"data_type",
            type:"select",
            specificOptionsKey: "MSSQL",
            required:true,
            selectType:"type"
        },
        {
            label:"Is Nullable",
            key:"is_nullable",
            type:"checkbox",
            required:true
        },
        {
            label:"Is Primary Key",
            key:"is_primary_key",
            type:"checkbox",
            required:true
        },
        {
            label:"Default Value",
            key:"default_value",
            type:"text",
            required:true
        },
        {
            label:"Maximum Length",
            key:"maximum_length",
            type:"number",
            required:true
        }
    ],
    view : [
        {
            label:"View Name",
            key:"name",
            type:"text",
            required:true
        },
        {
            label:"Definition",
            key : "definition",
            type : "code",
            required : true
         }
    ],
    trigger: [
        { label: "Trigger Name", key: "name", type: "text", required: true },
        { label: "When", key: "when", type: "text", required: true },
        { label: "Type", key: "type", type: "text", required: true },
        { label: "Table Name", key: "table_name", type: "select", required: true,selectType:"table" },
        { label: "Trigger Function Name", key: "function_name", type: "text", required: true },
    ],
    triggerfunction: [
        { label: "Function Name", key: "name", type: "text", required: true },
        { label: "Definition", key: "definition", type: "code" }, // Handle parameters dynamically
    ],
    function: [
        { label: "Function Name", key: "name", type: "text", required: true },
        { label: "Definition", key: "definition", type: "code" }, 
      //  { label: "Parameters", key: "children", type: "parameter" }, // Handle parameters dynamically
    ],
    procedure: [
        { label: "Procedure Name", key: "name", type: "text", required: true },
        { label: "Definition", key: "definition", type: "code" }, 
      //  { label: "Parameters", key: "children", type: "parameter" }, // Handle parameters dynamically
    ],
    sequence: [
        { label: "Sequence Name", key: "name", type: "text",required:true },
        { label: "Increment", key: "increment", type: "number",required:true }, // Handle parameters dynamically
        { label: "Start", key: "start", type: "number",required:true }, // Handle parameters dynamically
        { label: "Maximum", key: "maximum", type: "number",required:true }, // Handle parameters dynamically
        { label: "Minimum", key: "minimum", type: "number",required:true }, // Handle parameters dynamically
        { label: "Cycled", key: "cycled", type: "checkbox",required:true }, // Handle parameters dynamically
        { label: "Cached", key: "cached", type: "checkbox",required:true }, // Handle parameters dynamically
        { label: "Owned By Table", key: "owned_by_table", type: "select",required:true, selectType:"table"}, // Handle parameters dynamically
        { label: "Owned By Column", key: "owned_by_col", type: "select",required:true, selectType:"column" }, // Handle parameters dynamically
    ],
    constraint : [
        {label: "Type",  key:"c_type",type:"text",required:true},
        {label: "Name",  key:"name",type:"text",required:true},
        {label: "Table Name",  key:"table_name",type:"select",required:true,selectType:"table"},
        {label: "Column Name",  key:"column_name",type:"select",required:true,selectType:"column"},
    ],
    unique_key: [
        { label: "Column", key: "col", type: "text",required:true }, // Handle parameters dynamically
        { label: "Include Column", key: "include_col", type: "text",required:true }, // Handle parameters dynamically
        { label: "Tablespace", key: "tablespace", type: "text",required:true }, // Handle parameters dynamically
        { label: "Index", key: "index", type: "text",required:true }, // Handle parameters dynamically
    ],
    check: [
        { label: "Check", key: "check", type: "text",required:true }, // Handle parameters dynamically
        { label: "No Inherit?", key: "no_inherit_qm", type: "checkbox",required:true }, // Handle parameters dynamically
        { label: "Don't validate?", key: "dont_validate_qm", type: "checkbox",required:true }, // Handle parameters dynamically
    ],
    index : [
        {label: "Name",  key:"name",type:"text",required:true},
        {label: "Definition",  key:"definition",type:"text",required:true},
        {label: "Non Unique",  key:"non_unique",type:"checkbox",required:true},
        {label: "Table Name",  key:"table_name",type:"select",required:true,selectType:"table"},
        {label: "Column Name",  key:"column_name",type:"select",required:true,selectType:"column"},    ],
    foreign_key: [
        { label: "Deferrable?", key: "deferrable_qm", type: "checkbox",required:true }, // Handle parameters dynamically
        { label: "Deferred?", key: "deferred_qm", type: "checkbox",required:true }, // Handle parameters dynamically
        { label: "Match Type", key: "match_type", type: "select",required:true }, // Handle parameters dynamically
        { label: "Validated?", key: "validated", type: "checkbox",required:true }, // Handle parameters dynamically
        { label: "Auto FK index?", key: "auto_fk_index", type: "checkbox",required:true }, // Handle parameters dynamically
        { label: "Covering Index", key: "covering_index", type: "select",required:true }, // Handle parameters dynamically
    ],
    primary_key: [
        { label: "Column", key: "col", type: "select",required:true }, // Handle parameters dynamically
        { label: "Include Column", key: "include_col", type: "select",required:true }, // Handle parameters dynamically
        { label: "Tablespace", key: "tablespace", type: "select",required:true }, // Handle parameters dynamically
        { label: "Index", key: "index", type: "select",required:true }, // Handle parameters dynamically
    ],
    role : [
        {label: "Name",  key:"name",type:"text",required:true},
        {label: "Can Login",  key:"can_login",type:"checkbox",required:true},
        {label: "Is inherit",  key:"is_insherit",type:"checkbox",required:true},
        {label: "Is Super",  key:"is_super",type:"checkbox",required:true},
        {label: "Is Create Role",  key:"is_create_role",type:"checkbox",required:true},
        {label: "Is Create DB",  key:"is_create_db",type:"checkbox",required:true},
        {label: "Is Replication",  key:"is_replication",type:"checkbox",required:true},
        {label: "Connection Limit",  key:"connection_limit",type:"number",required:true},
        {label: "Valid Until",  key:"valid_until",type:"text",required:true},
        {label: "Password",  key:"password",type:"text",required:true},
    ],
    schema : [
        {label: "Name",  key:"name",type:"text",required:true},
        {label: "User Name",  key:"user_name",type:"text",required:true},
    ],
    database : [
        {label: "Name",  key:"name",type:"text",required:true},
    ]   
}