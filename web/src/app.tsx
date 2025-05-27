import { Suspense, useEffect, useState } from "react";
import { Outlet, useLocation } from "react-router-dom";
import useLocalStorage from "react-use/lib/useLocalStorage";
import LoadingPage from "@/common/pages/loading-page";
import Navigation from "@/common/component/navigation";
import { RoutePaths } from "@/router";
import { useKSpaceStore } from "./store/kspace";

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

    if (currentKSpace.name === "private") {
      link.href = "/static/favicon/chnots-private.svg";
    } else if (currentKSpace.name === "public") {
      link.href = "/static/favicon/chnots.svg";
    } else {
      link.href = "/static/favicon/chnots-protect.svg";
    }

    console.log("set favicon", link.href);
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
      <div className="bg-kbg w-full h-full">
        <Suspense fallback={<LoadingPage />}>
          <Outlet />
        </Suspense>
      </div>
    </div>
  );
};

export default App;
