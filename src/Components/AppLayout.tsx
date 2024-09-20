import { useState } from "react";
import { FiDatabase, FiSettings, FiFileText, FiChevronLeft, FiChevronRight } from 'react-icons/fi';
import { FaTable, FaColumns, FaFolder } from 'react-icons/fa';

// Sample data structure
const sampleData = [
  {
    name: "Database1",
    type: "database",
    children: [
      {
        name: "Users",
        type: "table",
        children: [
          { name: "id", type: "column" },
          { name: "name", type: "column" },
        ],
      },
      {
        name: "Orders",
        type: "table",
        children: [
          { name: "order_id", type: "column" },
          { name: "user_id", type: "column" },
        ],
      },
    ],
  },
  {
    name: "Database2",
    type: "database",
    children: [
      {
        name: "Products",
        type: "table",
        children: [
          { name: "product_id", type: "column" },
          { name: "price", type: "column" },
        ],
      },
    ],
  },
];

interface TreeNodeProps {
  node: any;
}

const TreeNode: React.FC<TreeNodeProps> = ({ node }) => {
  const [expanded, setExpanded] = useState(false);

  const handleNodeClick = () => {
    console.log(`Clicked on ${node.name}`);
  };

  return (
    <div className="pl-4">
      <div className="flex items-center justify-between cursor-pointer">
        <span onClick={() => setExpanded(!expanded)} className="mr-1">
          {expanded ? <span>▼</span> : <span>▶</span>}
        </span>
        
        <span onClick={handleNodeClick} className="flex items-center cursor-pointer">
          {node.type === 'database' ? <FiDatabase className="mr-1" /> 
          : node.type === 'table' ? <FaTable className="mr-1" /> 
          : node.type === 'column' ? <FaColumns className="mr-1" /> 
          : <FaFolder className="mr-1" />}
          <span className="text-white">{node.name}</span>
        </span>
      </div>
      {expanded && node.children && (
        <div className="ml-4">
          {node.children.map((child: any, index: number) => (
            <TreeNode key={index} node={child} />
          ))}
        </div>
      )}
    </div>
  );
};

const AppLayout: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [sidebarLeft, setSidebarLeft] = useState<boolean>(true);
  
  return (
    <div className="flex flex-col h-screen bg-gray-800">
      {/* Top Navigation Bar */}
      <div className="bg-gray-800 text-white flex items-center p-4 space-x-4">
        <button className="hover:text-red-200">File</button>
        <button className="hover:text-red-200">Tools</button>
        <button className="hover:text-red-200">Edit</button>
        <button className="hover:text-red-200">Help</button>
        <button className="hover:text-red-200">Window</button>
      </div>

      <div className="flex flex-1">
        {/* Sidebar */}
        <div className={`${sidebarLeft ? 'order-first' : 'order-last'} w-64 bg-gray-800 text-white flex flex-col`}>
          <div className="p-4 bg-gray-700 text-center">
            <button onClick={() => setSidebarLeft(!sidebarLeft)} className="text-red-200 hover:text-white">
              {sidebarLeft ? <FiChevronLeft size={24} /> : <FiChevronRight size={24} />}
            </button>
          </div>

          {/* Databases Tree Structure */}
          <div className="mt-6">
            <h2 className="text-xl font-bold mb-2">Databases</h2>
            {sampleData.length === 0 ? (
              <p className="text-gray-400">No databases available</p>
            ) : (
              sampleData.map((db, index) => (
                <TreeNode key={index} node={db} />
              ))
            )}
          </div>
        </div>
        
        {/* Main Content */}
        <div className="flex-1 p-8">
          {children}
        </div>
      </div>
    </div>
  );
};

export default AppLayout;
