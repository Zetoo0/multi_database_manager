import { createFileRoute } from "@tanstack/react-router";
import { SettingsSubMenu } from "../../../components/settings/SettingsSubMenu";
import { SettingsSubGroup } from "../../../components/settings/SettingsSubGroup";
import { useTranslation } from "react-i18next";

export const Route = createFileRoute("/settings/_menus/about")({
  component: AboutSettings,
});

function AboutSettings() {
  const { t } = useTranslation("settings");

  return (
    <SettingsSubMenu title={t("about.title")}>
      <SettingsSubGroup title={t("about.subtitle")}></SettingsSubGroup>
    </SettingsSubMenu>
  );
}
