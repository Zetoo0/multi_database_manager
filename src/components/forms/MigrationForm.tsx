import { useEffect, useState } from "react";
import { useMetadata } from "../../providers/MetadataProvider";
import { Select } from "../ui/Select";
import { Button } from "../ui/Button";
import { executeMigration } from "../../hooks/migration/useExecuteMigration";
import { MultiSelect } from "../ui/MultiSelect";
import { Input } from "../ui/Input";
import { useTranslation } from "react-i18next";

/*enum DatabaseType{
    MySql,
    Postgres,
    MsSql,
    SqLite,
    Oracle
}*/
type DatabaseType = "MySql" | "Postgres" | "MsSql" | "SqLite" | "Oracle";

export interface MigrationConfig{
    db_name:string;
    db_type:string;
    limit?: DMLLimit;
    exclude_columns?: string[];
    obfuscations?: Obfuscation;
}


interface DMLLimit{
    colname:string;
    limit: number;//usize
}

type ObfuscationType = "FIXED" | "REPLACE";

interface Obfuscation{
    type_:ObfuscationType;
    col_name:string[];
}


export const MigrationForm = () => {
    const {databaseMetadata} = useMetadata();
    const [selectedDatabase, setSelectedDatabase] = useState<string>("");
    const [selectedDatabaseType, setSelectedDatabaseType] = useState<DatabaseType>("MySql");
    const [selectedOptions, setSelectedOptions] = useState<string[]>([]);
    const [excludeColumns,setExcludeColumns] = useState<string[]>([]);
    const [isObfuscation,setIsObfuscation] = useState<boolean>(false);
    const [selectedOnfuscationType,setSelectedObfuscationType] = useState<ObfuscationType>("FIXED");
    const [selectedObfuscationColumns, setSelectedObfuscationColumns] = useState<string[]>([]);
    const [dmlLimitNum, setDMLLimitNum] = useState<number>(10);

    const {t } = useTranslation("form");

    const onSubmit = async () => {
        const dmlLimit:DMLLimit = {
            colname: "",
            limit:dmlLimitNum,
        }
        const obfusc:Obfuscation = {
            type_: selectedOnfuscationType,
            col_name: selectedObfuscationColumns
        }
        const config:MigrationConfig = {
            db_name: selectedDatabase,
            db_type: "MySql",
            limit: dmlLimit,
            exclude_columns: selectedOptions,
            obfuscations: obfusc
        }
        console.log("migration config: ",config);
        await executeMigration(config);
    }

    const onSelectOptionAdd = (value:string) => {
        if(selectedOptions.includes(value)){
            setSelectedOptions(selectedOptions.filter((option) => option != value));
            return;
        }
        setSelectedOptions([...selectedOptions,value]);
    }

    const onObfuscationOptionAdd = (value:string) => {
        if(selectedObfuscationColumns.includes(value)){
            setSelectedObfuscationColumns(selectedObfuscationColumns.filter((option) => option != value));
            return;
        }
        setSelectedObfuscationColumns([...selectedObfuscationColumns,value]);
    }

    const onDatabaseChange = (db_name:string) => {
        setSelectedDatabase(db_name);
        getSelectedDatabaseColumns();
    }

    const onObfuscationChange = () => {
        setIsObfuscation(!isObfuscation);
    }

    const getSelectedDatabaseColumns = () => {
        //console.log("selected database: ",selectedDatabase);
        //setExcludeColumns([]);
       // let colName = databaseMetadata[db].schemas["public"].tables[table].columns[column].name;
       let tempArr:string[] = []; 
       //TODO LEHET KELL
       Object.keys(databaseMetadata).filter((db) => db !== selectedDatabase).map((db) => Object.keys(databaseMetadata[db].schemas["public"].tables).map((table) => Object.keys(databaseMetadata[db].schemas["public"].tables[table].columns).map((column) => {
        let colName = databaseMetadata[db].schemas["public"].tables[table].columns[column].name;
        if(!tempArr.includes(colName)){
            tempArr.push(colName);
        }
       })));
       const tableNames:string[] = Object.keys(databaseMetadata[selectedDatabase].schemas["public"].tables);
       const columnNamesForSelectedTable:string[] = Object.keys(databaseMetadata[selectedDatabase].schemas["public"].tables[tableNames[3]].columns);
       console.log("Tables of selected database: ",tableNames);
       console.log("Columns of selected table: ",columnNamesForSelectedTable); 
       console.log("Current exclude columns: ",tempArr);
        setExcludeColumns(tempArr);
        
        //TODO, adott adatbázis tábláinak kiválasztása
        //      és kiválasztott tábla oszlopai, 2 select

       // Object.keys(databaseMetadata).filter((db) => db !== selectedDatabase).map((db) => Object.keys(databaseMetadata[db].schemas["public"].tables).map((table) => Object.keys(databaseMetadata[db].schemas["public"].tables[table].columns).map((column) => {
            
       // })))
        
        //setExcludeColumns((cols) => [...cols, ]);
        //console.log("After pushing exclude columns: ",excludeColumns);
    }
    
    useEffect(() => {
        console.log("Exclude columns: ",excludeColumns);
    }, [excludeColumns]);

    return (
        <div className="flex flex-col gap-2 rounded-lg shadow-lg max-w-md mx-auto">
                <label>{t("migration.database")}</label>
                {
                    databaseMetadata && (
                        <Select options={Object.keys(databaseMetadata).map((db) => db)} onValueChange={(e) => onDatabaseChange(e)} value={selectedDatabase ?? ""}/>
                    )
                }

                <label>{t("migration.databaseType")}</label>
                <Select options={["MsSql","MySql","Postgres","SqLite","Oracle"]} onValueChange={(e) => setSelectedDatabaseType(e as DatabaseType)} value={selectedDatabaseType.toString() ?? ""}/>
                <Button  onClick={onSubmit}>{t("migration.migrate")}</Button>
                <label>{t("migration.dmlRowLimitPerFile")}</label>
                <Input type="number" value={dmlLimitNum} onChange={(e) => setDMLLimitNum(parseInt(e.target.value))} />
                <label>{t("migration.excludeColumns")}</label>
                {
                        databaseMetadata &&(
                        <MultiSelect value="" options={excludeColumns} onValueChange={onSelectOptionAdd} placeholder="Select Columns"/> 
                        )
                }
                <Input type="checkbox" onChange={onObfuscationChange} />
                {
                    isObfuscation && (
                        <div>

                            <MultiSelect value="" options={excludeColumns} onValueChange={onObfuscationOptionAdd} placeholder="Select Obfuscation Columns"/> 
                            <Select options={["FIXED","REPLACE"]} onValueChange={(e) => setSelectedObfuscationType(e as ObfuscationType)} value={selectedOnfuscationType.toString() ?? ""}/>
                        </div>
                    )
                }
        </div>
 
)
}