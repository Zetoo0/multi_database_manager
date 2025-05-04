import { TriPanelLayout } from "../layout/PageLayout/TriPanelLayout";
import { DatabaseTree } from "../layout/SideBar/DatabaseTree";
import { QueryResultTable } from "../query/QueryResultTable";
import { TabInterface } from "../ui/TabInterface";

export const MainContent = () => {

  return (
    <TriPanelLayout
      leftPanel={<div className="size-[220px]">
        <DatabaseTree />
        
      </div>}
      rightTopPanel={<div className="size-[1500px]">
          <TabInterface/>
      </div>}
      rightBottomPanel={<div className="size-[1500px]">
        <QueryResultTable />
      </div>  
    }
    />
  );
};
