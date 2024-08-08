import "@github/relative-time-element";
import { createRoot } from "react-dom/client";
import "@/styles/tailwind.css";
import "@/styles/global.css";

import { CssVarsProvider } from "@mui/joy";
import { RouterProvider } from "react-router-dom";
import router from "./router";
import theme from "./theme";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

(async () => {
  const container = document.getElementById("root");
  const root = createRoot(container as HTMLElement);
  root.render(
    <CssVarsProvider theme={theme}>
      <RouterProvider router={router} />
      {/*       <Toaster
        position="top-right"
        toastOptions={{ className: "dark:bg-zinc-700 dark:text-gray-300" }}
      /> */}
    </CssVarsProvider>
  );
})();
