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
    type: "database",
    children: [
      {
        name: "Tables",
        type: "folder",
        children: [
          {
            name: "Users",
            type: "table",
            children: [
              { name: "id", type: "column" },
              { name: "username", type: "column" },
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
        name: "Views",
        type: "folder",
        children: [{ name: "UserView", type: "view" }],
      },
      {
        name: "Stored Procedures",
        type: "folder",
        children: [{ name: "GetUserOrders", type: "stored_procedure" }],
      },
      {
        name: "Functions",
        type: "folder",
        children: [{ name: "CalculateTax", type: "function" }],
      },
    ],
  },
  {
    name: "Database2",
    type: "database",
    children: [
      {
        name: "Tables",
        type: "folder",
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
      {
        name: "Views",
        type: "folder",
        children: [{ name: "ProductView", type: "view" }],
      },
    ],
  },
];


function App() {
  return (
    <AppLayout sampleData={sampleData}>
      <ConnectionForm/>
    </AppLayout>
  );
}

export default App;
