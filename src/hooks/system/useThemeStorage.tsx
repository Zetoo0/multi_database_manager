import { getThemeStore} from "../../store/Store";
import { useQuery, useMutation } from "@tanstack/react-query";

const fetchThemeInfoList = async () => {
    console.log("Fetching....");
    const store = await getThemeStore();
    const data = await store.get('theme');
    console.log("Fetch theme data: ",data);
    return data;
}

const saveTheme = async (themeInfo: Record<string,string>) => {
    try{
        const store = await getThemeStore();
        console.log("Saving theme: ", themeInfo);
       // const themeArr = Object.values(themeInfo);

        await store.set('theme', themeInfo);
        await store.save();
        return "Success"
    }catch(e){
        console.error(e);
    }
}


export const useThemeInfoStorage =  () => {
    const {
        status,
        data:themeInfoList,
        error,
    } = useQuery({
        queryKey: ["themeInfoList"],
        queryFn: fetchThemeInfoList,
        staleTime:1000*60*60, //1000 * 60 * 60,
        refetchOnWindowFocus: false
    });
    return {status,themeInfoList: themeInfoList,error};
}

export const useExecuteSaveTheme = () => {
    const {mutate, isSuccess} = useMutation({        
        mutationFn:({themeInfo} : {themeInfo:Record<string,string>}) => saveTheme(themeInfo)
        
    });
    return {saveThemeData: mutate, isSuccessSaveTheme: isSuccess};
}