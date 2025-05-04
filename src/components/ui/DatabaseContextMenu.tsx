import * as ContextMenus from "@radix-ui/react-context-menu";
import { cn } from "../../utils/tailwindUtils";
import { Tree } from "./Tree";
import { ACTIONS_CONFIG } from "../../Types/object/ContextMenuObjectMapping";
import { useEffect } from "react";
interface DatabaseContextMenuProps {
    node:Tree,
    onAction:(action:string,node:Tree) => void,
    children?: React.ReactNode
};



export const DatabaseContextMenu = ({ node, onAction, children }: DatabaseContextMenuProps) => {
    const actions = ACTIONS_CONFIG[node.type] || [];
  
    useEffect(() => {
      console.log("Select node:", node);
    },[])

    return (
      <ContextMenus.Root>
        <ContextMenus.Trigger className={cn("cursor-pointer")}>
          {children}
        </ContextMenus.Trigger>
        <ContextMenus.Content className="bg-white rounded shadow-md p-2">
          {
            actions.map((item) => (
                <ContextMenus.Item
                key={item.action} 
                onSelect={() => onAction(item.action, node)}
                className="cursor-pointer p-2 hover:bg-gray-700 rounded"
              >
                {item.label}
              </ContextMenus.Item>
            ))
          }
          {/*node.type && <ContextMenus.Item onSelect={() => onAction("create", node)}>Create {node.type}</ContextMenus.Item>*/}
        </ContextMenus.Content>
      </ContextMenus.Root>
    );
  };