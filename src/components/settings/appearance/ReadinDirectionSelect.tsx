import { ReadingDirection, useLayout } from "../../../providers/LayoutProvider";
import * as RadioGroup from "@radix-ui/react-radio-group";
import { cn } from "../../../utils/tailwindUtils";
import { useTranslation } from "react-i18next";

export const ReadingDirectionSelect = () => {
  const { readingDirection, setReadingDirection } = useLayout();

  const handleSelect = (value: ReadingDirection) => {
    setReadingDirection(value);
  };

  return (
    <RadioGroup.Root
      defaultValue={readingDirection}
      onValueChange={handleSelect}
      className="grid grid-cols-2 gap-2"
    >
      <DirCard dir="ltr" /> 
      <DirCard dir="rtl" />
    </RadioGroup.Root>
  );
};

const DirCard = ({ dir }: { dir: ReadingDirection }) => {
  const { t } = useTranslation("settings");
  return (
    <div className=" flex size-full flex-col items-center">
      <RadioGroup.Item
        value={dir}
        id={`dir-${dir}`}
        className={cn(
          "h-fit overflow-hidden w-full rounded-lg border-2 bg-background peer group data-[state=checked]:border-primary",
          {
            "-scale-x-100": dir === "rtl",
          }
        )}
      >
        <div className="p-2 group-data-[state=unchecked]:opacity-50">
          <div className="grid aspect-[16/9] w-full grid-cols-[1.5fr_10fr] rounded-md border-2">
            <div className="bg-muted"></div>

            <div className="grid grid-rows-[1fr_1fr_2fr_8fr] items-center">
              <div className="grid grid-cols-3 px-1">
                <div className="h-2 w-1/3 rounded-full bg-muted"></div>
                <div className="h-2 w-full rounded-full bg-muted"></div>
              </div>

              <div className="size-full bg-muted"></div>

              <div className="size-full p-1">
                <div className="size-full rounded-sm bg-muted"></div>
              </div>

              <div className="mx-auto h-full w-[53%] bg-muted"></div>
            </div>
          </div>
        </div>
      </RadioGroup.Item>

      <label
        htmlFor={`dir-${dir}`}
        className="mt-2 cursor-pointer peer-data-[state=unchecked]:text-border"
      >
        {t(`appearance.dir.${dir}`)}
      </label>
    </div>
  );
};
