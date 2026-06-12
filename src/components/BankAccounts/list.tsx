"use client";

import { invoke } from "@tauri-apps/api/core";
import { FolderOpen, RefreshCw, Trash2 } from "lucide-react";
import type React from "react";
import { useCallback, useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import type { BankAccountWithProvider } from "../../models/typeshare_definitions";

const TABLE_HEAD = [
  "ID",
  "Title",
  "Institution ID",
  "Account ID",
  "IBAN",
  "Last Synced",
  "Actions",
];

function getRelativeTime(isoString: string | null | undefined): string {
  if (!isoString) {
    return "Never synced";
  }

  const date = new Date(isoString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60_000);
  const diffHours = Math.floor(diffMs / 3_600_000);
  const diffDays = Math.floor(diffMs / 86_400_000);

  if (diffMins < 1) {
    return "Just now";
  }
  if (diffMins < 60) {
    return `${diffMins}m ago`;
  }
  if (diffHours < 24) {
    return `${diffHours}h ago`;
  }
  if (diffDays < 30) {
    return `${diffDays}d ago`;
  }

  return date.toLocaleDateString();
}

interface BankAccountsListProps {
  refreshTrigger?: number;
}

const BankAccountsList: React.FC<BankAccountsListProps> = ({
  refreshTrigger = 0,
}) => {
  const [groupedAccounts, setGroupedAccounts] = useState<
    Record<string, BankAccountWithProvider[]>
  >({});
  const [syncingIds, setSyncingIds] = useState<Set<string | number>>(new Set());

  const fetchAccounts = useCallback(() => {
    invoke("get_banking_accounts")
      .then((rustBankAccounts: unknown) => {
        const bankAccounts = rustBankAccounts as BankAccountWithProvider[];
        const grouped = bankAccounts.reduce(
          (acc, item) => {
            const key = item.bank_account_provider.bank_connection_id;
            if (!acc[key]) {
              acc[key] = [];
            }
            acc[key].push(item);
            return acc;
          },
          {} as Record<string, BankAccountWithProvider[]>
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

  const handleDelete = (item: BankAccountWithProvider) => {
    if (
      item.provider_name === "LocalCSV" ||
      item.provider_name === "LocalMTA"
    ) {
      invoke("delete_bank_account", { accountId: item.bank_account.id })
        .then(() => fetchAccounts())
        .catch((error) =>
          console.error("Failed to delete local account:", error)
        );
    } else {
      invoke("disconnect_bank_account", {
        providerTitle: item.provider_name,
        bankConnectionId: item.bank_account_provider.bank_connection_id,
      })
        .then(() => fetchAccounts())
        .catch((error) =>
          console.error("Failed to disconnect bank account:", error)
        );
    }
  };

  const handleOpenFolder = (item: BankAccountWithProvider) => {
    if (item.bank_account_provider.institution_id && item.bank_account.iban) {
      invoke("open_account_directory", {
        bankName: item.bank_account_provider.institution_id,
        iban: item.bank_account.iban,
      }).catch((error) => {
        console.error("Failed to open directory:", error);
      });
    }
  };

  const handleSyncAccount = (accountId: number) => {
    setSyncingIds((prev) => new Set(prev).add(accountId));
    invoke("sync_account", { targetAccountId: accountId })
      .then(() => {
        console.log(`Synced account ${accountId}`);
        fetchAccounts();
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
        fetchAccounts();
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
            <CardHeader className="flex flex-row items-center justify-between border-b bg-slate-50/50 px-6 py-3">
              <h3 className="font-semibold text-slate-900 text-sm uppercase tracking-wider">
                {firstItem.provider_name === "LocalCSV"
                  ? "Local CSV Provider"
                  : firstItem.provider_name === "LocalMTA"
                    ? "Local MTA Provider"
                    : `Bank Connection: ${bankConnectionId}`}
              </h3>
              <Button
                className="h-8 gap-2 text-xs"
                disabled={isSyncingProvider}
                onClick={() =>
                  handleSyncProvider(
                    firstItem.bank_account_provider.provider_id,
                    bankConnectionId
                  )
                }
                size="sm"
                variant="outline"
              >
                <RefreshCw
                  className={`size-3 ${isSyncingProvider ? "animate-spin" : ""}`}
                />
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
                      const {
                        bank_account,
                        bank_account_provider,
                        provider_name,
                      } = item;
                      const isLast = index === items.length - 1;
                      const classes = isLast
                        ? "p-4"
                        : "border-b border-slate-100 p-4";
                      const isSyncingAccount = syncingIds.has(bank_account.id);

                      return (
                        <tr
                          className="transition-colors hover:bg-slate-50"
                          key={bank_account.id}
                        >
                          <td className={classes}>
                            <span className="font-normal text-slate-700 text-sm">
                              {bank_account.id}
                            </span>
                          </td>
                          <td className={classes}>
                            <span className="font-medium text-slate-900 text-sm">
                              {bank_account.name}
                            </span>
                          </td>
                          <td className={classes}>
                            <span className="font-normal text-slate-600 text-sm">
                              {bank_account_provider.institution_id}
                            </span>
                          </td>
                          <td className={classes}>
                            <span className="font-normal text-slate-600 text-sm">
                              {bank_account_provider.external_account_id}
                            </span>
                          </td>
                          <td className={classes}>
                            <span className="rounded bg-slate-100 px-1.5 py-0.5 font-mono text-slate-500 text-xs">
                              {bank_account.iban}
                            </span>
                          </td>
                          <td className={classes}>
                            <span className="font-normal text-slate-600 text-sm">
                              {getRelativeTime(
                                bank_account_provider.last_synced_at
                              )}
                            </span>
                          </td>
                          <td className={classes}>
                            <div className="flex gap-2">
                              <Button
                                className="size-8 text-slate-700 hover:text-blue-600"
                                disabled={isSyncingAccount}
                                onClick={() =>
                                  handleSyncAccount(bank_account.id)
                                }
                                size="icon"
                                title="Sync Transactions"
                                variant="ghost"
                              >
                                <RefreshCw
                                  className={`size-4 ${isSyncingAccount ? "animate-spin" : ""}`}
                                />
                              </Button>
                              {(provider_name === "LocalCSV" ||
                                provider_name === "LocalMTA") && (
                                <Button
                                  className="size-8 text-slate-700 hover:text-emerald-600"
                                  onClick={() => handleOpenFolder(item)}
                                  size="icon"
                                  title="Open in Finder"
                                  variant="ghost"
                                >
                                  <FolderOpen className="size-4" />
                                </Button>
                              )}
                              <Button
                                className="size-8 text-slate-900 hover:text-red-600"
                                onClick={() => handleDelete(item)}
                                size="icon"
                                title="Delete Account"
                                variant="ghost"
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
