import { Suspense, useEffect, useState } from "react";
import { Outlet, useLocation } from "react-router-dom";
import useLocalStorage from "react-use/lib/useLocalStorage";
import LoadingPage from "@/common/pages/loading-page";
import { RoutePaths } from "@/router";
import faviconSvg from "../public/static/favicon/chnots.svg?raw";
import useParamState from "./hooks/use-param-state";
import { useKSpaceStore } from "./krate/kspace/store";

const App = () => {
  const location = useLocation();
  const { currentKSpaceObj, currentKSpace } = useKSpaceStore((s) => {
    return {
      currentKSpaceObj: s.currentKSpaceObj,
      currentKSpace: s.currentKSpace,
    };
  });
  const [lastVisited] = useLocalStorage<string>("lastVisited", "/home");
  const [initialized, setInitialized] = useState(false);

  const [, setKSpaceParam] = useParamState<string>("ns", currentKSpace);
  useEffect(() => {
    setKSpaceParam(currentKSpace);
  }, [currentKSpace, setKSpaceParam]);

  useEffect(() => {
    let link = document.querySelector<HTMLLinkElement>("link[rel~='icon']");
    if (!link) {
      link = document.createElement("link");
      link.rel = "icon";
      document.getElementsByTagName("head")[0].appendChild(link);
    }

    const c = currentKSpaceObj();
    if (c) {
      const s = faviconSvg.replace("#282828", c.color);
      link.href = `data:image/svg+xml,${encodeURIComponent(s)}`;
    }
  }, [currentKSpaceObj]);

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
            lastVisited,
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
  }, [currentKSpace, lastVisited, location.pathname]);

  return !initialized ? (
    <LoadingPage />
  ) : (
    <Suspense fallback={<LoadingPage />}>
      <Outlet />
    </Suspense>
  );
};

export default App;
