import Layout from './Layout'; // Import the Layout component
import React, { useState } from 'react';
import { useEffect } from 'react';
import { EditorState } from '@codemirror/basic-setup';
import { EditorView, basicSetup } from '@codemirror/basic-setup';
import { sql } from '@codemirror/lang-sql';
const SqlEditor: React.FC = () => {
    const [code, setCode] = useState<string>("SELECT * FROM table");
   

    useEffect(() => {
        const editor = new EditorView({
          state: EditorState.create({
            doc: 'SELECT * FROM users;',
            extensions: [basicSetup, sql()],
          }),
          parent: document.querySelector('#editor-container')!,
        });
    
        return () => {
          editor.destroy(); // Clean up the editor when the component unmounts
        };
      }, []);
      
  return (
    <Layout title="Editor">
        <div className="w-full h-96 p-4 border border-gray-300">
      {/* This will be the editor container */}
      <div id="editor-container"></div>
    </div>
    </Layout>
  );
};

export default SqlEditor;