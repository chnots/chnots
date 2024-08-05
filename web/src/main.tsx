import { createRoot } from "react-dom/client";
import "@/styles/tailwind.css";
import "@/styles/global.css";

import App from "./App";

(async () => {
  const container = document.getElementById("root");
  const root = createRoot(container as HTMLElement);
  root.render(<App />);
})();
