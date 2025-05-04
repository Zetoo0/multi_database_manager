import { useEffect, useState } from "react";
import { Tree } from "./Tree";
import { DatabaseContextMenu } from "./DatabaseContextMenu";

interface TreeNodeProps{
    node: Tree;
    onAction:(action:string,node:Tree) => void;    
}

export const TreeNode = ({node,onAction}:TreeNodeProps) => {

    
    const [expanded,setExpanded] = useState(false);

    const [clicked, setClicked] = useState(false);
    
    useEffect(() => {
        console.log("Handling node in TreeNode: ", node);
      },[])


    return (
            <div onClick={() => setClicked(!clicked)}>
                <span onClick={() => setExpanded(!expanded)}>
                    {expanded ? <span>-</span> : <span>+</span>}
                </span>
                <span onClick={(e) => e.stopPropagation()}  className={`hover:bg-muted cursor-pointer} `} >

                    <DatabaseContextMenu node={node} onAction={onAction}>
                        {node.name}
                    </DatabaseContextMenu>
                </span>
                    {expanded && node.children && (
                        <div className="ml-4 mt-2 space-y-1">
                            {node.children.map((child: Tree, index:number) => (
                                <TreeNode key={index} node={child} onAction={onAction}/>
                            ))}
                        </div>
                    )}

            </div>
    )
}
