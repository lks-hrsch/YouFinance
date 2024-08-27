import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

import { ThemeProvider } from "@material-tailwind/react";

import Root from "./routes/Root";
import BankAccountDataProvider from "./routes/settings/BankAccountDataProvider";
import BankAccounts from "./routes/settings/BankAccounts";

import Menu from "./components/Navigation/Menu";

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

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider>
      <Menu />
      <RouterProvider router={router} />
    </ThemeProvider>
  </React.StrictMode>,
);
