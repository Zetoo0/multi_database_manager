import { Button } from "../../ui/Button";
import { cn } from "../../../utils/tailwindUtils";
import { useUserSettings } from "../../../hooks/system/useUserSettings";
import { useTranslation } from "react-i18next";


export const UserSettingsLogger = () => {
  const { t } = useTranslation("settings");
  const { userSettings } = useUserSettings();
  
  function logUserSettings() {
    console.log(userSettings);
  }

  return (
    <div className={cn("flex flex-col")}>
      <Button variant={"secondary"} onClick={logUserSettings}>{t("test.userSettings.log")}</Button>
    </div>
  );
};