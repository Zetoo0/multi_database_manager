import { message, open } from '@tauri-apps/plugin-dialog';
import {readFile,writeFile} from "@tauri-apps/plugin-fs"
import {appLocalDataDir} from "@tauri-apps/api/path"





/*const openFile = async (path: string): Promise<string> => {
    const status: string = await invoke("open_file", { path });
    
    return status;
};*/

export const useFileOpenDialog = async () =>  {
    try {
        const selected: string | null= await open({
            multiple: false,
            directory: false,
            canCreateDirectories: true,
            title: "Open file",
            filters: [{ name: 'All Files', extensions: ['*'] }]
        });
        console.log(selected);
        if (!selected) {
            return "";
        }

        const imgData = await readFile(selected);
        const imgAsB64 = arrayBufferToBase64(imgData);
        console.log("Image as Base64: ",imgAsB64);
        const appDir = await appLocalDataDir();
        const newPath = `${appDir}/${selected.split("/").pop()}`;
        await writeFile(newPath, imgData);
        console.log("New path: ",newPath);
       // const status = await openFile(selected);
        //console.log(status);
        const imgUrl = `data:image/jpeg;base64,${imgAsB64}`;
        return imgUrl;//`file://${newPath}`;
    } catch (error:any) {
        message(error["E"], {title:"Error Open FIle Dialog", kind:'error'})
        return "";
    }
}

export const useFileOpenDialogCreateDatabase = async () =>  {
    try {
        const selected: string | null= await open({
            multiple: false,
            directory: false,
            canCreateDirectories: true,
            title: "Open file",
            filters: [{ name: 'All Files', extensions: ['*'] }]
        });
        console.log(selected);
        let decoder: TextDecoder = new TextDecoder("utf-8");
        const fileData = await readFile(selected??"");
        const decoded = decoder.decode(fileData);
        console.log("File data: ",decoded);
        if (!selected) {
            return "";
        }
        return decoded;
    } catch (error:any) {
      await message(error["E"], {title:"Error Open FIle Dialog", kind:'error'})
      return "";
    }
}

export const useFileOpenDialogGetPath = async () =>  {
    try {
        const selected: string | null= await open({
            multiple: false,
            directory: false,
            canCreateDirectories: true,
            title: "Open file",
            filters: [{ name: 'All Files', extensions: ['*'] }]
        });
        console.log("path: ",selected);
        if (!selected) {
            return "";
        }
        return selected;
    } catch (error:any) {
      await message(error["E"], {title:"Error Open FIle Dialog", kind:'error'})
      return "";
    }
}

function arrayBufferToBase64(buffer: any) {
  let binary = '';
  const bytes = new Uint8Array(buffer);
  const len = bytes.byteLength;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return btoa(binary);
}

/*
export const useFileOpenDialog = () => {
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const openDialog = useCallback(async () => {
    try {
      const result = await open({
        multiple: false,
        directory: false,
        canCreateDirectories: true,
        title: "Open file",
        filters: [{ name: 'All Files', extensions: ['*'] }]
      });

      if (result) {
        setSelectedFile(result as string); // Assuming it's always a string
      } else {
        setSelectedFile(null); // No file selected
      }
    } catch (err) {
      console.error("Error opening file dialog:", err);
      setError((err as Error).message || "An unknown error occurred");
    }
  }, []);

  return {
    selectedFile,
    error,
    openDialog
  };
};
*/
    