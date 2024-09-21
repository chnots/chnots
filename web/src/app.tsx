import { Suspense, useEffect, useState } from "react";
import { Outlet, useLocation } from "react-router-dom";
import useLocalStorage from "react-use/lib/useLocalStorage";
import useResponsiveWidth from "@/hooks/use-responsive-width";
import useCurrentNamespace from "@/hooks/use-current-namespace";
import LoadingPage from "@/pages/loading-page";
import Navigation from "@/components/navigation";
import { RoutePaths } from "@/router";

const App = () => {
  const location = useLocation();
  const { sm } = useResponsiveWidth();
  const currentNamespace = useCurrentNamespace();
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
        <div className="w-16 px-2 group flex flex-col justify-start items-start fixed top-0 left-0 select-none border-r dark:border-zinc-800 h-full bg-zinc-50 dark:bg-zinc-800 dark:bg-opacity-40 transition-all hover:shadow-xl z-2">
          <Navigation className="!h-auto" />
        </div>
      )}
      <div className="h-screen max-h-screen sm:pl-16">
        <Suspense fallback={<LoadingPage />}>
          <Outlet />
        </Suspense>
      </div>
    </>
  );
};

export default App;
