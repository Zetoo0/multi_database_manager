import { createFileRoute } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { SettingsSubMenu } from "../../../components/settings/SettingsSubMenu";
import { SettingsSubGroup } from "../../../components/settings/SettingsSubGroup";

export const Route = createFileRoute("/settings/_menus/")({
  component: IndexSettings,
});

function IndexSettings() {
  const { t } = useTranslation("settings");

  return (
    <SettingsSubMenu title={t("general.title")}>
      <SettingsSubGroup title={t("general.subtitle")}></SettingsSubGroup>
    </SettingsSubMenu>
  );
}
