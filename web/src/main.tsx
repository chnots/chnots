import "@github/relative-time-element";
import { createRoot } from "react-dom/client";
import "@/styles/tailwind.css";
import "@/styles/global.css";

import { RouterProvider } from "react-router-dom";

import browserRoute from "./router";

(async () => {
  const container = document.getElementById("root");
  const root = createRoot(container as HTMLElement);
  root.render(<RouterProvider router={browserRoute} />);
})();
