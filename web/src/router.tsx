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

export enum RoutePaths {
  ROOT = "/",
  Chnots = "/chnots",
  Toents = "/toents",
  LLMChat = "/llmchat",
  Settings = "/settings",
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
    ],
  },
];

const browserRoute = createBrowserRouter(routes, {
    basename: "/",
});

export default browserRoute;
