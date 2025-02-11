import { Suspense, useEffect, useState } from "react";
import { Outlet, useLocation } from "react-router-dom";
import useLocalStorage from "react-use/lib/useLocalStorage";
import useResponsiveWidth from "@/hooks/use-responsive-width";
import LoadingPage from "@/pages/loading-page";
import Navigation from "@/components/navigation";
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
    <>
      {sm && (
        <div className="w-16 flex flex-col justify-start items-start fixed top-0 left-0 select-none kborder h-full z-2 py-5 bg-secondary">
          <Navigation />
        </div>
      )}
      <div className="h-screen max-h-screen sm:pl-16 bg-kbg">
        <Suspense fallback={<LoadingPage />}>
          <Outlet />
        </Suspense>
      </div>
    </>
  );
};

export default App;
