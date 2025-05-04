import { Settings, File, FolderOpen, Container, Logs } from "lucide-react";
import { useTheme } from "../../../providers/ThemeProvider";
import { MenuBarButton } from "./MenuBarButton";
import { TitleBarSpacer } from "../TitleBar/TitleBarSpacer";
import { cn } from "../../../utils/tailwindUtils";
import { useTranslation } from "react-i18next";

interface SideMenuBarProps {
  className?: string;
}

export const SideMenuBar = ({ className }: SideMenuBarProps) => {
  const { toggleDarkMode } = useTheme();
  const { t } = useTranslation("menubar");

  return (
    <TitleBarSpacer
      className={cn(
        "w-[66px] pb-[28px] flex-col flex h-full rtl:border-l ltr:border-r bg-accent",
        className
      )}
    >
      <MenuBarButton icon={<File />} label={t("newFile")} />
      <MenuBarButton
        icon={<FolderOpen />}
        onClick={toggleDarkMode}
        label={t("openFile")}
      />
      <MenuBarButton icon={<Container />} label={"Connection"} to="/connect/valamiConnection" />
      <MenuBarButton icon={<Logs />} label={"Migration"} to="/migration/migrationPage" />

      <MenuBarButton
        className="mt-auto"
        icon={<Settings />}
        to="/settings"
        label={t("settings")}
      />
    </TitleBarSpacer>
  );
};
