import { invoke } from "@tauri-apps/api/core";
// import { useEffect, useState } from "react";

type ResponseBody = {
    data: object;
    error: object | null;
};

export const saveUserSettings = async (settings: object): Promise<object> => {
    return await invoke<ResponseBody>("save_user_settings", {settings}).then(resp => resp.data);
}

// export const useSaveUserSettings = () => {
//     const [userSettings, setUserSettings] = useState({});
    
//     useEffect(() => {
//         saveUserSettings()
//             .then((info) => {
//                 setUserSettings(info);
//             });
//     }, []);

//     return {
//         userSettings,
//     };
// }