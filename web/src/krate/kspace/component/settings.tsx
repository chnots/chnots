import EditableTable, { ColumnConfig } from "@/common/component/editable-table";

const KSpaceSettings = () => {
  const columnsConfig: ColumnConfig[] = [
    { id: "name", name: "Name", type: "string", required: true },
    {
      id: "age",
      name: "Age",
      type: "number",
      required: false,
    },
    {
      id: "joinDate",
      name: "Join Date",
      type: "date",
      required: false,
    },
    {
      id: "adavtar",
      name: "Adavtar",
      type: "image",
      required: true,
    },
  ];

  return <EditableTable columnConfig={columnsConfig} initialData={[]} />;
};

export default KSpaceSettings;
