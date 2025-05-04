import { HTMLAttributes } from "react";
import { cn } from "../../utils/tailwindUtils";

interface ConnectionSubGroupProps extends HTMLAttributes<HTMLDivElement> {
  title: string;
  help?: string;
}

export const ConnectionSubGroup = ({
  children,
  className,
  title,
  help,
}: ConnectionSubGroupProps) => {
  return (
    <div
      className={cn("border-b pb-4 mb-4 last:mb-0 last:border-b-0", className)}
    >
      <h4 className="mb-2 select-none text-xs font-bold uppercase text-muted-foreground">
        {title}
      </h4>
      <p className="mb-4 text-xs text-muted-foreground">{help}</p>
      {children}
    </div>
  );
};
