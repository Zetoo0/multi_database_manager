import { cloneElement, ReactElement } from "react";
import { cn } from "../../../utils/tailwindUtils";
import { Button } from "../../ui/Button";
import { Link, LinkProps } from "@tanstack/react-router";
import * as Tooltip from "@radix-ui/react-tooltip";
import { motion } from "framer-motion";

interface MenuBarButtonProps {
  className?: string;
  icon: ReactElement;
  onClick?: () => void;
  to?: LinkProps["to"];
  label: string;
}

export const MenuBarButton = ({
  icon,
  onClick,
  className,
  to,
  label,
}: MenuBarButtonProps) => {
  const clonedIcon = cloneElement(icon, { strokeWidth: 2 });

  return (
    <Tooltip.Provider>
      <Tooltip.Root delayDuration={0}>
        <Tooltip.Trigger asChild>
          <Button
            variant="ghostMuted"
            size="icon"
            className={cn("w-[66px] h-14 rounded-none", className)}
            onClick={onClick}
            asChild={!!to}
          >
            {to ? (
              <Link
                to={to}
                className="border-x-2 border-x-transparent data-[status]:border-l-primary data-[status]:text-primary"
              >
                {clonedIcon}
              </Link>
            ) : (
              clonedIcon
            )}
          </Button>
        </Tooltip.Trigger>
        <Tooltip.Portal>
          <Tooltip.Content side="right" sideOffset={4} asChild>
            <motion.div
              className="rounded bg-accent-foreground px-2 py-1.5 text-sm leading-none text-background shadow"
              initial={{ opacity: 0, scaleX: 0.5, originX: 0 }}
              animate={{ opacity: 1, scaleX: 1 }}
            >
              {label}
              <Tooltip.Arrow
                height={4}
                width={8}
                className="fill-accent-foreground"
              />
            </motion.div>
          </Tooltip.Content>
        </Tooltip.Portal>
      </Tooltip.Root>
    </Tooltip.Provider>
  );
};
