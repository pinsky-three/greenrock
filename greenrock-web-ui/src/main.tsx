import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import { ErrorBoundary } from "./components/ErrorBoundary.tsx";

createRoot(document.getElementById("root")!).render(
  // <StrictMode> {/* Disabled temporarily to avoid double API calls in development */}
  <ErrorBoundary>
    <App />
  </ErrorBoundary>
  // </StrictMode>
);
