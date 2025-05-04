import { FormField } from "../../Types/object/FormField";
import { dataTypeOptions } from "../../Types/object/DataTypeOption";
import {  useEffect,  useState } from "react";
import { Button } from "../ui/Button";
import { Input } from "../ui/Input";
import { Select } from "../ui/Select";
//import { Table as UITable } from "../ui/Table";
//import {  formSchemas } from "./FormSchema";
import { Column,/* Constraint, Function,*/ Functions, /*Index,*/ Procedure, /*Role,*/ Sequence, Table, View } from "../../Types/DatabaseTypes";
//import { useExecuteCreateSequence,CreateSequenceInfo, CreateTableInfo, useExecuteCreateTableQuery, useExecuteCreateRole, useExecuteCreateFunction, useExecuteCreateView, useExecuteCreateIndex, useExecuteCreateTrigger, useExecuteCreateDatabaseQuery } from "../../hooks/query/useExecuteCreateQuery";
//import { useExecuteEditConstraint, useExecuteEditFunction, useExecuteEditIndex, useExecuteEditSequence, useExecuteEditTableColumn, useExecuteEditView } from "../../hooks/query/useExecuteEditQuery";
//import { useMetadata } from "../../providers/MetadataProvider";
import { SqlEditor } from "../query/Editor";
import { useConnection } from "../../providers/ConnectionProvider";
import { Translation, useTranslation } from "react-i18next";
import { useFileOpenDialogCreateDatabase } from "../../hooks/system/useFileOpenDialog";
import { useGetTablesOfDatabase } from "../../hooks/system/databaseMetadataQuery";
import { useGetColumnsOfTable } from "../../hooks/system/databaseMetadataQuery";

type CreateBaseFormProps = {
    schema: FormField[];
    onSubmit: (data: any, oldCol?:any) => void;
    dynamicOptions?: typeof dataTypeOptions;
    submitButtonText?: string;
    onChange?: (value: any) => void;
    type_:string;
    data? : Table | Column | View | Procedure | Functions | Sequence | null;
    isCreate:boolean;
    databaseName: string;
    schemaName: string;
}


export const GenericForm = ({ schema, onSubmit, dynamicOptions, submitButtonText = "Submit", data, type_ ,isCreate, databaseName,schemaName}:CreateBaseFormProps) => {
    const {t} = useTranslation("object");
    const [formData, setFormData] = useState<Record<string, any>>({});
    const [colNumbers, setColNumbers] = useState<number>(1);
    const [selectedTable, setSelectedTable] = useState<string>("");
    const [selectedColumn, setSelectedColumn] = useState<string>("");
    const {currentDatabaseType} = useConnection();

    const [columns, setColumns] = useState<Column[]>([{
      name: "", data_type: "", is_nullable: false, default_value: "", is_primary_key: false, maximum_length: undefined,
     // type: "column",
      table_name: "",
      db_name: "",
      schema_name: "",
      type_: "column"
    }]);
    //const [createTableData, setCreateTableData] = useState<CreateTableInfo>();

    
    //TODO, hookban megcsinálni?
    const getDatabaseTableNames = ():string[] => {
      console.log("Database name, schema name: ", databaseName,schemaName);
      return useGetTablesOfDatabase(databaseName,schemaName);
    } 

    const getSelectedTableColumns = ():string[] => {
      if(selectedTable === ""){
        if(data){
          if('table_name' in data){
            return useGetColumnsOfTable(data?.table_name, databaseName, schemaName);
          }
        }
        return[];
      }
      return useGetColumnsOfTable(selectedTable, databaseName, schemaName);
    }

    const handleChange = (key:string,value:string | number | boolean) => {
        console.log("Changing value of", key, "to", value);
       // setFormData((prev) => ({ ...prev, columns: { ...prev.columns, [columnKey]: { ...prev.columns[columnKey], column: value } } }))
         setFormData((prev) => ({ ...prev, [key]: value }));
        console.log("formData: ", formData);
        //onChange && onChange(value);
    };

    const openDatabaseFile = async () => {
      let content = await useFileOpenDialogCreateDatabase();
      handleChange("file_content", content);
    }
    

    const handleSubmit = (e: React.FormEvent, columnKey?:string) => {
        e.preventDefault();
       // console.log("Submitted Data:", formData);
        if(type_ === "table"){
          if(columnKey && data &&'columns' in data){
            console.log("Van columnkey");
          //  console.log("Old column??", data.columns[columnKey]);
            onSubmit(formData.columns[columnKey], data?.columns[columnKey]);
          }else{
            console.log("Nincs columnkey");
            console.log("Submitted: ", formData);
            let tableColumns:Column[] = columns.filter((column) => column.name !== "");
            let table = {
            name: formData.table_name,
            columns: tableColumns.reduce((acc, column) => ({ ...acc, [column.name]: column }), {}),
            }
            console.log("Table: ",table);
            onSubmit(table);  
          }
          /*let tableColumns:Column[] = columns.filter((column) => column.name !== "");
          let table = {
            name: formData.table_name,
            columns: tableColumns.reduce((acc, column) => ({ ...acc, [column.name]: column }), {}),
          }
          onSubmit(table);*/
        }else{
          onSubmit(formData);
        }
    };



    const splitOne = (str:string,separator:string) => {
      console.log("Splitting string: ",str);
      let sepIndex = str.indexOf(separator);
      console.log("Separator index: ",sepIndex);
      let sepStr = str;
      if(sepIndex == -1) return [str];
      if(sepIndex == 0){
        sepStr = sepStr.substring(1);
        console.log("New separator string: ",sepStr);
        sepIndex = sepStr.indexOf(separator);
        console.log("Separator index new: ",sepIndex);
      }
      console.log("Splitted string: ",sepStr.slice(0, sepIndex), sepStr.slice(sepIndex + separator.length));
      return [str.slice(0, sepIndex), sepStr.slice(sepIndex + separator.length)];
      
    }


    useEffect(() => {
        if (data) {
            setFormData(() => (data));
            console.log("data: ",data);
            console.log("formData: ",formData);
            console.log("Editing schema: ", schema);
        }
    }, [data]);

    return (
        <form onSubmit={handleSubmit} className="space-y-2 flex flex-col space-x-4    ">
        {
           isCreate && type_ === "table" && (
            <div className="flex flex-col space-y-2">
              <label className="font-medium">{t("tableName")}</label>
              <Input
                type="text"
                onChange={(e) => handleChange("tableName", e.target.value)}
              />
              {
                
                columns.map((column, index) => (
                  <div key={index} className="grid grid-cols-2 gap-4">
                    <label>{t("columnName")}</label>
                    <Input
                      type="text"
                      value={column.name || ""}
                      onChange={(e) => setColumns(prev => prev.map((col, i) => i === index ? { ...col, name: e.target.value } : col))}
                    />
                    <label>{t("dataType")}</label>
                    <Select
                      value={column.data_type || ""}
                      options={dynamicOptions?.[currentDatabaseType] || []}
                      onValueChange={(e) => setColumns(prev => prev.map((col, i) => i === index ? { ...col, data_type: e } : col))}
                    />
                    <label>{t("isNullable")}</label>
                    <Input
                      type="checkbox"
                      checked={column.is_nullable}
                      onChange={(e) => setColumns(prev => prev.map((col, i) => i === index ? { ...col, is_nullable: e.target.checked } : col))}
                    />
                    <label>{t("defaultValue")}</label>
                    <Input
                      type="text"
                      value={column.default_value || ""}
                      onChange={(e) => setColumns(prev => prev.map((col, i) => i === index ? { ...col, default_value: e.target.value } : col))}
                    />
                    <label>{t("isPrimaryKey")}</label>
                    <Input
                      type="checkbox"
                      checked={column.is_primary_key}
                      onChange={(e) => setColumns(prev => prev.map((col, i) => i === index ? { ...col, is_primary_key: e.target.checked } : col))}
                    />
                    <label>{t("maximumLength")}</label>
                    <Input
                      type="number"
                      value={column.maximum_length || ""}
                      onChange={(e) => setColumns(prev => prev.map((col, i) => i === index ? { ...col, maximum_length: parseInt(e.target.value) } : col))}
                    />
                    <hr className="my-2"/>
                  </div>
                ))
              }
                <Button onClick={() => setColumns(prev => [...prev, { name: "", data_type: "", is_nullable: false, default_value: "", is_primary_key: false, maximum_length: undefined, type: "column", table_name: "", db_name: "", type_: "column",schema_name:"", }])}>Add Column</Button>
            </div>
            
          )
        }
        {
          isCreate && type_ == "database" && (
            <div>
              <Input
                type="text"
                onChange={(e) => handleChange("databaseName", e.target.value)}
              />
              <Button onClick={openDatabaseFile} title="Open Database"/>
            </div>
          )
        }
        {
          !data && schema.map((field) => (
            <div key={field.key} className="flex flex-col space-y-2">
              <label className="font-medium">{t(`${field.label}` as keyof typeof Translation)}</label>
              {field.type === "text" && (
                <Input
                  type="text"
                  onChange={(e) => handleChange(field.key, e.target.value)}
                />
              )}
              {field.type === "number" && (
                <Input
                  type="number"
                  onChange={(e) => handleChange(field.key, e.target.value.toString()/*parseInt(e.target.value.toString())*/)}
                />
              )}
              {field.type === "checkbox" && (
                <Input
                  type="checkbox"
                  onChange={(e) => handleChange(field.key, e.target.checked)}
                />
              )}
              {
                field.type === "select" && field.selectType === "table" && (
                  <Select 
                    value={selectedTable || ""}
                    options={getDatabaseTableNames()}
                    onValueChange={(e) => {setSelectedTable(e); handleChange(field.key, e)}}
                  />
              )}
              {
                field.type === "select" && field.selectType === "column" && (
                  <Select 
                    value={selectedColumn || ""}
                    options={getSelectedTableColumns()}
                    onValueChange={(e) => {setSelectedColumn(e); handleChange(field.key, e)}}
                  />
              )}
              {
                field.type === "select" && field.selectType === "type" && (
                  <Select 
                    value={field.key || ""}
                    options={field.options || dynamicOptions?.[currentDatabaseType] || []}
                    onValueChange={(e) => handleChange(field.key, e)}
                  />
              )
              }
            
            </div>
          ))
        }
        
        {
          formData.columns && type_=="table" && (
            <div>
              <button>Add Column</button>
              <div className="grid grid-cols-[repeat(auto-fit,minmax(60px,1fr))] gap-4 items-center font-bold text-gray-700">
                {schema.map((field) => (
                  <div key={field.key} className="text-center">
                    {field.label}
                  </div>
                ))}
              </div>
              {
              Object.keys(formData.columns).map((columnKey, _rowIndex) => (  
              
                <div
                  key={columnKey}
                  className={`grid grid-cols-[repeat(auto-fit,minmax(60px,1fr))] gap-2 items-center p-2 rounded`}
                >
                  {
                    <div className="flex flex-col space-y-2">
                      <label>{t("name")}</label>
                      <Input 
                        type="text"
                        value={formData.columns[columnKey].name || ""}
                        onChange={(e) => {
                          e.preventDefault();
                          setFormData((prev) => ({ ...prev, columns: { ...prev.columns, [columnKey]: { ...prev.columns[columnKey], name: e.target.value } } }))/*handleChange(`columns.${columnKey}.name`, e.target.value)*/
                        }}
                      />
                      <label>Column Key: {columnKey}, columnkey data type: {formData.columns[columnKey].data_type}</label>
                      <label>Contains varchar? {formData.columns[columnKey].data_type.toUpperCase().includes("VARCHAR")}</label>
                      <label>Typeof: {typeof formData.columns[columnKey].data_type}</label>
                      <label>{t("dataType")}</label>
                      <Select
                        value={formData.columns[columnKey].data_type.toLowerCase().includes("nvarchar") ? "text" : formData.columns[columnKey].data_type.toLowerCase()}
                        options={dynamicOptions?.[currentDatabaseType] || []}
                        onValueChange={(e) => {
                          setFormData((prev) => ({ ...prev, columns: { ...prev.columns, [columnKey]: { ...prev.columns[columnKey], data_type: e } } }))/*handleChange(`columns.${columnKey}.data_type`, e)*/
                        }}
                      />
                      <label>{t("maximumLength")}</label>
                      <Input
                        type="number"
                        value={formData.columns[columnKey].maximum_length || ""}
                        onChange={(e) => {
                          e.preventDefault();
                          setFormData((prev) => ({ ...prev, columns: { ...prev.columns, [columnKey]: { ...prev.columns[columnKey], maximum_length: parseInt(e.target.value) } } }))
                        }}
                      />
                      <label>{t("primaryKey")}</label>
                      <Input
                        type="checkbox"
                        checked={formData.columns[columnKey].is_primary_key || false}
                        onChange={(e) => {
                          e.preventDefault();
                          setFormData((prev) => ({ ...prev, columns: { ...prev.columns, [columnKey]: { ...prev.columns[columnKey], is_primary_key: e.target.checked } } }))
                        }}
                      />
                      <label>{t("nullable")}</label>
                      <Input
                        type="checkbox"
                        checked={formData.columns[columnKey].is_nullable || false}
                        onChange={(e) => {
                          e.preventDefault();
                          setFormData((prev) => ({ ...prev, columns: { ...prev.columns, [columnKey]: { ...prev.columns[columnKey], is_nullable: e.target.checked } } }))
                        }}
                      />
                      <label>{t("defaultValue")}</label>
                      <Input
                        type="text"
                        value={formData.columns[columnKey].default_value || ""}
                        onChange={(e) => {
                          e.preventDefault();
                          setFormData((prev) => ({ ...prev, columns: { ...prev.columns, [columnKey]: { ...prev.columns[columnKey], default_value: e.target.value } } }))
                        }}
                      />
                      <Button type="submit" onClick={(e) => handleSubmit(e, columnKey,)}>Save</Button>
                    </div>
                    
                  /*schema.map((field) => (
                    <div key={`${columnKey}-${field.key}`}>
                      {field.type === "text" && (
                        <Input
                          type="text"
                          value={formData.columns[columnKey][field.key] || ""}
                          onChange={(e) =>
                            handleChange(`columns.${columnKey}.${field.key}`, e.target.value)
                          }
                        />
                      )}
                      {field.type === "number" && (
                        <Input
                          type="number"
                          value={formData.columns[columnKey][field.key] || ""}
                          onChange={(e) =>
                            handleChange(`columns.${columnKey}.${field.key}`, e.target.value)
                          }
                        />
                      )}
                      {field.type === "select" && (
                        <Select 
                          value={formData.columns[columnKey][field.key] || ""}
                          options={field.options || dynamicOptions?.["PostgreSQL"] || []}
                          onValueChange={(e) => handleChange(`columns.${columnKey}.${field.key}`, e)}
                        />
                      )}
                      {field.type === "checkbox" && (
                        <div className="flex justify-center">
                          <Input
                            type="checkbox"
                            checked={formData.columns[columnKey][field.key] || false}
                            onChange={(e) =>
                              handleChange(`columns.${columnKey}.${field.key}`, e.target.checked)
                            }
                          />
                        </div>
                      )}
                    </div>
                  ))*/}
                </div>
              ))}
              <Button onClick={() => setColNumbers(colNumbers + 1)}>Add Column</Button>
            </div>
        )

        }
        { 
         type_!=="table" && data && schema.map((field) => (
            <div key={field.key} className="flex flex-col space-y-2 h-full">
              <label className="font-medium">{t(`${field.label}` as keyof typeof Translation)}</label>
              {field.type === "text"&& (
                <div>
                  <Input
                  type={field.type}
                  value={formData[field.key] || ""}
                  onChange={(e) => setFormData((prev) => ({ ...prev, [field.key]: e.target.value }))}
                  //onChange={(e) => handleChange(field.key, e.target.value)}
                />
                </div>
              )}
              {
                field.type === "parameter" && (
                  <table>
                    <thead>
                      <th>name</th>
                      <th>value</th>
                    </thead>
                    <tbody>
                      {
                        formData["parameters"]?.map((param: string, index: string | number /*MAYBE TODO*/) => (
                          
                          <tr key={index}>
                            <td>
                              <Input
                                type="text"
                                value={splitOne(param, " ")[0] || ""}
                                onChange={(e) => {
                                  e.preventDefault();
                                  setFormData((prev) => ({ ...prev, parameters: { ...prev.parameters, [index]: { ...prev.parameters[index], name: e.target.value } } }))
                                }}
                              />
                            </td>
                            <td>
                              <Input
                                type="text"
                                value={splitOne(param, " ")[1] || ""}
                                onChange={(e) => {
                                  e.preventDefault();
                                  setFormData((prev) => ({ ...prev, parameters: { ...prev.parameters, [index]: { ...prev.parameters[index], value: e.target.value } } }))
                                }}
                              />
                            </td>
                          </tr>
                        ))
                      }
                    </tbody>
                  </table>
                  //<UITable headers={["name"]} rows={} isObject={false}/>
                  /*<Input
                    type="text"
                    value={formData["parameters"][0] || ""}
                    onChange={(e) => setFormData((prev) => ({ ...prev, [field.key]: e.target.value }))}
                    onChange={(e) => handleChange(field.key, e.target.value)}
                  />*/
                )
              }
              {field.type === "number" && (
                <Input
                  type="number"
                  value={formData[field.key] || ""}
                  onChange={(e) => setFormData((prev) => ({ ...prev, [field.key]: e.target.value }))}
                 // onChange={(e) => handleChange(field.key, parseInt(e.target.value))}
                />
              )}
              {field.type === "checkbox" && (
                <Input
                  type="checkbox"
                  checked={formData[field.key] || false}
                  onChange={(e) => setFormData((prev) => ({ ...prev, [field.key]: e.target.checked }))}
                 // onChange={(e) => handleChange(field.key, e.target.checked)}
                />
              )}
                           {
                field.type === "select" && field.selectType === "table" && (
                  <Select 
                    value={selectedTable || formData[field.key]}
                    options={getDatabaseTableNames()}
                    onValueChange={(e) => {setSelectedTable(e); handleChange(field.key, e)}}
                  />
              )}
              {
                field.type === "select" && field.selectType === "column" && (
                  
                  <Select 
                    value={selectedColumn || formData[field.key]}
                    options={getSelectedTableColumns()}
                    onValueChange={(e) => {setSelectedColumn(e); handleChange(field.key, e)}}
                  />
              )}
              {
                <p>{formData[field.key]}</p>
              }
              {
                field.type === "select" && field.selectType === "type" && (
                  <Select 
                    value={field.key || ""}
                    options={field.options || dynamicOptions?.[currentDatabaseType] || []}
                    onValueChange={(e) => handleChange(field.key, e)}
                  />
              )}
              {
                field.type === "code" && (
                  <SqlEditor sqlType="postgres" value={formData[field.key] || ""}
                    onChange={(e) => setFormData((prev) => ({ ...prev, [field.key]: e }))}
                    
                  />
                )
              }
              {
                field.type === "columns" && (
                  <div>
                    <h3 className="font-bold mb-2">Columns</h3>
                  </div>
                )
              }
            </div>
          ))
        }
        
        <button type="submit" className="bg-blue-500 text-white px-4 py-2 rounded">
          {submitButtonText}
        </button>
      </form>
    );
}


/*
<Select onValueChange={(e) => handleChange(field.key, e)} options={field.options || dynamicOptions?.["PostgreSQL"]}} />
              /*<select
                className="border rounded p-2 color-gray-700"
                required={field.required}
                onChange={(e) => handleChange(field.key, e.target.value)}
              >
                <option value="" className="text-gray-700">Select...</option>
                {(field.options || dynamicOptions?.["PostgreSQL"])?.map((option) => (  
                  <option key={option} value={option} className="text-gray-700">
                    {option}
                  </option>
                ))}
              </select>*/