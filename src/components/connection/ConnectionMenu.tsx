import { GroupedList } from "../ui/GroupedList";
import { useTranslation } from "react-i18next";

export const ConnectionMenu = () => {
    const {t} = useTranslation("form");
    return (
        <div className="size-full bg-background">
        <GroupedList
          className="ml-auto w-[200px] rtl:ml-0 rtl:mr-auto"
          innerClassName="pr-2 rtl:pr-0 rtl:pl-2"
        >
          <div data-tauri-drag-region className="min-h-7 w-full"></div>

          <GroupedList.Group title={t("connection.connections")} hideSeparator>
            <GroupedList.GroupItem title={t("connection.connectionList")}  to="/connect/connections" />
            <GroupedList.GroupItem
              title={t("connection.newConnection")}
              to="/connect/valamiConnection"
            />
          </GroupedList.Group>
        </GroupedList>
      </div>
    )

}