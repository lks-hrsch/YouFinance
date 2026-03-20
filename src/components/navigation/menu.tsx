"use client";

import { Menu as MenuIcon, X } from "lucide-react";
import Link from "next/link";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import ListMenu from "./list-menu";

export default function Menu() {
  const [openNav, setOpenNav] = useState(false);

  useEffect(() => {
    const handleResize = () => {
      if (window.innerWidth >= 960) {
        setOpenNav(false);
      }
    };
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return (
    <nav className="mx-auto mb-4 rounded-lg border bg-white px-4 py-2 shadow-sm">
      <div className="flex items-center justify-between">
        <Link
          className="cursor-pointer py-1.5 font-semibold text-lg lg:ml-2"
          href="/"
        >
          youfinance
        </Link>
        <div className="hidden lg:block">
          <ListMenu />
        </div>
        <Button
          className="lg:hidden"
          onClick={() => setOpenNav(!openNav)}
          size="icon"
          variant="ghost"
        >
          {openNav ? <X className="size-6" /> : <MenuIcon className="size-6" />}
        </Button>
      </div>
      {openNav && (
        <div className="mt-2 lg:hidden">
          <ListMenu />
        </div>
      )}
    </nav>
  );
}
