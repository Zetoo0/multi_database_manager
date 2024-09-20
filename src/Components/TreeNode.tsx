import React from 'react';

type Column = {
  name: string;
  type: 'column';
};

type Table = {
  name: string;
  type: 'table';
  children: Column[];
};

type View = {
  name: string;
  type: 'view';
  children: Column[];
};

type Procedure = {
  name: string;
  type: 'procedure';
  children: { name: string; type: 'parameter' }[];
};

type Database = {
  name: string;
  type: 'database';
  children: (Table | View | Procedure)[];
};

export interface TreeNodeProps {
  database: Database;
}

const TreeNode: React.FC<TreeNodeProps> = ({ database }) => {
  return (
    <div className="ml-4">
      <div className="font-bold text-lg">{database.name}</div>
      {database.tables.map((table) => (
        <div key={table.name} className="ml-4">
          <div className="text-md">{table.name}</div>
          <div className="ml-4">
            {table.columns.map((column) => (
              <div key={column.name} className="text-sm text-gray-600">
                {column.name} ({column.data_type})
              </div>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
};

export default TreeNode;
