import ChnotThreadSidebar from "@/krate/chnot/component/thread/sidebar";
import { ChnotViewType } from "@/krate/chnot/store";
import { useState } from "react";
import { SidebarInset, SidebarProvider } from "@/common/component/ui/sidebar";
import { genTID } from "@/lib/id_util";
import UnderConstructionPage from "@/common/pages/under-construction-page";
import ChnotSingleSidebar from "../component/single/sidebar";
import ChnotSingleHeadbar from "../component/single/header";
import { ChnotKind } from "../po";
import ChnotSingleMain from "../component/single/main";

/**
 * Page for chnots, which is left and right layouted.
 *
 * Current there is only one chnot editor in the page, use multi webpages.
 * @returns ChnotSearchRspThread Page
 */
const ChnotPage = ({ viewType }: { viewType: ChnotViewType }) => {
  const [monoComponentKey, setMonoComponentKey] = useState(genTID());
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
              <ChnotSingleMain />
            </SidebarInset>
          </>
        ) : viewType === ChnotViewType.Thread ? (
          <>
            <ChnotThreadSidebar viewType={viewType} />
            <SidebarInset className="min-w-0">
              <ChnotSingleHeadbar
                onNew={() => {
                  setMonoComponentKey(genTID());
                }}
                setKind={function (kind: ChnotKind): void {}}
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
