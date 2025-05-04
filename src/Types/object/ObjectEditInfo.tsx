import { Table, Column, View, Procedure, Functions, Sequence, Schema, Constraint, Index, Role, TriggerFunction } from "../DatabaseTypes";

export type ObjectEditInfo = {
    type:string;
    database_name:string;
    schema_name:string;
    editObject:Column | Table | Functions | Sequence | View | Procedure | Constraint | Index | Schema | TriggerFunction | Role//Table | Column[] | View | Procedure | Functions | Sequence | Schema
}