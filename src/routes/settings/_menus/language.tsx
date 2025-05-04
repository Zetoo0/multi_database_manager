import { createFileRoute } from "@tanstack/react-router";
import * as RadioGroup from "@radix-ui/react-radio-group";
import { LanguageCard } from "../../../components/settings/language/LanguageCard";
import { useTranslation } from "react-i18next";
import { Locale } from "../../../i18n";
import { SettingsSubGroup } from "../../../components/settings/SettingsSubGroup";
import { SettingsSubMenu } from "../../../components/settings/SettingsSubMenu";

export const Route = createFileRoute("/settings/_menus/language")({
  component: LanguageSettings,
});

function LanguageSettings() {
  const {
    t,
    i18n: { changeLanguage, language },
  } = useTranslation("settings");

  const handleLanguageSelect = (lang: Locale) => {
    changeLanguage(lang);
  };

  return (
    <SettingsSubMenu title={t("language.title")}>
      <SettingsSubGroup title={t("language.subtitle")}>
        <RadioGroup.Root
          asChild
          defaultValue={language}
          onValueChange={handleLanguageSelect}
        >
          <ul className="flex flex-col gap-2">
            {Object.values(Locale).map((lang) => (
              <LanguageCard key={lang} lang={lang} />
            ))}
          </ul>
        </RadioGroup.Root>
      </SettingsSubGroup>
    </SettingsSubMenu>
  );
}
