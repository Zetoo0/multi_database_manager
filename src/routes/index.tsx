import { createFileRoute } from "@tanstack/react-router";
import { TitleBarSpacer } from "../components/layout/TitleBar/TitleBarSpacer";
import { MainContent } from "../components/main/MainContent";

export const Route = createFileRoute("/")({
  component: IndexPage,
});

function IndexPage() {
  return (
    <TitleBarSpacer>
      <MainContent />
    </TitleBarSpacer>
  );
}
