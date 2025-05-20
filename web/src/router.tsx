// src/router/index.tsx
import { createBrowserRouter, Navigate } from "react-router-dom";
import type { RouteObject } from "react-router-dom";
import ToentPage from "@/common/pages/toent-page";
import ChnotPage from "@/features/chnot/page/chnot";
import SettingsPage from "@/common/pages/settings-page";
import App from "@/app";
import { Toaster } from "sonner";
import ErrorPage from "@/common/pages/error-page";
import LLMChatPage from "./features/llmchat/page/llmchat";
import FullScreenTimer from "./features/timer/timer";
import ExcalidrawPage from "./features/tool/excalidraw/page/excalidraw";
import ResourcePage from "./features/resource/page";

export enum RoutePaths {
  ROOT = "/",
  Chnots = "/chnots",
  Toents = "/toents",
  LLMChat = "/llmchat",
  Settings = "/settings",
  Timer = "/timer",
  ToolExcalidraw = "/tool/excalidraw",
  Resources = "/resources",
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
        path: RoutePaths.Chnots,
        element: <ChnotPage />,
      },
      {
        path: RoutePaths.LLMChat,
        element: <LLMChatPage />,
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
        path: RoutePaths.ToolExcalidraw,
        element: <ExcalidrawPage />,
      },
      {
        path: RoutePaths.Resources,
        element: <ResourcePage />,
      },
    ],
  },
];

const browserRoute = createBrowserRouter(routes, {
    basename: "/",
});

export default browserRoute;
