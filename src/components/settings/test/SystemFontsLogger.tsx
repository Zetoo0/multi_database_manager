import { Button } from "../../ui/Button";
import { cn } from "../../../utils/tailwindUtils";
import { useTranslation } from "react-i18next";
import { useSystemFonts } from "../../../hooks/system/useSystemFonts";

export const SystemFontsLogger = () => {
  const { t } = useTranslation("settings");
  const { systemFonts } = useSystemFonts();
  
  function logSystemFonts() {
    console.log(systemFonts);
  }

  return (
    <div className={cn("flex flex-col")}>
      <Button variant={"secondary"} onClick={logSystemFonts}>{t("test.systemFonts.log")}</Button>
    </div>
  );
};