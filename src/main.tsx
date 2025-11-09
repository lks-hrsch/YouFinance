import ReactDOM from "react-dom/client";
import "./styles.css";

import { ThemeProvider } from "@material-tailwind/react";
import { attachConsole } from "@tauri-apps/plugin-log";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Menu from "./components/Navigation/Menu";
import Root from "./routes/Root";
import BankAccountDataProvider from "./routes/settings/BankAccountDataProvider";
import BankAccounts from "./routes/settings/BankAccounts";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
  },
  {
    path: "/settings/bank-account-data-provider",
    element: <BankAccountDataProvider />,
  },
  {
    path: "/settings/bank-accounts",
    element: <BankAccounts />,
  },
]);

// Attach console asynchronously without blocking render
attachConsole().catch(console.error);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <ThemeProvider>
    <Menu />
    <RouterProvider router={router} />
  </ThemeProvider>
);
