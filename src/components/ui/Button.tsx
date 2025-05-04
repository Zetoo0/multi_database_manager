import { cva, VariantProps } from "class-variance-authority";
import { forwardRef } from "react";
import { Slot } from "@radix-ui/react-slot";
import { cn } from "../../utils/tailwindUtils";

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
}

const buttonVariants = cva(
  "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50",
  {
    variants: {
      variant: {
        default:
          "bg-primary text-primary-foreground shadow hover:bg-primary/90",
        destructive:
          "bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90",
        outline:
          "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
        secondary:
          "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80",
        ghost: "hover:bg-accent hover:text-accent-foreground",
        ghostMuted:
          "text-secondary-foreground/70 hover:text-secondary-foreground",
        link: "text-primary underline-offset-4 hover:underline",
        menubar:
          "justify-between text-xs transition-none hover:bg-accent hover:text-accent-foreground",
      },
      size: {
        default: "h-8 px-4 py-2",
        sm: "h-7 rounded-md px-3 text-xs",
        lg: "h-9 rounded-md px-8",
        icon: "size-8",
        menubar: "h-6 w-full rounded-sm px-2 py-1",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
);

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, asChild = false, ...props }, ref) => {
    const Comp = asChild ? Slot : "button";
    return (
      <Comp
        className={cn(buttonVariants({ variant, size, className }))}
        ref={ref}
        {...props}
      />
    );
  }
);
Button.displayName = "Button";
