import { useObject } from "../../providers/ObjectProvider"
import { GroupedList } from "../ui/GroupedList"


export const ObjectMenu = () => {
    const {objectCreationInfo,editObject} = useObject()


    return (
        <div className="size-full bg-background">
        <GroupedList
          className="ml-auto w-[200px] rtl:ml-0 rtl:mr-auto"
          innerClassName="pr-2 rtl:pr-0 rtl:pl-2"
        >
          <div data-tauri-drag-region className="min-h-7 w-full"></div>

          <GroupedList.Group title="Create/Edit" hideSeparator>
            <GroupedList.GroupItem title="General"  />
            <GroupedList.GroupItem
              title="Definitions"
              to="/object/definitions"
            />
            {objectCreationInfo?.type == "table" || editObject?.type == "table" ? (
                <GroupedList.GroupItem
                    title="Columns"
                    to="/object/columns"
                />
            ) : null}
            {
                objectCreationInfo?.type == "function" || editObject?.type == "function" || editObject?.type == "procedure" || objectCreationInfo?.type == "function" ? (
                    <div>
                        <GroupedList.GroupItem
                            title="Parameters"
                           // to="/object/parameters"
                        />
                        <GroupedList.GroupItem
                            title="Code"
                            to="/object/code"
                        />
                    </div>
                ) : null
            }
          </GroupedList.Group>
        </GroupedList>
      </div>
    )

}