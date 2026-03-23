import { useMemo } from "react";
import { useLocation } from "react-router-dom";
import { SidebarProvider } from "@/common/component/ui/sidebar";
import type { TID } from "@/lib/id_util";
import ChnotMain from "../component/layout/main";
import ChnotSidebar from "../component/layout/sidebar";

/**
 * Page for chnots, which is left and right layouted.
 *
 * Current there is only one chnot editor in the page, use multi webpages.
 * @returns ChnotSearchRspThread Page
 */
const ChnotPage = () => {
  const location = useLocation();
  const focusedOtid = useMemo<TID | undefined>(() => {
    const otidParam = new URLSearchParams(location.search).get("otid") ?? "";
    const raw = otidParam.trim();
    const otid = Number(raw);
    if (raw && Number.isFinite(otid) && otid > 0) {
      return otid;
    }
    return undefined;
  }, [location.search]);

  if (focusedOtid) {
    return (
      <ChnotMain
        className="h-svh w-full"
        initialOtid={focusedOtid}
        hideSidebar
      />
    );
  }

  return (
    <SidebarProvider>
      <ChnotSidebar />
      <ChnotMain className="h-svh w-full" />
    </SidebarProvider>
  );
};

export default ChnotPage;
