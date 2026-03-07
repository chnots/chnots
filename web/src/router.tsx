// src/router/index.tsx

import type { RouteObject } from "react-router-dom";
import { createBrowserRouter, Navigate } from "react-router-dom";
import { Toaster } from "sonner";
import App from "@/app";
import ErrorPage from "@/common/pages/error-page";
import SettingsPage from "@/common/pages/settings-page";
import ToentPage from "@/krate/toent/component/toent-page";
import ChnotPage from "@/krate/chnot/page/chnot-page";
import FullScreenTimer from "./krate/timer/timer";

export enum RoutePaths {
  ROOT = "/",
  Chnots = "/chnots",
  Toents = "/toents",
  LLMChat = "/llmchat",
  Settings = "/settings",
  Timer = "/timer",
  ToolExcalidraw = "/tool/excalidraw",
  KFiles = "/kfiles",
  Pad = "/pad",
}

const routes: RouteObject[] = [
  {
    path: RoutePaths.ROOT,
    element: (
      <>
        <App />
        <Toaster />
      </>
    ),
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <Navigate to={RoutePaths.Chnots} replace />,
      },
      {
        path: RoutePaths.Toents,
        element: <ToentPage />,
      },
      {
        path: RoutePaths.Settings,
        element: <SettingsPage />,
      },
      {
        path: RoutePaths.Timer,
        element: <FullScreenTimer />,
      },
      {
        path: RoutePaths.Chnots,
        element: <ChnotPage />,
      },
      /*       {
        path: RoutePaths.Pad,
        element: <Pad />,
      }, */
    ],
  },
];

const browserRoute = createBrowserRouter(routes, {
  basename: "/",
});

export default browserRoute;
