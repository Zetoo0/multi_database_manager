import { useState } from "react";
import { FiDatabase, FiSettings, FiFileText, FiChevronLeft, FiChevronRight } from 'react-icons/fi';
import { FaTable, FaColumns, FaFolder } from 'react-icons/fa';
import { Database } from './TreeNode';
import TreeNode from "./TreeNode";

const AppLayout: React.FC<{ children: React.ReactNode , sampleData:Database[]}> = ({ children, sampleData }) => {
  const [sidebarLeft, setSidebarLeft] = useState<boolean>(true);  
  
  return (
    <div className="flex flex-col h-screen bg-gray-800">
      {/* Navigation Bar */}
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

          {/* Databases Structure */}
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
        
        {/* Content */}
        <div className="flex-1 p-8">
          {children}
        </div>
      </div>
    </div>
  );
};

export default AppLayout;
