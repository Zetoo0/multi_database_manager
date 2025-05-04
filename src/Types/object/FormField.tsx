import { dataTypeOptions } from "./DataTypeOption"

export type FormField = {
    label:string;
    key:string;
    type: "text" | "number" | "checkbox" | "select" | "columns" | "code" | "parameter";
    options? : string[];
    specificOptionsKey? : keyof typeof dataTypeOptions;
    inputType?: string;
    required?: boolean;
    selectType?: string;
}



