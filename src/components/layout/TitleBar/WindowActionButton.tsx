import { cloneElement, ReactElement } from "react";
import { cn } from "../../../utils/tailwindUtils";

type Variant = "default" | "destructive";

interface WindowActionButtonProps {
  icon: ReactElement;
  onClick: () => void;
  disabled?: boolean;
  variant?: Variant;
  iconSize?: number;
}

export const WindowActionButton = ({
  icon,
  onClick,
  disabled,
  variant = "default",
  iconSize = 16,
}: WindowActionButtonProps) => {
  const clonedIcon = cloneElement(icon, { size: iconSize, strokeWidth: 2 });

  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={cn("h-7 w-10 flex items-center justify-center", {
        "hover:bg-muted": variant === "default",
        "hover:bg-red-500": variant === "destructive",
      })}
    >
      {clonedIcon}
    </button>
  );
};
