import { HTMLAttributes, ReactNode } from "react";
import { cn } from "../../../utils/tailwindUtils";

interface TitleBarSpacerProps extends HTMLAttributes<HTMLDivElement> {
  children?: ReactNode;
}

export const TitleBarSpacer = ({ children, ...props }: TitleBarSpacerProps) => {
  return (
    <div {...props} className={cn("titlebar-spacer", props.className)}>
      {children}
    </div>
  );
};
