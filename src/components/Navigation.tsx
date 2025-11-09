"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn } from "@/lib/utils";

export default function Navigation() {
  const pathname = usePathname();

  return (
    <nav className="flex gap-8 border-gray-300 border-b bg-gray-100 p-4">
      <h1 className="font-bold text-2xl text-amber-300">Navigation</h1>
      <Link
        className={cn(
          "font-medium no-underline transition-colors hover:text-primary",
          pathname === "/" ? "font-bold text-primary" : "text-gray-700"
        )}
        href="/"
      >
        Transactions
      </Link>
      <Link
        className={cn(
          "font-medium no-underline transition-colors hover:text-primary",
          pathname === "/settings/bank-account-data-provider"
            ? "font-bold text-primary"
            : "text-gray-700"
        )}
        href="/settings/bank-account-data-provider"
      >
        Providers
      </Link>
      <Link
        className={cn(
          "font-medium no-underline transition-colors hover:text-primary",
          pathname === "/settings/bank-accounts"
            ? "font-bold text-primary"
            : "text-gray-700"
        )}
        href="/settings/bank-accounts"
      >
        Bank Accounts
      </Link>
    </nav>
  );
}
