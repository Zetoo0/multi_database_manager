import { Button } from "../../ui/Button";
import { cn } from "../../../utils/tailwindUtils";
import { useTranslation } from "react-i18next";
import { useOSTypes } from "../../../hooks/system/useOSTypes";


export const OSTypesLogger = () => {
  const { t } = useTranslation("settings");
  const { osTypes } = useOSTypes();
  
  function logUserSettings() {
    console.log(osTypes);
  }

  return (
    <div className={cn("flex flex-col")}>
      <Button variant={"secondary"} onClick={logUserSettings}>{t("test.osTypes.log")}</Button>
    </div>
  );
};