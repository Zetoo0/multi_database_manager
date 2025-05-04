import { useState } from 'react';
import { useEffect } from 'react';
import AceEditor from 'react-ace';
import 'ace-builds/src-noconflict/theme-nord_dark';
import 'ace-builds/src-noconflict/theme-monokai';
import 'ace-builds/src-noconflict/theme-ambiance';
import 'ace-builds/src-noconflict/theme-tomorrow_night';
import 'ace-builds/src-noconflict/theme-tomorrow_night_blue';
import 'ace-builds/src-noconflict/theme-tomorrow_night_bright';
import 'ace-builds/src-noconflict/theme-tomorrow_night_eighties';
import 'ace-builds/src-noconflict/theme-xcode';
import 'ace-builds/src-noconflict/theme-textmate';
import 'ace-builds/src-noconflict/theme-ambiance';
import 'ace-builds/src-noconflict/theme-chaos';
import 'ace-builds/src-noconflict/theme-chrome';
import 'ace-builds/src-noconflict/theme-clouds';
import 'ace-builds/src-noconflict/theme-cobalt';
import 'ace-builds/src-noconflict/theme-crimson_editor';
import 'ace-builds/src-noconflict/theme-dawn';
import 'ace-builds/src-noconflict/theme-dracula';
import 'ace-builds/src-noconflict/theme-dreamweaver';
import 'ace-builds/src-noconflict/theme-eclipse';
import 'ace-builds/src-noconflict/theme-github';
import 'ace-builds/src-noconflict/theme-gob';
import 'ace-builds/src-noconflict/theme-gruvbox';
import 'ace-builds/src-noconflict/theme-idle_fingers';
import 'ace-builds/src-noconflict/theme-kr_theme';
import 'ace-builds/src-noconflict/theme-kuroir';
import 'ace-builds/src-noconflict/theme-merbivore';
import 'ace-builds/src-noconflict/theme-monokai';
import 'ace-builds/src-noconflict/theme-pastel_on_dark';
import 'ace-builds/src-noconflict/theme-solarized_dark';
import 'ace-builds/src-noconflict/theme-solarized_light';
import 'ace-builds/src-noconflict/theme-sqlserver';
import 'ace-builds/src-noconflict/mode-sql';
import 'ace-builds/src-noconflict/mode-mysql';
import 'ace-builds/src-noconflict/mode-pgsql';
import { QueryInfo } from '../../Types/query/QueryInfo';
import { Button } from '../ui/Button';
import { useQuery } from '../../providers/QueryProvider';
import { useMetadata } from '../../providers/MetadataProvider';
import { Select } from '../ui/Select';
import { useTheme } from '../../providers/ThemeProvider';
import { useConnection } from '../../providers/ConnectionProvider';
import { useTranslation } from 'react-i18next';
interface SqlEditorProps {
    sqlType: string,
    value: string,
    onChange: (value: string) => void
}

export const SqlEditor = ({sqlType, value, onChange}:SqlEditorProps) => {
    const [code, setCode] = useState<string>("");
    const [_queryInfo, setQueryInfo] = useState<QueryInfo>();
    const {setSelectedConnection, selectedConnection } = useConnection();
    const {databaseMetadata} = useMetadata();
    const { handleQuery } = useQuery();
    const {editorTheme,editorImage} = useTheme();
    const {t} = useTranslation("common");

    const execute = async () => {
        let query_info:QueryInfo = {
            sql: code,
            db_name: selectedConnection ?? "",
            db_type: "postgresql"
        };
        setQueryInfo(query_info);
        console.log(query_info);
        handleQuery(query_info ?? {sql: "", db_name: "", db_type: ""});
    }   


    const onValueChange = (value: string) => {
        onChange(value);
        setCode(value);
    }

    useEffect(() => {
        if(databaseMetadata){
            //Object.keys(databaseMetadata).map((db) => console.log(db));
        }
        console.log("Editor image: ", editorImage);
    }, [value]);

    return(
        <div>
            <div>
                {
                    editorImage && (
                        <div
                        style={{
                            position: 'relative',
                            width: '100%',
                            backgroundImage: `url(${editorImage})`,
                            backgroundSize: 'cover',
                        }}
                        >
                            <AceEditor
                    mode={sqlType}
                    theme={editorTheme}
                    name="editor"
                    fontSize={16}
                    showPrintMargin={false}
                    showGutter={true}
                    highlightActiveLine={true}
                    value={value}
                    onChange={onValueChange}
                    style={{
                        backgroundColor: 'transparent',
                        width: '100%',      
                    }}
                />
                        </div>
                        
                    )
                }
                {!editorImage && <AceEditor
                    mode={sqlType}
                    theme={editorTheme}
                    name="editor"
                    fontSize={16}
                    showPrintMargin={false}
                    showGutter={true}
                    highlightActiveLine={true}
                    value={value}
                    onChange={onValueChange}
                    style={{
                       width: 'full' 
                                        }}
                    
                />}
            </div>
            <Button onClick={execute} variant={"default"}>{t("actions.execute")}</Button>
            {
                    databaseMetadata && (
                        <div>
                            <label>{t("database")}</label>
                            <Select options={Object.keys(databaseMetadata).map((db) => db)} onValueChange={(e) => setSelectedConnection(e)} value={selectedConnection ?? ""}/>
                        </div>
                    )
            }

        </div>
    )



};