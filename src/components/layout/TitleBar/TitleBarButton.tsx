import * as Tooltip from "@radix-ui/react-tooltip";
import { motion } from "framer-motion";
import { cloneElement, ReactElement } from "react";
import { Button } from "../../ui/Button";

interface TitleBarButtonProps {
  icon: ReactElement;
  label: string;
  onClick?: () => void;
  disabled?: boolean;
  className?: string;
}

export const TitleBarButton = ({
  icon,
  label,
  onClick,
  disabled,
  className,
}: TitleBarButtonProps) => {
  const clonedIcon = cloneElement(icon, { size: 20, strokeWidth: 1.5 });

  return (
    <Tooltip.Provider>
      <Tooltip.Root delayDuration={0}>
        <Tooltip.Trigger asChild>
          <Button
            variant="ghost"
            size="icon"
            onClick={onClick}
            disabled={disabled}
            className={className}
          >
            {clonedIcon}
          </Button>
        </Tooltip.Trigger>

        <Tooltip.Portal>
          <Tooltip.Content side="bottom" sideOffset={4} asChild>
            <motion.div
              className="rounded bg-accent-foreground px-2 py-1.5 text-sm leading-none text-background shadow"
              initial={{ opacity: 0, scaleY: 0.5, originY: 0 }}
              animate={{ opacity: 1, scaleY: 1 }}
            >
              {label}
              <Tooltip.Arrow height={4} width={8} />
            </motion.div>
          </Tooltip.Content>
        </Tooltip.Portal>
      </Tooltip.Root>
    </Tooltip.Provider>
  );
};
