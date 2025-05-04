import { getCurrentWindow, type Window } from "@tauri-apps/api/window";
import { createContext, ReactNode, useContext } from "react";

type AppWindowContextType = {
  window: Window;
};

const AppWindowContext = createContext<AppWindowContextType | null>(null);

export const useAppWindow = () => {
  const context = useContext(AppWindowContext);
  if (!context) {
    throw new Error("useAppWindow must be used within a AppWindowProvider");
  }
  return context;
};

interface AppWindowProviderProps {
  children: ReactNode;
} 

export const AppWindowProvider = ({ children }: AppWindowProviderProps) => {
  const window = getCurrentWindow();

  return (
    <AppWindowContext.Provider value={{ window }}>
      {children}
    </AppWindowContext.Provider>
  );
};
