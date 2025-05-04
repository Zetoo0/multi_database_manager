import { MemoryStick } from "lucide-react";
import { useMemoryUsage } from "../../../hooks/system/useMemoryUsage";
import { cn } from "../../../utils/tailwindUtils";
import { useTranslation } from "react-i18next";
import { useEffect, useState } from "react";

interface StatusBarProps {
  className?: string;
}

export const StatusBar = ({ className }: StatusBarProps) => {
  const { memoryUsage } = useMemoryUsage();
  const {
    i18n: { language },
  } = useTranslation();

  const [timestamp, setTimestamp] = useState(Date.now());

  useEffect(() => {
    const intervalId = setInterval(() => {
      setTimestamp(Date.now());
    }, 1000); // update every 1 second

    return () => clearInterval(intervalId);
  }, []);

  return (
    <div
      className={cn(
        "border-t px-2 leading-none grid grid-cols-2 text-muted-foreground size-full z-10 bg-background text-xs font-light",
        className
      )}
    >
      {/* LEFT SECTION OF STATUS BAR - SYSTEM USAGE, COUNTS */}
      <div className="flex items-center gap-2 leading-none">
        <div>
          {new Date(timestamp).toLocaleTimeString(language, {
            hour: "2-digit",
            minute: "2-digit",
          })}
        </div>

        <div>
          {new Date(timestamp).toLocaleDateString(language, {
            month: "short",
            day: "numeric",
            year: "numeric",
          })}
        </div>

        <div className="flex items-center gap-1">
          <MemoryStick size={16} />
          {memoryUsage}
        </div>
      </div>
    </div>
  );
};
