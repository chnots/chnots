import { SidebarProvider } from "@/common/component/ui/sidebar";
import ChnotMain from "../component/layout/main";
import ChnotSidebar from "../component/layout/sidebar";

/**
 * Page for chnots, which is left and right layouted.
 *
 * Current there is only one chnot editor in the page, use multi webpages.
 * @returns ChnotSearchRspThread Page
 */
const ChnotPage = () => {
  return (
    <SidebarProvider>
      <ChnotSidebar />
      <ChnotMain className="h-svh w-full" />
    </SidebarProvider>
  );
};

export default ChnotPage;
