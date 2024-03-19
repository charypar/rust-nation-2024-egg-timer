import React from "react";
import ReactDOM from "react-dom/client";

import init_core from "shared/shared";

import "./index.css";
import App from "./App.tsx";

init_core().then(() => {
  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
});
