import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/common/component/ui/tabs";
import KSpaceSettings from "@/krate/kspace/component/settings";
import { EndpointSettings } from "@/krate/sync/component/settings";

const Settings = () => {
  return (
    <div className="p-6 mx-auto w-full">
      <h1 className="text-2xl font-bold mb-8">Settings</h1>

      <Tabs
        defaultValue="workspace"
        className="flex flex-col md:flex-row gap-6 w-full"
      >
        <TabsList className="flex flex-col h-fit p-2 bg-muted rounded-lg w-full md:w-64">
          <TabsTrigger
            value="workspace"
            className="w-full justify-start px-4 py-3 data-[state=active]:bg-background"
          >
            Workspace Settings
          </TabsTrigger>
          <TabsTrigger
            value="endpoint"
            className="w-full justify-start px-4 py-3 data-[state=active]:bg-background"
          >
            Endpoint Settings
          </TabsTrigger>
        </TabsList>

        <div className="flex-1">
          <TabsContent value="workspace">
            <KSpaceSettings />
          </TabsContent>
          <TabsContent value="endpoint">
            <EndpointSettings />
          </TabsContent>
        </div>
      </Tabs>
    </div>
  );
};

export default Settings;