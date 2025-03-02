import { Suspense, useEffect, useState } from "react";
import { Outlet, useLocation } from "react-router-dom";
import useLocalStorage from "react-use/lib/useLocalStorage";
import useResponsiveWidth from "@/hooks/use-responsive-width";
import LoadingPage from "@/common/pages/loading-page";
import Navigation from "@/common/component/navigation";
import { RoutePaths } from "@/router";
import { useNamespaceStore } from "./store/namespace";

const App = () => {
  const location = useLocation();
  const { sm } = useResponsiveWidth();
  const { currentNamespace } = useNamespaceStore();
  const [lastVisited] = useLocalStorage<string>("lastVisited", "/home");
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    if (!currentNamespace) {
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
    <div className="h-screen max-h-screen flex flex-col">
      <div className="w-full h-12 justify-start items-start fixed top-0 left-0 select-none kborder z-2 bg-secondary border-b">
        <Navigation />
      </div>
      <div className="bg-kbg w-full h-full flex-1 pt-12">
        <Suspense fallback={<LoadingPage />}>
          <Outlet />
        </Suspense>
      </div>
    </div>
  );
};

export default App;
