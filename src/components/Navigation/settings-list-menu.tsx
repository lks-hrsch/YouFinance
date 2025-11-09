"use client";

import { ChevronDown } from "lucide-react";
import Link from "next/link";
import { useState } from "react";

const navListMenuItems = [
  {
    title: "BankAccountDataProvider",
    description: "Configure APIs to retrieve bank account data.",
    href: "/settings/bank-account-data-provider",
  },
  {
    title: "BankAccounts",
    description: "Configure Bank accounts",
    href: "/settings/bank-accounts",
  },
];

export default function SettingsListMenu() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="relative">
      <button
        className="flex items-center gap-1 rounded-full px-3 py-2 font-medium text-sm outline-none hover:bg-slate-100"
        onClick={() => setIsOpen(!isOpen)}
        type="button"
      >
        Settings
        <ChevronDown className="size-3" />
      </button>
      {isOpen && (
        <>
          <button
            className="fixed inset-0 cursor-default"
            onClick={() => setIsOpen(false)}
            type="button"
          >
            <span className="sr-only">Close menu</span>
          </button>
          <div className="absolute top-full left-0 z-50 mt-2 w-64 rounded-md border bg-white shadow-lg">
            {navListMenuItems.map(({ title, description, href }) => (
              <Link
                className="flex flex-col items-start p-3 hover:bg-slate-50"
                href={href}
                key={href}
                onClick={() => setIsOpen(false)}
              >
                <span className="font-bold text-sm">{title}</span>
                <span className="text-slate-500 text-xs">{description}</span>
              </Link>
            ))}
          </div>
        </>
      )}
    </div>
  );
}
