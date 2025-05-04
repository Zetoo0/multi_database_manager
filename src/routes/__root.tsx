import { createRootRoute, Outlet } from "@tanstack/react-router";
import { TitleBar } from "../components/layout/TitleBar/TitleBar";
import { StatusBar } from "../components/layout/StatusBar/StatusBar";
import { useLayout } from "../providers/LayoutProvider";
import { cn } from "../utils/tailwindUtils";
import { SideMenuBar } from "../components/layout/MenuBar/SideMenuBar";

export const Route = createRootRoute({
  component: RootLayout,
});

function RootLayout() {
  const { menuBarPos, menuBarHidden, showStatusBar } = useLayout();

  return (
    <>
      <TitleBar height="29px" />

      <div
        className={cn("size-full max-w-[100vw] overflow-hidden grid", {
          "grid-cols-[auto_1fr]": menuBarPos === "left",
        })}
      >
        {menuBarPos === "left" && (
          <SideMenuBar className={menuBarHidden ? "hidden" : "z-50"} />
        )}
        <div
          className={cn({
            "grid max-h-screen grid-rows-[1fr_24px]": showStatusBar,
          })}
        >
          <Outlet />
          <StatusBar className={showStatusBar ? "" : "hidden"} />
        </div>
      </div>
    </>
  );
}
