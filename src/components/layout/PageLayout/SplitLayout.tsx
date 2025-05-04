import { ReactNode } from "react";
import { cn } from "../../../utils/tailwindUtils";

type Side = "left" | "right" | "both" | "none";

interface SplitLayoutProps {
  left: ReactNode;
  right: ReactNode;
  reverseBackground?: boolean;
  ignoreTitleBar?: Side;
  leftSize?: string;
  rightSize?: string;
}

export const SplitLayout = ({
  left,
  right,
  reverseBackground = false,
  ignoreTitleBar = "none",
  leftSize = "auto",
  rightSize = "auto",
}: SplitLayoutProps) => {
  return (
    <div
      className="mx-auto grid size-full"
      style={{ gridTemplateColumns: `${leftSize} ${rightSize}` }}
    >
      <Section
        side="left"
        reverseBackground={reverseBackground}
        ignoreTitleBar={ignoreTitleBar}
        className="ltr:border-r rtl:border-l "
      >
        {left}
      </Section>
      <Section
        side="right"
        reverseBackground={!reverseBackground}
        ignoreTitleBar={ignoreTitleBar}
      >
        {right}
      </Section>
    </div>
  );
};

interface SectionProps {
  children?: ReactNode;
  side: Side;
  reverseBackground?: boolean;
  ignoreTitleBar?: Side;
  className?: string;
}

const Section = ({
  children,
  side,
  reverseBackground,
  ignoreTitleBar,
  className,
}: SectionProps) => {
  return (
    <div
      className={cn(
        "overflow-y-auto h-screen",

        {
          "bg-accent": !reverseBackground,
          "titlebar-spacer":
            ignoreTitleBar !== side && ignoreTitleBar !== "both",
        },
        className
      )}
    >
      {/* {ignoreTitleBar !== side && ignoreTitleBar !== "both" && (
        <div className="titlebar-spacer"></div>
      )} */}
      {children}
    </div>
  );
};
