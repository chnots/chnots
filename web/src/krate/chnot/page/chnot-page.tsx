import ChnotThreadSidebar from "@/krate/chnot/component/thread/sidebar";
import { ChnotViewType } from "@/krate/chnot/store";
import { useState } from "react";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/common/component/ui/sidebar";
import { genTID, genUID } from "@/lib/id_util";
import { Button } from "@/common/component/ui/button";
import Icon from "@/common/component/icon";
import UnderConstructionPage from "@/common/pages/under-construction-page";
import ChnotSingleSidebar from "../component/single/sidebar";
import ChnotSingleEditor from "../component/single/editor";

const HeadBar = ({ onNew }: { onNew: () => void }) => {
  return (
    <div className="w-full">
      <SidebarTrigger />
      <Button onClick={onNew}>
        <Icon.BadgePlusIcon />
      </Button>
    </div>
  );
};

/**
 * Page for chnots, which is left and right layouted.
 *
 * Current there is only one chnot editor in the page, use multi webpages.
 * @returns ChnotSearchRspThread Page
 */
const ChnotPage = ({ viewType }: { viewType: ChnotViewType }) => {
  const [monoComponentKey, setMonoComponentKey] = useState(genUID());
  return (
    <div className="bg-panel flex h-full max-h-full rounded-md overflow-hidden">
      <SidebarProvider
        style={
          {
            "--sidebar-width": "calc(var(--spacing) * 96)",
            "--header-height": "calc(var(--spacing) * 12)",
          } as React.CSSProperties
        }
      >
        {viewType === ChnotViewType.Single ? (
          <>
            <ChnotSingleSidebar viewType={ChnotViewType.Single} />
            <SidebarInset className="min-w-0">
              <HeadBar
                onNew={() => {
                  setMonoComponentKey(genUID());
                }}
              />
              <ChnotSingleEditor key={monoComponentKey} otid={genTID()} />
            </SidebarInset>
          </>
        ) : viewType === ChnotViewType.Thread ? (
          <>
            <ChnotThreadSidebar viewType={viewType} />
            <SidebarInset className="min-w-0">
              <HeadBar
                onNew={function (): void {
                  setMonoComponentKey(genUID());
                }}
              />
              {/*               <ChnotThreadEditor
                key={monoComponentKey}
                threadMeta={{
                  otid: 0,
                  kspace: "",
                  pin_tid: undefined,
                  archive_tid: undefined,
                  tid: 0,
                }}
                globalBar={undefined}
              /> */}
            </SidebarInset>
          </>
        ) : (
          <UnderConstructionPage />
        )}
      </SidebarProvider>
    </div>
  );
};

export default ChnotPage;
