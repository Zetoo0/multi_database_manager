import React from 'react';
import { useState } from 'react';
import { FaTable, FaColumns, FaFolder } from 'react-icons/fa';
import { FiDatabase } from 'react-icons/fi';

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

type RoleAndUser = {
  name: string;
  type: 'role_and_user';
  children: {name:string;type: 'parameter'}[];
}

type Index = {
  name:string;
  type:'index';
}

type Trigger = {
  name:string;
  type:'trigger';
}

type Language = {
  name:string;
  type:'language';
}

type MateralizedView = {
  name:string;
  type:'materalized_view';
}

type UserGivenType = {
  name:string;
  type:'user_given_type';
}

type ForeignDataWrapper = {
  name:string;
  type:'foreign_data_wrapper';
}

type Constraint = {
  name:string;
  type:'constraint';
}

type Lock = {
  name:string;
  type:'lock';
}



type Function = {
  name: string;
  type: 'function';
  children: { name: string; type: 'parameter' }[];
}

export type Database = {
  name: string;
  type: 'database';
  children: (Table | View | Procedure | Function | Index | Lock | ForeignDataWrapper | UserGivenType | MateralizedView | RoleAndUser | Language | Constraint | Trigger)[];
};
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

export default TreeNode;