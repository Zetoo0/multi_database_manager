import { createContext, ReactNode, useContext, useState } from "react";

export type MenuBarPosition = "left" | "top";

export type ReadingDirection = "ltr" | "rtl";

type LayoutContextType = {
  menuBarPos: MenuBarPosition;
  menuBarHidden: boolean;
  showStatusBar: boolean;
  showTitleBar: boolean;
  readingDirection: ReadingDirection;
  setMenuBarPos: (pos: MenuBarPosition) => void;
  setMenuBarHidden: (hidden: boolean) => void;
  setShowStatusBar: (showStatusBar: boolean) => void;
  setShowTitleBar: (showTitleBar: boolean) => void;
  setReadingDirection: (readingDirection: ReadingDirection) => void;
};

const LayoutContext = createContext<LayoutContextType | null>(null);

export const useLayout = () => {
  const context = useContext(LayoutContext);
  if (!context) {
    throw new Error("useLayout must be used within a LayoutProvider");
  }
  return context;
};

interface LayoutProviderProps {
  children: ReactNode;
}

export const LayoutProvider = ({ children }: LayoutProviderProps) => {
  const [menuBarPos, setMenuBarPos] = useState<MenuBarPosition>("top");
  const [menuBarHidden, setMenuBarHidden] = useState(false);

  const [showStatusBar, setShowStatusBar] = useState(true);
  const [showTitleBar, setShowTitleBar] = useState(true);
  const [readingDirection, setReadingDirection] =
    useState<ReadingDirection>("ltr");

  const handleDirChange = (dir: ReadingDirection) => {
    document.documentElement.dir = dir;
    setReadingDirection(dir);
  };



  return (
    <LayoutContext.Provider
      value={{
        menuBarPos,
        menuBarHidden,
        showStatusBar,
        showTitleBar,
        readingDirection,

        setMenuBarPos,
        setMenuBarHidden,
        setShowStatusBar,
        setShowTitleBar,
        setReadingDirection: handleDirChange,
      }}
    >
      {children}
    </LayoutContext.Provider>
  );
};
