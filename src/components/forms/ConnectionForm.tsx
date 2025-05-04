import { useState } from 'react';
import { DatabaseConnection } from '../../hooks/system/useConnectToDatabase';
import { Input } from '../ui/Input';
import { Button } from '../ui/Button';
import { Select } from '../ui/Select';
import { useConnectionInfoStorageSave } from '../../hooks/system/useConnectionInfoStorageSave';
import { useConnection } from '../../providers/ConnectionProvider';
import { useTranslation } from 'react-i18next';
import { useFileOpenDialogGetPath } from '../../hooks/system/useFileOpenDialog';
interface ConnectionFormProps {
    onConnection: (connection_info: DatabaseConnection) => void;
    onMetadatasGet: (connection_info: DatabaseConnection) => void;
}

export const ConnectionForm = ({onConnection, onMetadatasGet} : ConnectionFormProps) => {

    const {handleConnection} = useConnection();
    const [server, setServer] = useState('');
    const [port, setPort] = useState('');
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [driverType, setDriverType] = useState('postgresql');
    const [file, setFile] = useState<string>('');
    const {saveData} = useConnectionInfoStorageSave();
    const {t} = useTranslation("form");
    //const [isConnected, setIsConnected] = useState(false);
   // const {metadatas, setMetadatas} = useMetadata();

    const openFileDialog = async () => {
        const path = await useFileOpenDialogGetPath();
        console.log("Path: ",path);
        setFile(path);
    }

    const submitConnection = () => {
        if(driverType === 'sqlite') {
            console.log("File: ",file);
            const connection_info = {
                server: file,
                port: "a",
                username: "a",
                password: "a",
                driver_type: driverType,
            };
            handleConnection(connection_info);
            onConnection(connection_info);
            onMetadatasGet(connection_info);
        }else{
            const connection_info = {
                server: server,
                port: port,
                username: username,
                password: password,
                driver_type: driverType,
            };
            handleConnection(connection_info);
            onConnection(connection_info);
            onMetadatasGet(connection_info);
        }

      //  setupConnectionInfoStorage(connection_info);

    }

    const saveConnection = () => {
        const connection_info: DatabaseConnection = {
            server: server,
            port: port,
            username: username,
            password: password,
            driver_type: driverType,
        };
        saveData(connection_info,{
            onSuccess: () => {
                console.log("Connection saved");
            },
            onError: () => {
                console.log("Error saving connection");
            }
        });
    }

    return(
        <div className="connection-form p-6 rounded-lg shadow-lg max-w-md mx-auto">
            <h2 className="text-2xl font-semibold text-beige-800 mb-4">{t("connection.databaseConnection")}</h2>
            <form onSubmit={(e) => { e.preventDefault(); submitConnection(); }} className="space-y-4">
                <div>
                    <label className="block text-sm font-medium text-beige-700">
                        {t("connection.driverType")}
                    </label>
                    <Select value={driverType} onValueChange={(e) => setDriverType(e)} options={[
                        "PostgreSQL",
                        "MySQL",
                        "SQLite",
                        "MSSQL",
                        "Oracle",
                    ]} className='w-full'/>
                </div>

                {driverType !== 'sqlite' && (
                    <>
                        <div className="grid grid-cols-10 gap-4">
                            <div className="col-span-7">
                                <label className="block text-sm font-medium text-beige-700">
                                    {t("connection.server")}
                                </label>
                                <Input type="text" value={server} onChange={(e) => setServer(e.target.value)}/>
                            </div>
                            <div className="col-span-3">
                                <label className="block text-sm font-medium text-beige-700">
                                    {t("connection.port")}
                                </label>
                                <Input type="number" value={port} onChange={(e) => setPort(e.target.value)}/>
                            </div>
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-beige-700">
                                {t("connection.username")}
                            </label>
                            {/*<input
                                type="text"
                                value={username}
                                onChange={(e) => setUsername(e.target.value)}
                                required
                                className="mt-1 p-2 w-full border border-gray-600 rounded-md focus:ring-indigo-500 focus:border-indigo-500 text-black"
                            />*/}
                            <Input type="text" value={username} onChange={(e) => setUsername(e.target.value)}/>
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-beige-700">
                                {t("connection.password")}
                            </label>
                            <Input type="password" value={password} onChange={(e) => setPassword(e.target.value)}/>
                        </div>
                    </>
                )}

                {driverType === 'sqlite' && (
                    <div>
                        <label className="block text-sm font-medium text-beige-700">
                            {t("connection.selectSqliteFile")}
                        </label>
                        <Button variant="outline" onClick={() => openFileDialog()} type='button'>Open</Button>
                    </div>
                )}

                <Button variant="default" type='submit' className='w-full'>{t("connection.connect")}</Button>
            </form>
            <Button variant="default" onClick={saveConnection} className='w-full padding-top'>{t("connection.saveConnection")}</Button>
        </div>
    )
}

//                        {//<Input type="file" accept=".sqlite, .db" onChange={(e) => setFile(e.target.files ? e.target.files[0] : null)} />}s
