import { useEffect, useState } from "react";
import { Input } from "../ui/Input";
import { Select } from "../ui/Select";
import { Button } from "../ui/Button";
import { dataTypeOptions } from "../../Types/object/DataTypeOption";
import { Column } from "../../Types/DatabaseTypes";

interface ColumnsFormProps {
  data: Column[];
  onChange: (data: Column[]) => void;
}

export const ColumnsForm = ({ data, onChange }: ColumnsFormProps) => {
  const [columns, setColumns] = useState<Column[]>(data || []);

  // Synchronize state with new data
  useEffect(() => {
    setColumns(data);
  }, [data]);

  // Handle changes to individual columns
  const handleColumnChange = (index: number, key: string, value: any) => {
    const updatedColumns = [...columns];
    updatedColumns[index] = {
      ...updatedColumns[index],
      [key]: value,
    };
    setColumns(updatedColumns);
    onChange(updatedColumns); // Propagate the change to parent
  };

  // Add a new empty column
  const handleAddColumn = () => {
    const newColumn: Column = {
      name: "",
      data_type: "",
      is_nullable: false,
      default_value: "",
      is_primary_key: false,
      maximum_length: undefined,
      //type: "column",
      table_name: "",
      db_name: "",
      type_: "",
      schema_name: ""
    };
    const updatedColumns = [...columns, newColumn];
    setColumns(updatedColumns);
    onChange(updatedColumns); // Propagate the new state
  };

  return (
    <div>
      {columns.map((column, index) => (
        <div key={index} className="grid grid-cols-2 gap-4">
          <label>Name</label>
          <Input
            type="text"
            value={column.name}
            onChange={(e) =>
              handleColumnChange(index, "name", e.target.value)
            }
          />
          <label>Data Type</label>
          <Select
            options={dataTypeOptions.PostgreSQL}
            value={column.data_type}
            onValueChange={(value) =>
              handleColumnChange(index, "data_type", value)
            }
          />
          <label>Max Length</label>
          <Input
            type="number"
            value={column.maximum_length || ""}
            onChange={(e) =>
              handleColumnChange(index, "maximum_length", e.target.value)
            }
          />
          <label>Is Nullable</label>
          <Input
            type="checkbox"
            checked={column.is_nullable}
            onChange={(e) =>
              handleColumnChange(index, "is_nullable", e.target.checked)
            }
          />
          <label>Default Value</label>
          <Input
            type="text"
            value={column.default_value}
            onChange={(e) =>
              handleColumnChange(index, "default_value", e.target.value)
            }
          />
          <label>Is Primary</label>
          <Input
            type="checkbox"
            checked={column.is_primary_key}
            onChange={(e) =>
              handleColumnChange(index, "is_primary_key", e.target.checked)
            }
          />
        </div>
      ))}
      <Button onClick={handleAddColumn}>Add Column</Button>
    </div>
  );
};
