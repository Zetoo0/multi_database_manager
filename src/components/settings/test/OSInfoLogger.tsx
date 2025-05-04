import { Button } from "../../ui/Button";
import { cn } from "../../../utils/tailwindUtils";
import { useTranslation } from "react-i18next";
import { useSystemInfo } from "../../../hooks/system/useSystemInfo";


export const OSInfoLogger = () => {
  const { t } = useTranslation("settings");
  const { systemInfo } = useSystemInfo();
  
  function logUserSettings() {
    console.log(systemInfo);
  }

  return (
    <div className={cn("flex flex-col")}>
      <Button variant={"secondary"} onClick={logUserSettings}>{t("test.osInfo.log")}</Button>
    </div>
  );
};