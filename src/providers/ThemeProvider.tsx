import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import { useExecuteSaveTheme, useThemeInfoStorage } from "../hooks/system/useThemeStorage";

const DEFAULT_THEME = {
  background: "39 52% 90%",
  foreground: "20 15% 20%",
  primary: "34 53% 45%",
  "primary-foreground": "39 52% 95%",
  secondary: "39 30% 80%",
  "secondary-foreground": "20 15% 10%",
  accent: "34 50% 50%",
  "accent-foreground": "39 52% 90%",
  border: "39 30% 60%",
}

type ThemeContextType = {
  isDark: boolean;
  setIsDark: (isDark: boolean) => void;
  toggleDarkMode: () => void;
  theme: {"--background": string; "--foreground": string; "--primary": string};
  setTheme: (theme: {"--background": string; "--foreground": string; "--primary": string}) => void;
  editorTheme: string,
  setEditorTheme: (theme: string) => void,
  editorImage: string,
  setEditorImage: (image: string) => void,
  updateThemeInfo:(key:string,value:string) => void,
  themeInfo: any
  saveTheme:() => void,
};

const ThemeContext = createContext<ThemeContextType | null>(null);

export const useTheme = () => {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error("useTheme must be used within a ThemeProvider");
  }
  return context;
};

interface ThemeProviderProps {
  children: ReactNode;
}

export const ThemeProvider = ({ children }: ThemeProviderProps) => {
  const [isDark, setIsDark] = useState(true);
  const [theme, setTheme] = useState({
    "--background": "#fff",
    "--foreground": "#fff",
    "--primary": "#3b82f6",
  });
  const [editorTheme, setEditorTheme] = useState("github");
  const [editorImage, setEditorImage] = useState("");
 // const {themeInfoList} = useThemeInfoStorage();
  const {saveThemeData} = useExecuteSaveTheme();
  const {themeInfoList} = useThemeInfoStorage();
  const [themeInfo,setThemeInfo] = useState<Record<string,string>>(DEFAULT_THEME);

  const toggleDarkMode = () => {
    setIsDark((prev) => {
      document.body.classList.toggle("dark", !prev);
      return !prev;
    });
  };

  const updateThemeInfo = (key:string,value:string) => {
    document.documentElement.style.setProperty(`--${key}`, value);
    setThemeInfo((prev) => ({ ...prev, [key]: value }));
   /* console.log("Theme update key and value: ",key,value);
    console.log("updating theme, old: ", themeInfo);
    setThemeInfo((prev) => ({ ...prev, [key]: value }));
    document.documentElement.style.setProperty(`--${key}`, value);
    console.log("new theme info: ", themeInfo);*/
  }

  const saveTheme = () => {
    console.log("Saving theme info: ",JSON.stringify(themeInfo));
    saveThemeData({themeInfo});  
  }

  const loadTheme = () => {
    console.log("Loading theme info: ",themeInfoList);
    if(themeInfoList && themeInfoList !== themeInfo){
        let themeInfoL:Record<string,string> = themeInfoList as Record<string,string>;
        for(const key in themeInfoL){
          updateThemeInfo(key,themeInfoL[key]);
        }
        setThemeInfo(themeInfoList as Record<string,string>);
    }
   // console.log("Loading theme info: ",JSON.stringify(themeInfo));
  }

  useEffect(() => {
    loadTheme();
  /* // loadTheme();
  //   document.body.classList.toggle("dark", isDark);
  //console.log("Theme info: ",themeInfoList);
  /*Object.keys(themeInfo).forEach((key) => {
    document.documentElement.style.setProperty(`--${key}`, themeInfo[key]);
  });*/
    
  }, [themeInfo,themeInfoList,isDark]);

  return (
    <ThemeContext.Provider value={{ isDark, setIsDark, toggleDarkMode,theme, setTheme, editorTheme, setEditorTheme,editorImage,setEditorImage,updateThemeInfo,themeInfo,saveTheme}}>
      {children}
    </ThemeContext.Provider>
  );
};

