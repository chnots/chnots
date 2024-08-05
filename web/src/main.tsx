import { createRoot } from "react-dom/client";
import "@/styles/tailwind.css";
import "@/styles/global.css";

import Chnots from "./pages/Chnots";

(async () => {
  const container = document.getElementById("root");
  const root = createRoot(container as HTMLElement);
  root.render(<Chnots />);
})();
