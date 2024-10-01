import Layout from './Layout'; // Import the Layout component
import React, { useState } from 'react';
import { useEffect } from 'react';
import AceEditor from 'react-ace';
import 'ace-builds/src-noconflict/theme-nord_dark';
import 'ace-builds/src-noconflict/mode-sql';
import 'ace-builds/src-noconflict/mode-mysql';
import 'ace-builds/src-noconflict/mode-pgsql';
import { Ace } from 'ace-builds';

const SqlEditor: React.FC = () => {
    const [code, setCode] = useState<string>("SELECT * FROM table");
      
  return (
    <Layout title="Editor">
        <div className="w-full h-96 p-4 border border-gray-300">
      {/* Editor container */}
      <div id="editor-container">
        <AceEditor 
          mode={'pgsql'}
          theme='nord_dark'
          name='sql_editor'
          width='100%'
          height='350px'
        />
      </div>
    </div>
    </Layout>
  );
};

export default SqlEditor;