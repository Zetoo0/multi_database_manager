import { load } from "@tauri-apps/plugin-store";
import { appConfigDir,appDataDir, appLocalDataDir} from "@tauri-apps/api/path"

// const connectionStore = await load('connecitonInfos.json');
//export const themeStore = await load('theme.json');

export const getConnectionStore = async () => {
    const connectionStore = await load('connecitonInfos.json');
    return connectionStore;
}

export const getThemeStore = async () => {
    const themeStore = await load('theme.json');
    return themeStore;
}

export const logAppDataDir = async () => {
    const dataDir = await appDataDir();
    console.log(dataDir);
}

export const logAppConfigDir = async () => {
    const configDir = await appConfigDir();
    console.log(configDir);
}

export const logAppLoacalDataDir = async () => {
    const configDir = await appLocalDataDir();
    console.log(configDir);
}