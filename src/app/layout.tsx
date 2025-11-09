import type { Metadata } from "next";
import Navigation from "@/components/navigation";
import "./globals.css";

export const metadata: Metadata = {
  title: "YouFinance",
  description: "Personal finance management",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body>
        <Navigation />
        <main className="p-8">{children}</main>
      </body>
    </html>
  );
}
