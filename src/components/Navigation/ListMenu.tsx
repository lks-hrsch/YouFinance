import React from "react";
import SettingsListMenu from "./SettingsListMenu";
import { invoke } from "@tauri-apps/api/tauri";

import { ArrowPathIcon } from "@heroicons/react/24/outline";

import { Typography, List, ListItem } from "@material-tailwind/react";

const ListMenu: React.FC = () => {
  const fetchData = async () => {
    try {
      await invoke("get_transactions_handler");
    } catch (error) {
      console.error("Failed to fetch accounts:", error);
    }

    // refresh the page
    window.location.reload();
  };

  return (
    <List
      className="mt-4 mb-6 p-0 lg:mt-0 lg:mb-0 lg:flex-row lg:p-1"
      placeholder={undefined}
    >
      <Typography
        as="a"
        href="/"
        variant="small"
        color="blue-gray"
        className="font-medium"
        placeholder={undefined}
      >
        <ListItem
          className="flex items-center gap-2 py-2 pr-4"
          placeholder={undefined}
        >
          Home
        </ListItem>
      </Typography>
      <SettingsListMenu />
      <div className="flex items-center justify-center rounded-lg p-2 ">
        {/* Refresh button */}
        <a onClick={fetchData}>
          <ArrowPathIcon strokeWidth="2" className="h-4 w-4 text-gray-900" />
        </a>
      </div>
    </List>
  );
};

export default ListMenu;
