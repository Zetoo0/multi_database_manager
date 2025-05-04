import { Button } from "../../ui/Button";
import { cn } from "../../../utils/tailwindUtils";

import { useTranslation } from "react-i18next";
import { useDefaultUserSettings } from "../../../hooks/system/useDefaultUserSettings";


export const DefaultUserSettingsLogger = () => {
  const { t } = useTranslation("settings");
  const { userSettings } = useDefaultUserSettings();
  
  function logUserSettings() {
    console.log(userSettings);
  }

  return (
    <div className={cn("flex flex-col")}>
      <Button variant={"secondary"} onClick={logUserSettings}>{t("test.userSettings.log")}</Button>
    </div>
  );
};