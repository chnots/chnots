import React, { ReactElement, Suspense, useEffect, useState } from "react";
import { Outlet, useLocation } from "react-router-dom";
import useLocalStorage from "react-use/lib/useLocalStorage";
import useResponsiveWidth from "@/hooks/use-responsive-width";
import LoadingPage from "@/common/pages/loading-page";
import Navigation from "@/common/component/navigation";
import { RoutePaths } from "@/router";
import { useWorkspaceStore } from "./store/workspace";

const App = () => {
  const location = useLocation();
  const { currentWorkspace } = useWorkspaceStore();
  const [lastVisited] = useLocalStorage<string>("lastVisited", "/home");
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    let link = document.querySelector<HTMLLinkElement>("link[rel~='icon']");
    if (!link) {
      link = document.createElement("link");
      link.rel = "icon";
      document.getElementsByTagName("head")[0].appendChild(link);
    }
    if (currentWorkspace.name === "private") {
      link.href = "/chnots-private.svg";
    } else if (currentWorkspace.name === "public") {
      link.href = "/chnots.svg";
    } else {
      link.href = "/chnots-protect.svg";
    }
  }, [currentWorkspace]);

  useEffect(() => {
    if (!currentWorkspace) {
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
