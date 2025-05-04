import { AnimatePresence, AnimatePresenceProps } from "framer-motion";
import { createContext, ReactNode, useContext } from "react";

export type Direction = "backwards" | "forwards";

const variants = {
  initial: (direction: Direction) => ({
    x: direction === "forwards" ? "100%" : "-100%",
  }),
  target: {
    x: "0%",
  },
  exit: (direction: Direction) => ({
    x: direction === "forwards" ? "-100%" : "100%",
  }),
};

const DirectionContext = createContext<Direction>("forwards");

type AnimatePresenceWithDirectionProps = {
  children: ReactNode;
  direction: Direction;
} & Omit<AnimatePresenceProps, "custom">;

export const AnimatePresenceWithDirection = ({
  children,
  direction,
  ...props
}: AnimatePresenceWithDirectionProps) => {
  return (
    <DirectionContext.Provider value={direction}>
      <AnimatePresence {...props}>{children}</AnimatePresence>
    </DirectionContext.Provider>
  );
};

export const useDirectionAnimation = () => {
  const direction = useContext(DirectionContext);

  return {
    variants,
    custom: direction,
    initial: "initial",
    animate: "target",
    exit: "exit",
  };
};
