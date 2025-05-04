import { Search } from "lucide-react";
import { GroupedList } from "../ui/GroupedList";
import { useTranslation } from "react-i18next";

export const MigrationMenu = () => {
  const { t } = useTranslation("form");

  return (
    <div className="size-full">
      <GroupedList
        className="ml-auto w-[200px] rtl:ml-0 rtl:mr-auto"
        innerClassName="pr-2 rtl:pr-0 rtl:pl-2"
      >
        <div data-tauri-drag-region className="min-h-7 w-full"></div>

        <li className="mx-4 mb-4 flex h-8 items-center rounded border bg-background px-2">
          <Search className="ml-auto size-4 text-muted-foreground rtl:ml-0 rtl:mr-auto" />
        </li>

        <GroupedList.Group title={t("migration.migration")} hideSeparator>
          <GroupedList.GroupItem title={t("migration.migrationForm")} to="/migration/migrationPage" />
        </GroupedList.Group>
      </GroupedList>
    </div>
  );
};
