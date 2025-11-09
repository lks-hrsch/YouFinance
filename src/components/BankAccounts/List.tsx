"use client";

import { invoke } from "@tauri-apps/api/core";
import { Trash2 } from "lucide-react";
import type React from "react";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import type { Account } from "../../models/typeshare_definitions";

const TABLE_HEAD = ["ID", "Title", "Institution ID", "Account ID", "IBAN"];

const BankAccountsList: React.FC = () => {
  const [groupedAccounts, setGroupedAccounts] = useState<
    Record<string, Account[]>
  >({});

  useEffect(() => {
    const fetchProviders = () => {
      invoke("get_banking_accounts")
        .then((rustBankAccounts: unknown) => {
          const bankAccounts = rustBankAccounts as Account[];
          const grouped = bankAccounts.reduce(
            (acc, account) => {
              if (!acc[account.bank_connection_id]) {
                acc[account.bank_connection_id] = [];
              }
              acc[account.bank_connection_id].push(account);
              return acc;
            },
            {} as Record<string, Account[]>
          );
          setGroupedAccounts(grouped);
        })
        .catch((error) => {
          console.error("Failed to fetch accounts:", error);
        });
    };

    fetchProviders();
  }, []);

  const handleDelete = (bankConnectionId: string) => {
    console.log(`Delete bank connection with ID: ${bankConnectionId}`);
    invoke("disconnect_bank_account", {
      providerTitle: "GoCardless",
      bankConnectionId,
    }).catch((error) => {
      console.error("Failed to disconnect bank account:", error);
    });
  };

  return (
    <div className="space-y-4">
      {Object.entries(groupedAccounts).map(([bankConnectionId, accounts]) => (
        <Card key={bankConnectionId}>
          <CardHeader className="flex flex-row items-center justify-between">
            <h3 className="font-semibold text-lg text-slate-900">
              Bank Connection ID: {bankConnectionId}
            </h3>
            <Button
              onClick={() => handleDelete(bankConnectionId)}
              size="icon"
              variant="destructive"
            >
              <Trash2 className="size-4" />
            </Button>
          </CardHeader>
          <CardContent className="p-0">
            <div className="overflow-x-auto">
              <table className="w-full min-w-max table-auto text-left">
                <thead>
                  <tr className="border-b bg-slate-50">
                    {TABLE_HEAD.map((head) => (
                      <th
                        className="p-4 font-normal text-slate-600 text-sm leading-none"
                        key={head}
                      >
                        {head}
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {accounts.map((account, index) => {
                    const isLast = index === accounts.length - 1;
                    const classes = isLast
                      ? "p-4"
                      : "border-b border-slate-100 p-4";

                    return (
                      <tr key={account.id}>
                        <td className={classes}>
                          <span className="font-normal text-slate-700 text-sm">
                            {account.id}
                          </span>
                        </td>
                        <td className={classes}>
                          <span className="font-normal text-slate-700 text-sm">
                            {account.title}
                          </span>
                        </td>
                        <td className={classes}>
                          <span className="font-normal text-slate-700 text-sm">
                            {account.institution_id}
                          </span>
                        </td>
                        <td className={classes}>
                          <span className="font-normal text-slate-700 text-sm">
                            {account.account_id}
                          </span>
                        </td>
                        <td className={classes}>
                          <span className="font-normal text-slate-700 text-sm">
                            {account.iban}
                          </span>
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
          </CardContent>
        </Card>
      ))}
    </div>
  );
};

export default BankAccountsList;
