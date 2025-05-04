import {
  createFileRoute,
  Link,
  Outlet,
  useNavigate,
} from "@tanstack/react-router";
import { SettingsMenu } from "../../components/settings/SettingsMenu";
import { SplitLayout } from "../../components/layout/PageLayout/SplitLayout";
import { useEffect } from "react";
import { useLayout } from "../../providers/LayoutProvider";
import { Button } from "../../components/ui/Button";
import { X } from "lucide-react";
import { useHotkey } from "../../hooks/hotkey/useHotkey";

export const Route = createFileRoute("/settings/_menus")({
  component: SettingsPage,
});

function SettingsPage() {
  const { setShowStatusBar, setShowTitleBar } = useLayout();
  const navigate = useNavigate();
  const { formatted } = useHotkey("closeSettings", () => {
    navigate({ to: "/" });
  });

  useEffect(() => {
    setShowStatusBar(false);
    setShowTitleBar(false);
    return () => {
      setShowStatusBar(true);
      setShowTitleBar(true);
    };
  }, []);

  return (
    <SplitLayout
      ignoreTitleBar="left"
      left={<SettingsMenu />}
      right={
        <div className="grid grid-cols-[auto_1fr] gap-2">
          <Outlet />
          <Button
            variant="outline"
            className="sticky top-0 size-8 rounded-full border-2 border-muted-foreground/70 p-0 text-muted-foreground/70 hover:border-muted-foreground"
            asChild
          >
            <Link to="/">
              <X strokeWidth="3" className="size-4" />

              <span className="absolute inset-x-0 top-full mx-auto mt-2 w-min text-xs uppercase">
                {formatted}
              </span>
            </Link>
          </Button>
        </div>
      }
    />
  );
}
