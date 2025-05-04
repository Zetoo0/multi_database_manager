import { X, Minus, Copy } from "lucide-react";
import { useAppWindow } from "../../../providers/AppWindowProvider";
import { WindowActionButton } from "./WindowActionButton";
import { useLayout } from "../../../providers/LayoutProvider";
import { useSystemInfo } from "../../../hooks/system/useSystemInfo";
import { cn } from "../../../utils/tailwindUtils";
import { TopMenuBar } from "../MenuBar/TopMenuBar";

export interface TitleBarProps {
  height: string;
}

export const TitleBar = ({ height }: TitleBarProps) => {
  const { window } = useAppWindow();
  const { menuBarPos, showTitleBar } = useLayout();
  const { systemInfo } = useSystemInfo();

  const handleWindowDrag = (e: React.MouseEvent<HTMLDivElement>) => {
    if (e.buttons === 1) {
      // // Primary (left) button
      if (e.detail === 2) {
        window.toggleMaximize(); // Maximize on double click
      } else {
        window.startDragging(); // Else start dragging
      }
    }
  };

  return (
    <header
      onMouseDown={handleWindowDrag}
      className={cn("fixed top-0 flex w-full select-none items-center", {
        "border-b": showTitleBar && menuBarPos === "top",
        "rtl:pr-[66px] rtl:pl-0": menuBarPos === "left",
        "pl-[66px]": systemInfo?.os?.type === "Mac OS" || menuBarPos === "left",
      })}
      style={{ height }}
    >
      {menuBarPos === "top" && showTitleBar && <TopMenuBar />}

      {systemInfo?.os?.type !== "Mac OS" && (
        <div
          onMouseDown={(e) => e.stopPropagation()}
          className="ml-auto flex rtl:ml-0 rtl:mr-auto"
        >
          <WindowActionButton
            icon={<Minus />}
            onClick={() => window.minimize()}
          />
          <WindowActionButton
            icon={<Copy className="-scale-x-100" />}
            iconSize={16}
            onClick={() => window.toggleMaximize()}
          />
          <WindowActionButton
            icon={<X />}
            onClick={() => window.close()}
            variant="destructive"
          />
        </div>
      )}
    </header>
  );
};
