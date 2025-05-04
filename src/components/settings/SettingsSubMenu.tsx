import { HTMLAttributes, ReactNode } from "react";
import { cn } from "../../utils/tailwindUtils";

interface SettingsSubMenuProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
  title: string;
}

export const SettingsSubMenu = ({
  children,
  title,
  className,
  ...props
}: SettingsSubMenuProps) => {
  return (
    <div
      className={cn("min-w-[640px] max-w-[640px] px-4", className)}
      {...props}
    >
      <h3 className="mb-4 select-none text-2xl font-semibold">{title}</h3>
      {children}
    </div>
  );
};
