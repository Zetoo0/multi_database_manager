import * as Selects from "@radix-ui/react-select";
import { cn } from "../../utils/tailwindUtils";
import { useEffect, useState } from "react";

export interface SelectProps{
    value: string;
    className?: string;
    options: /*{ label: string; value: string }*/string[];
    onValueChange: (value: string) => void;
    placeholder?: string
}

export const MultiSelect = ({ value, className, options, onValueChange,placeholder }: SelectProps) => {
    const [selectedOptions, setSelectedOptions] = useState<string[]>([]);

  useEffect(() => {
    console.log("Select options: ",options);
  },[options])

  const onOptionSelect = (option: string) => {
    if(selectedOptions.includes(option)){
        selectedOptions.splice(selectedOptions.indexOf(option), 1);
        return;
    }
    setSelectedOptions(prev => [...prev, option]);
  };

    return (
      <Selects.Root value={value} onValueChange={(e) => {
        onValueChange(e);
        onOptionSelect(e);
      }}>
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
            {options &&options.map((option) => (
              <Selects.Item
                key={option.toLowerCase()}
                value={option.toLowerCase()}
                className="flex cursor-pointer items-center rounded px-3 py-2 text-sm w-full"
              >
                <Selects.ItemText>{selectedOptions.includes(option.toLowerCase()) ? option.toLowerCase() + "✔️" : option.toLowerCase()}</Selects.ItemText>
              </Selects.Item>
            ))}
          </Selects.Viewport>
          <Selects.ScrollUpButton/>
        </Selects.Content>
        </Selects.Portal>
      </Selects.Root>
    );
  };
