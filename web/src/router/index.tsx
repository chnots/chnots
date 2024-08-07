// src/router/index.tsx
import { createBrowserRouter, Navigate } from "react-router-dom";
import type { RouteObject } from "react-router-dom";
import Toents from "@/pages/Toents";
import Chnots from "@/pages/Chnots";
import RootLayout from "@/layouts/RootLayout";
import Settings from "@/pages/Settings";

export enum Routes {
  ROOT = "/",
  Chnots = "/chnots",
  Toents = "/toents",
  Settings = "/settings",
}

const routes: RouteObject[] = [
  {
    path: Routes.ROOT,
    element: <RootLayout />,
    children: [
      {
        index: true,
        element: <Navigate to={Routes.Chnots} replace />,
      },
      {
        path: Routes.Chnots,
        element: <Chnots />,
      },
      {
        path: Routes.Toents,
        element: <Toents />,
      },
      {
        path: Routes.Settings,
        element: <Settings />,
      },
    ],
  },
];

const browserRoute = createBrowserRouter(routes, {
  basename: "/",
});

export default browserRoute;
