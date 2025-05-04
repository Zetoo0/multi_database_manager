import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

type ResponseBody = {
    data: object;
    error: object | null;
};

const getUserSettings = async (): Promise<object> => {
    return await invoke<ResponseBody>("get_user_settings").then(resp => resp.data);
}

export const useUserSettings = () => {
    const [userSettings, setUserSettings] = useState({});
    
    useEffect(() => {
        getUserSettings()
            .then((info) => {
                setUserSettings(info);
            });
    }, []);

    return {
        userSettings,
    };
}