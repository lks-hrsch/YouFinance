import { ArrowPathIcon } from "@heroicons/react/24/outline";
import { List, ListItem, Typography } from "@material-tailwind/react";
import { invoke } from "@tauri-apps/api/core";
import type React from "react";
import SettingsListMenu from "./SettingsListMenu";

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
      onPointerEnterCapture={undefined}
      onPointerLeaveCapture={undefined}
      placeholder={undefined}
    >
      <Typography
        as="a"
        className="font-medium"
        color="blue-gray"
        href="/"
        onPointerEnterCapture={undefined}
        onPointerLeaveCapture={undefined}
        placeholder={undefined}
        variant="small"
      >
        <ListItem
          className="flex items-center gap-2 py-2 pr-4"
          onPointerEnterCapture={undefined}
          onPointerLeaveCapture={undefined}
          placeholder={undefined}
        >
          Home
        </ListItem>
      </Typography>
      <SettingsListMenu />
      <div className="flex items-center justify-center rounded-lg p-2">
        {/* Refresh button */}
        <a onClick={fetchData}>
          <ArrowPathIcon className="h-4 w-4 text-gray-900" strokeWidth="2" />
        </a>
      </div>
    </List>
  );
};

export default ListMenu;
