import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
const getOSTypes = async (): Promise<string> => {
    return await invoke("get_os_types");
}

export const useOSTypes = () => {
    const [osTypes, setOSTypes] = useState("");
    
    useEffect(() => {
        getOSTypes()
            .then(info => JSON.parse(info))
            .then((info) => {
                setOSTypes(info);
            });
    }, []);

    return {
        osTypes,
    };
}