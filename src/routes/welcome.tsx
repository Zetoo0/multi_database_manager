import { createFileRoute, Link } from "@tanstack/react-router";
import { Button } from "../components/ui/Button";
import { useLayout } from "../providers/LayoutProvider";
import { useEffect } from "react";

export const Route = createFileRoute("/welcome")({
  component: WelcomePage,
});

function WelcomePage() {
  const { setMenuBarHidden, setShowStatusBar } = useLayout();

  useEffect(() => {
    setMenuBarHidden(true);
    setShowStatusBar(false);
    return () => {
      setMenuBarHidden(false);
      setShowStatusBar(true);
    };
  }, []);

  return (
    <div className="flex h-full flex-col items-center justify-center gap-4">
      <h1 className="">Welcome</h1>
      
      <Button variant="outline" asChild>
        <Link to="/">Get Started</Link>
      </Button>
    </div>
  );
}
