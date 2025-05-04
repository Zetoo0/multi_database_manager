export const ACTIONS_CONFIG: Record<
  string,
  { label: string; action: string }[]
> = {
  database: [
    { label: "Create Schema", action: "create" },
    { label: "Create Table", action: "create" },
  ],
  schema: [
    { label: "Create Table", action: "create-table" },
    { label: "Create View", action: "create-view" },
    { label: "Create Function", action: "create-function" },
    { label:"Create Sequence", action:"create-sequence"},
    { label: "Create Constraint", action: "create-constraint" },
    { label:"Create Role", action:"create-role"},
    { label:"Create Index", action:"create-index"},
    { label:"Create Trigger", action:"create-triggerfunction"},
  ],
  table: [
    { label: "Add Column", action: "create" },
    { label: "Edit Table", action: "edit" },
    { label: "Delete Table", action: "delete" },
    { label: "Create Table", action: "create" },
    {label:"Create Constraint", action:"create-constraint"},
    {label:"Create Index", action:"create-index"},
    {label:"Create Trigger", action:"create-trigger"},

  ],
  column: [
    { label: "Edit Column", action: "edit" },
    { label: "Delete Column", action: "delete" },
  ],
  view: [
    { label: "Edit View", action: "edit" },
    { label: "Delete View", action: "delete" },
    { label: "Delete View", action: "create" },
  ],
  function: [
    { label: "Edit Function", action: "edit" },
    { label: "Delete Function", action: "delete" },
  ],
  sequence: [
    { label: "Edit Sequence", action: "edit" },
    { label: "Delete Sequence", action: "delete" },
    {label:"Create Sequence", action:"create"}
  ],
  index : [
    { label: "Edit Index", action: "edit" },
    { label: "Delete Index", action: "delete" },
  ],
  constraint : [
    { label: "Edit Constraint", action: "edit" },
    { label: "Delete Constraint", action: "delete" },
  ],
  role: [
    { label: "Edit Role", action: "edit" },
    { label: "Delete Role", action: "delete" },
  ],
  trigger: [
    { label: "Edit Trigger", action: "edit" },
    { label: "Delete Trigger", action: "delete" },
  ],
  triggerfunction: [
    { label: "Edit Trigger", action: "edit" },
    { label: "Delete Trigger", action: "delete" },
  ]
};
