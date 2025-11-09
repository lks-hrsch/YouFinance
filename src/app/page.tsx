"use client";

import TransactionListComponent from "@/components/transaction-list";

export default function HomePage() {
  return (
    <>
      <h1 className="text-amber-800">Transaction List</h1>
      <TransactionListComponent />
    </>
  );
}
