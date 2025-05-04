import { createContext, useContext, useState } from "react";
import { ObjectCreationInfo } from "../Types/object/ObjectCreationInfo";
import { ObjectEditInfo } from "../Types/object/ObjectEditInfo";

type ObjectContextProvider = {
  objectCreationInfo: ObjectCreationInfo | null;
  editObject: ObjectEditInfo | null;
  setEditObject: (editObject: ObjectEditInfo | null) => void;
  setObjectCreationInfo: (objectCreationInfo: ObjectCreationInfo | null) => void;
};

const ObjectContext = createContext<ObjectContextProvider | undefined>(undefined);

interface ObjectProviderProps {
  children: React.ReactNode;
}

export const ObjectProvider = ({ children }: ObjectProviderProps) => {
  const [objectCreationInfo, setObjectCreationInfo] = useState<ObjectCreationInfo | null>(null);
  const [editObject, setEditObject] = useState<ObjectEditInfo | null>(null);


  return (
    <ObjectContext.Provider
      value={{
        objectCreationInfo,
        editObject,
        setEditObject,
        setObjectCreationInfo,
      }}
    >
      {children}
    </ObjectContext.Provider>
  );
};

export const useObject = ()  => {
  const context = useContext(ObjectContext);
  if (!context) {
    throw new Error("useMetadata must be used within a MetadataProvider");
  }
  return context;
};
