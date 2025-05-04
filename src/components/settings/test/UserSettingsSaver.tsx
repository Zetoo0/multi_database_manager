import { Button } from "../../ui/Button";
import { cn } from "../../../utils/tailwindUtils";
import { useTranslation } from "react-i18next";
import {saveUserSettings} from "../../../hooks/system/useSaveUserSettings";

export const UserSettingsSaver = () => {
  const { t } = useTranslation("settings");

  const save = async () => {
    const settings = {
      "autosavePeriod": 1000,
    };
    await saveUserSettings(settings);
  }
  
  return (
    <div className={cn("flex flex-col")}>
      <Button variant={"secondary"} onClick={save}>{t("test.saveUserSettings.log")}</Button>
    </div>
  );
};