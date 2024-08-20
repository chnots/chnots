import clsx from "clsx";
import { Suspense, useEffect, useState } from "react";
import { Outlet, useLocation } from "react-router-dom";
import useLocalStorage from "react-use/lib/useLocalStorage";
import Navigation from "@/components/Navigation";
import useResponsiveWidth from "@/hooks/useResponsiveWidth";
import Loading from "@/pages/Loading";
import { Routes } from "@/router";
import useCurrentDomain from "@/hooks/useCurrentDomain";

const RootLayout = () => {
  const location = useLocation();
  const { sm } = useResponsiveWidth();
  const currentDomain = useCurrentDomain();
  const [lastVisited] = useLocalStorage<string>("lastVisited", "/home");
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    if (!currentDomain) {
      if (
        ([Routes.ROOT, Routes.Chnots, Routes.Toents] as string[]).includes(
          location.pathname
        )
      ) {
        window.location.href = Routes.Chnots;
        return;
      }
    } else {
      if (location.pathname === Routes.ROOT) {
        if (
          lastVisited &&
          ([Routes.Chnots, Routes.Toents] as string[]).includes(lastVisited)
        ) {
          window.location.href = lastVisited;
        } else {
          window.location.href = Routes.Chnots;
        }
        return;
      }
    }

    setInitialized(true);
  }, []);

  return !initialized ? (
    <Loading />
  ) : (
    <div className="w-full min-h-full">
      <div
        className={clsx(
          "w-full transition-all mx-auto flex flex-row justify-center items-start",
          "sm:pl-16"
        )}
      >
        {sm && (
          <div
            className={clsx(
              "group flex flex-col justify-start items-start fixed top-0 left-0 select-none border-r dark:border-zinc-800 h-full bg-zinc-50 dark:bg-zinc-800 dark:bg-opacity-40 transition-all hover:shadow-xl z-2",
              "w-16 px-2"
            )}
          >
            <Navigation className="!h-auto" />
            <div
              className={clsx(
                "w-full grow h-auto flex flex-col justify-end",
                "items-center"
              )}
            ></div>
          </div>
        )}
        <main className="w-full bg-gray-100 h-auto flex-grow shrink flex flex-col justify-start items-center">
          <Suspense fallback={<Loading />}>
            <Outlet />
          </Suspense>
        </main>
      </div>
    </div>
  );
};

export default RootLayout;
