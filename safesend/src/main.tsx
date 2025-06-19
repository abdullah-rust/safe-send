// 🧠 Global JS error handler
window.onerror = function (msg, src, line, col, err) {
  console.log("💥 JS Error:", msg, src, line, col, err);
};

import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { BrowserRouter } from "react-router-dom";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <BrowserRouter>
    <React.StrictMode>
      <App />
    </React.StrictMode>
  </BrowserRouter>
);

setTimeout(() => {
  const splash = document.getElementById("splash");
  if (splash) splash.remove();
}, 300);
