import ConnectionForm from './Components/Forms/ConnectionForm.tsx';
import { React } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import AppLayout from './Components/AppLayout.tsx';
import SqlEditor from './Components/Editor.tsx';



function App() {
  return (
    <Router>
      <AppLayout>
        <Routes>
         {<Route path="/" element={<ConnectionForm />} />}
          {<Route path="/editor" element={<SqlEditor />} />}
        </Routes>
      </AppLayout>
    </Router>

  );
}

export default App;
