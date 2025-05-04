import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

type ResponseBody = {
    data: object;
    error: object | null;
};

const getDefaultUserSettings = async (): Promise<object> => {
    return await invoke<ResponseBody>("get_default_user_settings").then(resp => resp.data);
}

export const useDefaultUserSettings = () => {
    const [userSettings, setUserSettings] = useState({});
    
    useEffect(() => {
        getDefaultUserSettings()
            .then((info) => {
                setUserSettings(info);
            });
    }, []);

    return {
        userSettings,
    };
}