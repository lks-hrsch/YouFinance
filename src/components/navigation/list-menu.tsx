"use client";

import { invoke } from "@tauri-apps/api/core";
import { RefreshCw } from "lucide-react";
import Link from "next/link";
import { Button } from "@/components/ui/button";
import SettingsListMenu from "./settings-list-menu";

export default function ListMenu() {
  const fetchData = async () => {
    try {
      await invoke("sync_all_accounts");
    } catch (error) {
      console.error("Failed to fetch accounts:", error);
    }

    // refresh the page
    window.location.reload();
  };

  return (
    <nav className="mt-4 mb-6 flex items-center gap-1 p-0 lg:mt-0 lg:mb-0 lg:p-1">
      <Link
        className="rounded-full px-3 py-2 font-medium text-sm hover:bg-slate-100"
        href="/"
      >
        Home
      </Link>
      <SettingsListMenu />
      <Button
        className="ml-auto rounded-full"
        onClick={fetchData}
        size="icon"
        variant="ghost"
      >
        <RefreshCw className="size-4" />
      </Button>
    </nav>
  );
}
