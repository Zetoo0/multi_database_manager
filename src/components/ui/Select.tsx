import * as Selects from "@radix-ui/react-select";
import { cn } from "../../utils/tailwindUtils";
import { useEffect } from "react";

export interface SelectProps{
    value: string;
    className?: string;
    options: /*{ label: string; value: string }*/string[];
    onValueChange: (value: string) => void;
    placeholder?: string
}

export const Select = ({ value, className, options, onValueChange,placeholder }: SelectProps) => {

  useEffect(() => {
    console.log("Select options: ",options);
  },[options])

    return (
      <Selects.Root value={value} onValueChange={onValueChange}>
        <Selects.Trigger
          className={cn(
            "inline-flex items-center justify-between rounded border px-4 py-2 text-sm",
            className
          )}
        >
          <Selects.Value placeholder={placeholder} />
        </Selects.Trigger>
        <Selects.Portal>
        <Selects.Content
          className="mt-2 rounded border"
          position="popper"
        >
          <Selects.ScrollDownButton/>
          <Selects.Viewport className="p-1 bg-background max-h-60 overflow-y-auto">
            {options.map((option) => (
              <Selects.Item
                key={option.toLowerCase()}
                value={option.toLowerCase()}
                className="flex cursor-pointer items-center rounded px-3 py-2 text-sm w-full"
              >
                <Selects.ItemText>{option}</Selects.ItemText>
              </Selects.Item>
            ))}
          </Selects.Viewport>
          <Selects.ScrollUpButton/>
        </Selects.Content>
        </Selects.Portal>
      </Selects.Root>
    );
  };
