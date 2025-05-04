import { useLayoutEffect, useState } from "react";
import { getPanelElement } from "react-resizable-panels";

/**
 * Custom hook to get the size of a panel from `react-resizable-panels`.
 * @param id The id of the panel to get the size of.
 */
export function usePanelSize(id: string) {
  const [size, setSize] = useState<{
    width: number;
    height: number;
  }>();

  useLayoutEffect(() => {
    const panelElement = getPanelElement(id);

    if (panelElement) {
      const observer = new ResizeObserver(() =>
        setSize({
          width: panelElement.offsetWidth,
          height: panelElement.offsetHeight,
        })
      );

      observer.observe(panelElement);

      return () => observer.disconnect();
    }
  }, []);

  return size;
}
