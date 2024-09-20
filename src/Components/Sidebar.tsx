import React, { useEffect, useState } from 'react';
import TreeNode from './TreeNode'; // Import your TreeNode component
import { Database } from './TreeNode'

const sampleData: Database[] = [
  {
    name: 'my_database',
    tables: [
      {
        name: 'users',
        columns: [
          { name: 'id', data_type: 'integer' },
          { name: 'name', data_type: 'varchar' },
          { name: 'email', data_type: 'varchar' },
        ],
      },
      {
        name: 'orders',
        columns: [
          { name: 'order_id', data_type: 'integer' },
          { name: 'user_id', data_type: 'integer' },
          { name: 'amount', data_type: 'decimal' },
        ],
      },
    ],
  },
  {
    name: 'second_database',
    tables: [
      {
        name: 'products',
        columns: [
          { name: 'product_id', data_type: 'integer' },
          { name: 'name', data_type: 'varchar' },
          { name: 'price', data_type: 'decimal' },
        ],
      },
      {
        name: 'sales',
        columns: [
          { name: 'sale_id', data_type: 'integer' },
          { name: 'product_id', data_type: 'integer' },
          { name: 'quantity', data_type: 'integer' },
        ],
      },
    ],
  },
];



const Sidebar: React.FC = () => {
  const [metadata, setMetadata] = useState<Database[]>(sampleData);
  const [openDatabases, setOpenDatabases] = useState<{ [key: string]: boolean }>({});
  const [openTables, setOpenTables] = useState<{ [key: string]: boolean }>({});


  useEffect(() => {
    setMetadata(sampleData);
  }, []);

  // Toggle open/close state for databases
  const toggleDatabase = (dbName: string) => {
    setOpenDatabases((prev) => ({ ...prev, [dbName]: !prev[dbName] }));
  };

  // Toggle open/close state for tables
  const toggleTable = (tableName: string) => {
    setOpenTables((prev) => ({ ...prev, [tableName]: !prev[tableName] }));
  };

  return (
    <div className="w-64 h-full bg-gray-50 border-r border-gray-200 overflow-y-auto">
      <ul>
        {metadata.map((db) => (
          <li key={db.name}>
            <div
              className="font-bold text-lg p-2 cursor-pointer flex justify-between items-center"
              onClick={() => toggleDatabase(db.name)}
            >
              {db.name}
              <span>{openDatabases[db.name] ? '-' : '+'}</span>
            </div>
            {openDatabases[db.name] && (
              <ul className="ml-4">
                {db.tables.map((table) => (
                  <li key={table.name}>
                    <div
                      className="font-semibold text-base p-2 cursor-pointer flex justify-between items-center"
                      onClick={() => toggleTable(table.name)}
                    >
                      {table.name}
                      <span>{openTables[table.name] ? '-' : '+'}</span>
                    </div>
                    {openTables[table.name] && (
                      <ul className="ml-4">
                        {table.columns.map((column) => (
                          <li key={column.name} className="text-sm p-1">
                            {column.name} ({column.data_type})
                          </li>
                        ))}
                      </ul>
                    )}
                  </li>
                ))}
              </ul>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default Sidebar;
