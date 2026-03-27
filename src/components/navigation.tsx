"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn } from "@/lib/utils";

export default function Navigation() {
  const pathname = usePathname();

  return (
    <nav className="flex gap-8 border-gray-300 border-b bg-gray-100 p-4">
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
          pathname === "/settings" ? "font-bold text-primary" : "text-gray-700"
        )}
        href="/settings"
      >
        Settings
      </Link>
    </nav>
  );
}
