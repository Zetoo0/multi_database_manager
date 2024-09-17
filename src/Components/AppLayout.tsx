import { useState } from "react";
import { FiDatabase, FiSettings, FiFileText, FiChevronLeft, FiChevronRight } from 'react-icons/fi';

interface AppLayoutProps{
    children: React.ReactNode;
  }

const AppLayout: React.FC<AppLayoutProps> = ({children}) => {
    const [sidebarLeft, setSidebarLeft] = useState<boolean>(true); // Sidebar state (left/right)

    return(
        <div className="flex h-screen bg-gray-800">
      {/* Sidebar */}
      <div className={`${sidebarLeft ? 'order-first' : 'order-last'} w-24 bg-gray-800 text-white flex flex-col`}>
        {/* Sidebar Toggle Button */}
        <div className="p-4 bg-gray-700 text-center">
          <button onClick={() => setSidebarLeft(!sidebarLeft)} className="text-red-200 hover:text-white">
            {sidebarLeft ? <FiChevronLeft size={24} /> : <FiChevronRight size={24} />}
          </button>
        </div>

        {/* Navigation Icons */}
        <div className="flex flex-col items-center py-4 space-y-6">
          <button className="text-gray-400 hover:text-white">
            <FiDatabase size={24} />
            <span className="block text-sm mt-2">Editor</span>
          </button>
          <button className="text-gray-400 hover:text-white">
            <FiFileText size={24} />
            <span className="block text-sm mt-2">Files</span>
          </button>
          <button className="text-gray-400 hover:text-white">
            <FiSettings size={24} />
            <span className="block text-sm mt-2">Settings</span>
          </button>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 p-8">
        {children}
      </div>
    </div>
    )
}

export default AppLayout