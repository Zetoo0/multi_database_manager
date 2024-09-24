import ConnectionForm from './Components/Forms/ConnectionForm.tsx';
import { React } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import AppLayout from './Components/AppLayout.tsx';
import SqlEditor from './Components/Editor.tsx';
import Sidebar from './Components/Sidebar.tsx';
import { Database } from './Components/TreeNode.tsx';


const sampleData = [
  {
    name: "Database1",
    children: [
      {
        name: "Tables",
        children: [
          {
            name: "Users",
            children: [
              { name: "id" },
              { name: "username" },
              { name: "password" },
            ],
          },
          {
            name: "Orders",
            children: [
              { name: "order_id" },
              { name: "user_id" },
              { name: "total_amount" },
            ],
          },
        ],
      },
      {
        name: "Views",
        children: [{ name: "UserView" }],
      },
      {
        name: "Stored Procedures",
        children: [{ name: "GetUserOrders" }],
      },
      {
        name: "Functions",
        children: [{ name: "CalculateTax" }],
      },
      {
        name: "Indexes",
        children: [
          { name: "idx_users_username" },
        ],
      },
      {
        name: "Triggers",
        children: [
          { name: "before_insert_users" },
        ],
      },
      {
        name: "Roles and Users",
        children: [
          { name: "admin" },
          { name: "editor" },
        ],
      },
      {
        name: "Languages",
        children: [
          { name: "plpgsql" },
        ],
      },
      {
        name: "Constraints",
        children: [
          { name: "chk_users_username" },
        ],
      },
    ],
  },  {
    name: "Database2",
    children: [
      {
        name: "Tables",
        children: [
        ],
      },
      {
        name: "Views",
        children: [],
      },
      {
        name: "Stored Procedures",
        children: [],
      },
      {
        name: "Functions",
        children: [],
      },
      {
        name: "Indexes",
        children: [
          
        ],
      },
      {
        name: "Triggers",
        children: [
        
        ],
      },
      {
        name: "Roles and Users",
        children: [
        ],
      },
      {
        name: "Languages",
        children: [
        
        ],
      },
      {
        name: "Constraints",
        children: [
        
        ],
      },
    ],
  },
];

function App() {
  return (
    <AppLayout sampleData={ sampleData }>
      <ConnectionForm/>
      <SqlEditor/>
    </AppLayout>
  );
}

export default App;
