import { Suspense, useEffect, useState } from "react";
import { Outlet, useLocation } from "react-router-dom";
import useLocalStorage from "react-use/lib/useLocalStorage";
import LoadingPage from "@/common/pages/loading-page";
import Navigation from "@/common/component/navigation";
import { RoutePaths } from "@/router";
import { useKSpaceStore } from "./krate/kspace/store/store";
import faviconSvg from "../public/static/favicon/chnots.svg?raw";

const App = () => {
  const location = useLocation();
  const { currentKSpace } = useKSpaceStore();
  const [lastVisited] = useLocalStorage<string>("lastVisited", "/home");
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    let link = document.querySelector<HTMLLinkElement>("link[rel~='icon']");
    if (!link) {
      link = document.createElement("link");
      link.rel = "icon";
      document.getElementsByTagName("head")[0].appendChild(link);
    }

    const s = faviconSvg.replace("#282828", currentKSpace.color);

    link.href = `data:image/svg+xml,${encodeURIComponent(s)}`;
  }, [currentKSpace]);

  useEffect(() => {
    if (!currentKSpace) {
      if (
        (
          [RoutePaths.ROOT, RoutePaths.Chnots, RoutePaths.Toents] as string[]
        ).includes(location.pathname)
      ) {
        window.location.href = RoutePaths.Chnots;
        return;
      }
    } else {
      if (location.pathname === RoutePaths.ROOT) {
        if (
          lastVisited &&
          ([RoutePaths.Chnots, RoutePaths.Toents] as string[]).includes(
            lastVisited
          )
        ) {
          window.location.href = lastVisited;
        } else {
          window.location.href = RoutePaths.Chnots;
        }
        return;
      }
    }

    setInitialized(true);
  }, []);

  return !initialized ? (
    <LoadingPage />
  ) : (
    <div className="h-screen max-h-screen flex flex-row kc-basic">
      <div className="h-full justify-start items-start select-none  z-2 border-b w-16">
        <Navigation />
      </div>
      <div className="w-full h-full">
        <Suspense fallback={<LoadingPage />}>
          <Outlet />
        </Suspense>
      </div>
    </div>
  );
};

export default App;
