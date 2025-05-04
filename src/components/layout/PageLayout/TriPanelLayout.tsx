import { ReactNode } from "react";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "../../ui/Resizable";
import { ScrollArea, ScrollBar } from "../../ui/ScrollArea";

type TriPanelLayoutProps = {
  leftPanel?: ReactNode;
  rightTopPanel?: ReactNode;
  rightBottomPanel?: ReactNode;
};

export const TriPanelLayout = ({
  leftPanel,
  rightTopPanel,
  rightBottomPanel,
}: TriPanelLayoutProps) => {
  return (
    <ResizablePanelGroup direction="horizontal" className="size-full">
      <ResizablePanel
        id="panel-left"
        minSize={10}
        maxSize={90}
        defaultSize={20}
        className="relative"
      >
        <div className="absolute inset-0 size-full p-px">
          <ScrollArea className="size-full">{leftPanel}</ScrollArea>
        </div>
      </ResizablePanel>

      <ResizableHandle />

      <ResizablePanel minSize={10} maxSize={90} defaultSize={80}>
        <ResizablePanelGroup direction="vertical">
          <ResizablePanel
            id="panel-right-top"
            minSize={10}
            maxSize={90}
            defaultSize={80}
            className="relative"
          >
            <div className="absolute inset-0 size-full p-px">
              <ScrollArea className="size-full">
                {rightTopPanel}
                <ScrollBar orientation="horizontal" />
              </ScrollArea>
            </div>
          </ResizablePanel>

          <ResizableHandle />

          <ResizablePanel
            id="panel-right-bottom"
            minSize={10}
            maxSize={90}
            defaultSize={20}
            className="relative"
          >
            <div className="absolute inset-0 size-full p-px">
              <ScrollArea className="size-full">
                {rightBottomPanel}
                <ScrollBar orientation="horizontal" />
              </ScrollArea>
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </ResizablePanel>
    </ResizablePanelGroup>
  );
};
