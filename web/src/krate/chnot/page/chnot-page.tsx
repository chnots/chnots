import ChnotThreadSidebar from "@/krate/chnot/component/thread/sidebar";
import { ChnotViewType } from "@/krate/chnot/store";
import { SidebarProvider } from "@/common/component/ui/sidebar";
import UnderConstructionPage from "@/common/pages/under-construction-page";
import ChnotSingleSidebar from "../component/single/sidebar";
import ChnotSingleMain from "../component/single/main";
import ChnotThreadMain from "../component/thread/main";

/**
 * Page for chnots, which is left and right layouted.
 *
 * Current there is only one chnot editor in the page, use multi webpages.
 * @returns ChnotSearchRspThread Page
 */
const ChnotPage = ({ viewType }: { viewType: ChnotViewType }) => {
  return (
    <SidebarProvider>
      {viewType === ChnotViewType.Single ? (
        <div className="bg-panel flex max-w-screen w-screen overflow-y-auto">
          <ChnotSingleSidebar viewType={ChnotViewType.Single} />
          <main className="w-full h-screen">
            <ChnotSingleMain />
          </main>
        </div>
      ) : viewType === ChnotViewType.Thread ? (
        <>
          <ChnotThreadSidebar viewType={viewType} />
          <main className="w-full h-screen">
            <ChnotThreadMain />
          </main>
        </>
      ) : (
        <UnderConstructionPage />
      )}
    </SidebarProvider>
  );
};

export default ChnotPage;
