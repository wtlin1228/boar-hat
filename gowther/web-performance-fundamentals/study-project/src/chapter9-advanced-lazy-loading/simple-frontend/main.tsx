import { StrictMode } from "react";

import { createRoot, hydrateRoot } from "react-dom/client";

import "./index.css";
import App from "./App";

hydrateRoot(document.getElementById("root")!, <App />);

// createRoot(document.getElementById("root")!).render(
//   <StrictMode>
//     <App />
//   </StrictMode>,
// );
