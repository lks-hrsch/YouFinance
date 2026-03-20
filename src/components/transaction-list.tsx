"use client";

import { invoke } from "@tauri-apps/api/core";
import type React from "react";
import { useEffect, useState } from "react";
import { Card, CardContent } from "@/components/ui/card";
import type { Transaction } from "../models/typeshare_definitions";

const TransactionListComponent: React.FC = () => {
  const [transactions, setTransactions] = useState<Transaction[]>([]);

  useEffect(() => {
    const fetchTransactions = async () => {
      try {
        console.log("Fetching transactions...");
        const rustTransactions =
          await invoke<Transaction[]>("get_transactions");
        setTransactions(rustTransactions);
      } catch (err) {
        console.error("Failed to fetch transactions:", err);
      }
    };
    fetchTransactions();
  }, []);

  return (
    <Card>
      <CardContent className="p-0">
        <div className="overflow-x-auto">
          <table className="w-full table-auto text-left">
            <thead>
              <tr className="border-b bg-slate-50">
                <th className="p-4 font-semibold text-slate-900 text-sm">
                  Booking Date
                </th>
                <th className="p-4 font-semibold text-slate-900 text-sm">
                  Debtor
                </th>
                <th className="p-4 font-semibold text-slate-900 text-sm">
                  Creditor
                </th>
                <th className="p-4 text-right font-semibold text-slate-900 text-sm">
                  Amount
                </th>
              </tr>
            </thead>
            <tbody>
              {transactions.map((transaction, index) => {
                const isLast = index === transactions.length - 1;
                const rowClasses = isLast ? "" : "border-b border-slate-100";

                return (
                  <tr
                    className={rowClasses}
                    key={transaction.id.toString()}
                  >
                    <td className="p-4">
                      <span className="font-normal text-slate-700 text-sm">
                        {transaction.date}
                      </span>
                    </td>
                    <td className="p-4">
                      <div className="flex flex-col">
                        <span className="font-normal text-slate-900 text-sm">
                          {transaction.debitor_name}
                        </span>
                        <span className="text-slate-500 text-xs">
                          {transaction.debitor_iban}
                        </span>
                      </div>
                    </td>
                    <td className="p-4">
                      <div className="flex flex-col">
                        <span className="font-normal text-slate-900 text-sm">
                          {transaction.creditor_name}
                        </span>
                        <span className="text-slate-500 text-xs">
                          {transaction.creditor_iban}
                        </span>
                      </div>
                    </td>
                    <td className="p-4 text-right">
                      <div className="inline-flex items-center gap-1">
                        {transaction.amount < 0 ? (
                          <span className="font-medium text-red-600 text-sm">
                            {transaction.amount}
                          </span>
                        ) : (
                          <span className="font-medium text-green-600 text-sm">
                            +{transaction.amount}
                          </span>
                        )}
                        <span className="text-slate-600 text-sm">
                          {transaction.currency}
                        </span>
                      </div>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </CardContent>
    </Card>
  );
};

export default TransactionListComponent;
