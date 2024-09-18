import ConnectionForm from './Components/Forms/ConnectionForm.tsx';
import { React } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import AppLayout from './Components/AppLayout.tsx';
import SqlEditor from './Components/Editor.tsx';



function App() {
  return (
    <div>
      <ConnectionForm />
    </div>
  );
}

export default App;
