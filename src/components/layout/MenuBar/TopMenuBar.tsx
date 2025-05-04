import { useTranslation } from "react-i18next";
import {
  MenuBarRoot,
  MenuBarTrigger,
  MenuItem,
  MenuLink,
  MenuSeparator,
  MenuSubMenu,
} from "../../ui/MenuBar";
import { useNavigate } from "@tanstack/react-router";
import { useHotkey } from "../../../hooks/hotkey/useHotkey";
import { useLayout } from "../../../providers/LayoutProvider";
import { useObject } from "../../../providers/ObjectProvider";
import { useMetadata } from "../../../providers/MetadataProvider";

export const TopMenuBar = () => {
  const { t } = useTranslation("menubar");
  const navigate = useNavigate();
  const { readingDirection } = useLayout();
  const {setObjectCreationInfo} = useObject();

  const openSettings = () => {
    navigate({ to: "/settings" });
  };

  const openCreateDatabase = () => {
    setObjectCreationInfo({
      type: "database", database_name: "",
      schema_name: "" 
    });
    navigate({ to: "/object/definitions" });
  };

  const { formatted: settingsKey } = useHotkey("openSettings", openSettings);
  const {formatted: createDatabaseKey} = useHotkey("createDatabase", openCreateDatabase)
  const {refetchDatabaseMetadata} = useMetadata();

  const handleRefresh = () => {
    refetchDatabaseMetadata();
  };

  return (
    <MenuBarRoot dir={readingDirection}>
      <MenuBarTrigger title={t("file")}>
        <MenuItem title={t("newFile")} shortcut="⌘ N" />
        <MenuItem title={t("openFile")} shortcut="⌘ O" />

        <MenuSeparator />

        <MenuLink title={t("createDatabase")} shortcut={createDatabaseKey} to="/object/definitions"/>
       // <MenuItem title={t("refreshMetadata")} onClick={handleRefresh} shortcut="⌘ R"/>
        <MenuLink title={t("settings")} to="/settings" shortcut={settingsKey} />
      </MenuBarTrigger>

      <MenuBarTrigger title={t("edit")}>
        <MenuItem title={t("undo")} />
        <MenuItem title={t("redo")} />

        <MenuSeparator />

        <MenuSubMenu title={t("find")}>
          <MenuItem title={t("cut")} />
          <MenuItem title={t("copy")} />
          <MenuItem title={t("paste")} />
        </MenuSubMenu>

        <MenuSeparator />

        <MenuItem title={t("cut")} />
        <MenuItem title={t("copy")} />
        <MenuItem title={t("paste")} />
      </MenuBarTrigger>
      <MenuBarTrigger title={t("tools")}>
        <MenuItem title={t("query")} />
        <MenuLink title="migration" to="/migration/migrationPage"/>
      </MenuBarTrigger>
      <MenuBarTrigger title={t("connection")}>
        <MenuLink title={t("newConnection")} to="/connect/valamiConnection"/>
        <MenuLink title={t("connections")} to="/connect/connections"/>
      </MenuBarTrigger>      
    </MenuBarRoot>
  );
};
