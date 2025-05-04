import { Button } from "./Button";
import { LinkProps, Link } from "@tanstack/react-router";
import * as Menubar from "@radix-ui/react-menubar";
import { ChevronRight } from "lucide-react";
import { PropsWithChildren } from "react";

export const MenuBarRoot = Menubar.Root;

interface MenuBarTriggerProps extends PropsWithChildren {
  title: string;
}
export const MenuBarTrigger = ({ title, children }: MenuBarTriggerProps) => {
  return (
    <Menubar.Menu>
      <Menubar.Trigger asChild>
        <button className="h-7 rounded px-2 text-sm transition-colors hover:bg-accent data-[state=open]:bg-accent">
          {title}
        </button>
      </Menubar.Trigger>

      <Menubar.Portal>
        <Menubar.Content
          align="start"
          sideOffset={8}
          alignOffset={0}
          className="flex min-w-[200px] flex-col rounded-md border bg-background p-1 text-foreground"
        >
          {children}
        </Menubar.Content>
      </Menubar.Portal>
    </Menubar.Menu>
  );
};

interface MenuItemProps {
  title: string;
  shortcut?: string;
  onClick?: () => void;
}
export const MenuItem = ({ title, shortcut, onClick }: MenuItemProps) => {
  return (
    <Menubar.Item asChild>
      <Button variant="menubar" size="menubar" onClick={onClick}>
        {title} <div className="text-muted-foreground">{shortcut}</div>
      </Button>
    </Menubar.Item>
  );
};

interface MenuSubMenuProps extends PropsWithChildren {
  title: string;
}
export const MenuSubMenu = ({ title, children }: MenuSubMenuProps) => {
  return (
    <Menubar.Sub>
      <Menubar.SubTrigger asChild>
        <Button variant="menubar" size="menubar">
          {title}
          <ChevronRight className="size-4 text-muted-foreground rtl:-scale-100" />
        </Button>
      </Menubar.SubTrigger>

      <Menubar.Portal>
        <Menubar.SubContent className="flex min-w-[200px] flex-col rounded-md border bg-background p-1 text-foreground">
          {children}
        </Menubar.SubContent>
      </Menubar.Portal>
    </Menubar.Sub>
  );
};

interface MenuLinkProps {
  title: string;
  shortcut?: string;
  to: LinkProps["to"];
}
export const MenuLink = ({ title, shortcut, to }: MenuLinkProps) => {
  return (
    <Menubar.Item asChild>
      <Button variant="menubar" size="menubar" asChild>
        <Link to={to}>
          {title} <div className="text-muted-foreground">{shortcut}</div>
        </Link>
      </Button>
    </Menubar.Item>
  );
};

export const MenuSeparator = () => {
  return <Menubar.Separator className="my-1 h-px w-full bg-border" />;
};
