import { useEffect } from 'react';
import { DatabaseConnection } from '../../../hooks/system/useConnectToDatabase';
import { useConnectionInfoStorage } from '../../../hooks/system/useConnectionInfoStorage'
import { Button } from '../../ui/Button';
import { useConnection } from '../../../providers/ConnectionProvider';
import { useTranslation } from 'react-i18next';
interface ConnectionListProps{
    onConnection: (connection_info: DatabaseConnection) => void;
}

export const ConnectionList = ({onConnection}:ConnectionListProps) => {
    const {connectionInfoList} = useConnectionInfoStorage();
    const {handleConnection} = useConnection();
    const {t} = useTranslation("common")

    useEffect(() => {
        console.log("Got connectionInfoList: ",connectionInfoList);
    },[connectionInfoList])

    const submitConnection = (connection_info: DatabaseConnection) => {
        console.log("Connecting to: ",connection_info);
        handleConnection(connection_info);
        onConnection(connection_info);
    }
    
    return (
        <div>
          {
            connectionInfoList !== undefined && connectionInfoList.map((connection,index) => (
                <div className='p-6 rounded-lg shadow-lg max-w-md mx-auto' key={index}>
                    <h3>{connection.server}</h3>
                    <div className="grid grid-cols-10 gap-4">
                        <div className="col-span-7">
                            <label className="block text-sm font-medium text-beige-700">
                                {connection.driver_type}
                            </label>
                        </div>
                    </div>
                    <Button onClick={() => submitConnection(connection)}>{t("actions.connect")}</Button>
                </div> 
            ))
          }
        </div>
    )
}