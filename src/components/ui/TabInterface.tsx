import { Tab, TabList, TabPanel, Tabs } from "react-tabs";
import "react-tabs/style/react-tabs.css"; // Import styles for React Tabs
import { useState } from "react";
import { SqlEditor } from "../query/Editor";
import { Button } from "./Button";

export const TabInterface = ({ /*children*/ }: any) => {
  const [tabs, setTabs] = useState([{ title: "Query", content: "Initial content" }]);

  const handleAddTab = () => {
    const newTab = { title: `Query ${tabs.length + 1}`, content: "" };
    setTabs([...tabs, newTab]);
  };

  const handleRemoveTab = (index: number) => {
    setTabs((prevTabs) => prevTabs.filter((_, i) => i !== index));
  };

  return (
    <div>
      <Tabs defaultIndex={0}>
        <TabList>
          {tabs.map((tab, index) => (
            <Tab key={index}>
              {tab.title}
              <Button
                onClick={(e) => {
                  e.stopPropagation();
                  handleRemoveTab(index);
                }}
                className="ml-2 text-red-500"
              >
                x
              </Button>
            </Tab>
          ))}
          <Button onClick={handleAddTab} className="ml-4">
            +
          </Button>
        </TabList>

        {tabs.map((tab, index) => (
          <TabPanel key={index}>
            <SqlEditor
              sqlType="sql"
              value={tab.content}
              onChange={(value) => {
                const updatedTabs = [...tabs];
                updatedTabs[index].content = value;
                setTabs(updatedTabs);
              }}
            />
          </TabPanel>
        ))}
      </Tabs>
    </div>
  );
};
