import * as Tabs from "@radix-ui/react-tabs";
import { cn } from "../../utils/tailwindUtils";

export interface TabSelectProps {
  value: string;
  className?: string;
  options: { label: string; value: string }[];
  onValueChange: (value: string) => void;
}

export const TabSelect = ({
  value,
  options,
  className,
  onValueChange,
}: TabSelectProps) => {
  return (
    <Tabs.Root value={value} onValueChange={onValueChange}>
      <Tabs.List className={cn("flex", className)}>
        {options.map((option) => (
          <Tabs.Trigger
            key={option.value}
            value={option.value}
            className="inline-flex h-6 items-center justify-center whitespace-nowrap rounded-md border border-transparent px-3 py-1 text-xs ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:border-border data-[state=active]:bg-accent data-[state=active]:text-foreground"
          >
            {option.label}
          </Tabs.Trigger>
        ))}
      </Tabs.List>
    </Tabs.Root>
  );
};
