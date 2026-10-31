import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import "../styled-system/styles.css";
import App from "./App.tsx";
import { AppWrapper } from "./components/AppWrapper.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <AppWrapper theme="dark">
      <App />
    </AppWrapper>
  </StrictMode>,
);
