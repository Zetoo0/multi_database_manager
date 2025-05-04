import { createFileRoute } from "@tanstack/react-router";
import { SettingsSubMenu } from "../../../components/settings/SettingsSubMenu";
import { useTranslation } from "react-i18next";
import { SettingsSubGroup } from "../../../components/settings/SettingsSubGroup";
import { ReadingDirectionSelect } from "../../../components/settings/appearance/ReadinDirectionSelect";
import { MenuBarSwitcher } from "../../../components/settings/appearance/MenuBarSwitcher";
import { EditorThemeSelect } from "../../../components/settings/appearance/EditorThemeSelect";
import { ApplicationColorChanger } from "../../../components/settings/appearance/ApplicationColorChanger";

export const Route = createFileRoute("/settings/_menus/appearance")({
  component: AppearanceSettings,
});

function AppearanceSettings() {
  const { t } = useTranslation("settings");

  return (
    <SettingsSubMenu title={t("appearance.title")}>
      <SettingsSubGroup title={t("appearance.subtitle")}></SettingsSubGroup>

      <SettingsSubGroup title={t("appearance.theme.title")}>
        <EditorThemeSelect />
      </SettingsSubGroup>

      <SettingsSubGroup title="Color Changer">
        <ApplicationColorChanger />
      </SettingsSubGroup>

      <SettingsSubGroup title={t("appearance.layout.title")}>
        <MenuBarSwitcher />
      </SettingsSubGroup>

      <SettingsSubGroup
        title={t("appearance.dir.title")}
        help={t("appearance.dir.help")}
      >
        <ReadingDirectionSelect />
      </SettingsSubGroup>
    </SettingsSubMenu>
  );
}
