import { ReactNode } from "react";
import { Button } from "./Button";
import { cn } from "../../utils/tailwindUtils";
import { Link, LinkProps } from "@tanstack/react-router";

export interface GroupedListProps {
  children: ReactNode;
  className?: string;
  innerClassName?: string;
}

export const GroupedList = ({
  children,
  className,
  innerClassName,
}: GroupedListProps) => {
  return (
    <div className={cn("relative size-full", className)}>
      <ul
        className={cn(
          "scrollbar-sm flex flex-col absolute inset-0 overflow-y-auto",
          innerClassName
        )}
      >
        {children}
      </ul>
    </div>
  );
};

export interface GroupProps {
  title: string;
  children: ReactNode;
  className?: string;
  hideSeparator?: boolean;
}

GroupedList.Group = ({
  title,
  children,
  className,
  hideSeparator,
}: GroupProps) => {
  return (
    <>
      <li
        className={cn(
          "font-bold uppercase select-none text-xs text-muted-foreground px-4 py-1",
          className
        )}
      >
        {title}
      </li>
      {children}
      {!hideSeparator && <hr className="mx-4 my-2" />}
    </>
  );
};

export interface GroupItemProps {
  title: string;
  to?: LinkProps["to"];
}

GroupedList.GroupItem = ({ title, to }: GroupItemProps) => {
  return (
    <li>
      <Button
        variant="ghost"
        className="w-full justify-start text-base font-normal text-muted-foreground hover:bg-muted"
        asChild={!!to}
      >
        {to ? <Link to={to}>{title}</Link> : title}
      </Button>
    </li>
  );
};
