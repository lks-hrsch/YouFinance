"use client";

import { invoke } from "@tauri-apps/api/core";
import { Trash2, FolderOpen, RefreshCw } from "lucide-react";
import type React from "react";
import { useEffect, useState, useCallback } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import type { AccountWithProvider } from "../../models/typeshare_definitions";

const TABLE_HEAD = ["ID", "Title", "Institution ID", "Account ID", "IBAN", "Actions"];

interface BankAccountsListProps {
  refreshTrigger?: number;
}

const BankAccountsList: React.FC<BankAccountsListProps> = ({ refreshTrigger = 0 }) => {
  const [groupedAccounts, setGroupedAccounts] = useState<
    Record<string, AccountWithProvider[]>
  >({});
  const [syncingIds, setSyncingIds] = useState<Set<string | number>>(new Set());

  const fetchAccounts = useCallback(() => {
    invoke("get_banking_accounts")
      .then((rustBankAccounts: unknown) => {
        const bankAccounts = rustBankAccounts as AccountWithProvider[];
        const grouped = bankAccounts.reduce(
          (acc, item) => {
            const key = item.account.bank_connection_id;
            if (!acc[key]) {
              acc[key] = [];
            }
            acc[key].push(item);
            return acc;
          },
          {} as Record<string, AccountWithProvider[]>
        );
        setGroupedAccounts(grouped);
      })
      .catch((error) => {
        console.error("Failed to fetch accounts:", error);
      });
  }, []);

  useEffect(() => {
    fetchAccounts();
  }, [fetchAccounts, refreshTrigger]);

  const handleDelete = (item: AccountWithProvider) => {
    if (item.provider_title === "LocalCSV") {
      invoke("delete_bank_account", { accountId: item.account.id })
        .then(() => fetchAccounts())
        .catch((error) => console.error("Failed to delete local account:", error));
    } else {
      invoke("disconnect_bank_account", {
        providerTitle: item.provider_title,
        bankConnectionId: item.account.bank_connection_id,
      })
        .then(() => fetchAccounts())
        .catch((error) => console.error("Failed to disconnect bank account:", error));
    }
  };

  const handleOpenFolder = (item: AccountWithProvider) => {
    if (item.account.institution_id && item.account.iban) {
      invoke("open_account_directory", {
        bankName: item.account.institution_id,
        iban: item.account.iban,
      }).catch((error) => {
        console.error("Failed to open directory:", error);
      });
    }
  };

  const handleSyncAccount = (accountId: number) => {
    setSyncingIds((prev) => new Set(prev).add(accountId));
    invoke("sync_account", { accId: accountId })
      .then(() => {
        console.log(`Synced account ${accountId}`);
      })
      .catch((error) => {
        console.error(`Failed to sync account ${accountId}:`, error);
      })
      .finally(() => {
        setSyncingIds((prev) => {
          const next = new Set(prev);
          next.delete(accountId);
          return next;
        });
      });
  };

  const handleSyncProvider = (providerId: number, bankConnectionId: string) => {
    setSyncingIds((prev) => new Set(prev).add(bankConnectionId));
    invoke("sync_provider_accounts", { pId: providerId })
      .then(() => {
        console.log(`Synced accounts for provider ${providerId}`);
      })
      .catch((error) => {
        console.error(`Failed to sync provider ${providerId}:`, error);
      })
      .finally(() => {
        setSyncingIds((prev) => {
          const next = new Set(prev);
          next.delete(bankConnectionId);
          return next;
        });
      });
  };

  return (
    <div className="space-y-4">
      {Object.entries(groupedAccounts).map(([bankConnectionId, items]) => {
        const firstItem = items[0];
        const isSyncingProvider = syncingIds.has(bankConnectionId);

        return (
          <Card key={bankConnectionId}>
            <CardHeader className="flex flex-row items-center justify-between border-b bg-slate-50/50 py-3 px-6">
              <h3 className="font-semibold text-sm text-slate-900 uppercase tracking-wider">
                {firstItem.provider_title === "LocalCSV" ? "Local CSV Provider" : `Bank Connection: ${bankConnectionId}`}
              </h3>
              <Button
                onClick={() => handleSyncProvider(firstItem.account.provider_id, bankConnectionId)}
                size="sm"
                variant="outline"
                className="h-8 text-xs gap-2"
                disabled={isSyncingProvider}
              >
                <RefreshCw className={`size-3 ${isSyncingProvider ? "animate-spin" : ""}`} />
                {isSyncingProvider ? "Syncing..." : "Sync All"}
              </Button>
            </CardHeader>
            <CardContent className="p-0">
              <div className="overflow-x-auto">
                <table className="w-full min-w-max table-auto text-left">
                  <thead>
                    <tr className="border-b bg-slate-50">
                      {TABLE_HEAD.map((head) => (
                        <th
                          className="p-4 font-normal text-slate-600 text-xs leading-none"
                          key={head}
                        >
                          {head}
                        </th>
                      ))}
                    </tr>
                  </thead>
                  <tbody>
                    {items.map((item, index) => {
                      const { account, provider_title } = item;
                      const isLast = index === items.length - 1;
                      const classes = isLast
                        ? "p-4"
                        : "border-b border-slate-100 p-4";
                      const isSyncingAccount = syncingIds.has(account.id);

                      return (
                        <tr key={account.id} className="hover:bg-slate-50 transition-colors">
                          <td className={classes}>
                            <span className="font-normal text-slate-700 text-sm">
                              {account.id}
                            </span>
                          </td>
                          <td className={classes}>
                            <span className="font-medium text-slate-900 text-sm">
                              {account.title}
                            </span>
                          </td>
                          <td className={classes}>
                            <span className="font-normal text-slate-600 text-sm">
                              {account.institution_id}
                            </span>
                          </td>
                          <td className={classes}>
                            <span className="font-normal text-slate-600 text-sm">
                              {account.account_id}
                            </span>
                          </td>
                          <td className={classes}>
                            <span className="font-mono text-slate-500 text-xs bg-slate-100 px-1.5 py-0.5 rounded">
                              {account.iban}
                            </span>
                          </td>
                          <td className={classes}>
                            <div className="flex gap-2">
                              <Button
                                onClick={() => handleSyncAccount(account.id)}
                                size="icon"
                                variant="ghost"
                                className="size-8 text-slate-700 hover:text-blue-600"
                                title="Sync Transactions"
                                disabled={isSyncingAccount}
                              >
                                <RefreshCw className={`size-4 ${isSyncingAccount ? "animate-spin" : ""}`} />
                              </Button>
                              {provider_title === "LocalCSV" && (
                                <Button
                                  onClick={() => handleOpenFolder(item)}
                                  size="icon"
                                  variant="ghost"
                                  className="size-8 text-slate-700 hover:text-emerald-600"
                                  title="Open in Finder"
                                >
                                  <FolderOpen className="size-4" />
                                </Button>
                              )}
                              <Button
                                onClick={() => handleDelete(item)}
                                size="icon"
                                variant="ghost"
                                className="size-8 text-slate-900 hover:text-red-600"
                                title="Delete Account"
                              >
                                <Trash2 className="size-4" />
                              </Button>
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
      })}
    </div>
  );
};

export default BankAccountsList;
