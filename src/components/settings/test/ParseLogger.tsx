import { Button } from "../../ui/Button";
import { cn } from "../../../utils/tailwindUtils";
import { useTranslation } from "react-i18next";
import { useParsed } from "../../../hooks/system/useParse";

export const ParseLogger = () => {
  const { t } = useTranslation("settings");
  const { parsed } = useParsed();
  
  function logUserSettings() {
    console.log(parsed);
  }

  return (
    <div className={cn("flex flex-col")}>
      <Button variant={"secondary"} onClick={logUserSettings}>{t("test.parsed.log")}</Button>
    </div>
  );
};