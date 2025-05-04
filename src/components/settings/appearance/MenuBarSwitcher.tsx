import { MenuBarPosition, useLayout } from "../../../providers/LayoutProvider";
// import { Button } from "../../ui/Button";
import * as RadioGroup from "@radix-ui/react-radio-group";
import { cn } from "../../../utils/tailwindUtils";
import { useTranslation } from "react-i18next";

export const MenuBarSwitcher = () => {
  const { menuBarPos, setMenuBarPos } = useLayout();

  return (
    <RadioGroup.Root
      defaultValue={menuBarPos}
      onValueChange={setMenuBarPos}
      className="grid grid-cols-2 gap-2"
    >
      <PosCard pos="left" />
      <PosCard pos="top" />
    </RadioGroup.Root>
  );
};

const PosCard = ({ pos }: { pos: MenuBarPosition }) => {
  const { readingDirection } = useLayout();
  const { t } = useTranslation("settings");

  return (
    <div className=" flex size-full flex-col items-center">
      <RadioGroup.Item
        value={pos}
        id={`pos-${pos}`}
        className={cn(
          "h-fit overflow-hidden rtl:-scale-x-100 w-full rounded-lg border-2 bg-background peer group data-[state=checked]:border-primary"
        )}
      >
        <div className="p-2 group-data-[state=unchecked]:opacity-50">
          <div
            className={cn(
              "grid aspect-[16/9] w-full overflow-hidden rounded-md border-2",
              { "grid-cols-[1.5fr_10fr]": pos === "left" }
            )}
          >
            {pos === "left" && <div className="rounded-l bg-primary"></div>}
            <div className="grid grid-rows-[1fr_1fr_2fr_8fr] items-center">
              <div className="grid grid-cols-3 px-1">
                <div className="h-2 w-1/3 rounded-full bg-muted"></div>
                <div className="h-2 w-full rounded-full bg-muted"></div>
              </div>

              <div className="size-full bg-muted">
                {pos === "top" && (
                  <div className="h-full w-1/5 rounded-sm border-2 border-muted bg-primary"></div>
                )}
              </div>

              <div className="size-full p-1">
                <div className="size-full rounded-sm bg-muted"></div>
              </div>

              <div className="mx-auto h-full w-[53%] bg-muted"></div>
            </div>
          </div>
          {/* </div> */}
        </div>
      </RadioGroup.Item>

      <label
        htmlFor={`pos-${pos}`}
        className="mt-2 cursor-pointer peer-data-[state=unchecked]:text-border"
      >
        {t(
          `appearance.layout.${readingDirection === "rtl" && pos === "left" ? "leftRtl" : pos}`
        )}
      </label>
    </div>
  );
};
