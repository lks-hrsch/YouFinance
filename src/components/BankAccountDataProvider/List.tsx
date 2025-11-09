"use client";

import { invoke } from "@tauri-apps/api/core";
import type React from "react";
import { useEffect, useState } from "react";
import { Card, CardContent } from "@/components/ui/card";
import type { Provider } from "../../models/typeshare_definitions";

const TABLE_HEAD = ["ID", "Provider", "Secret ID", "Secret Key"];

const BankAccountDataProviderList: React.FC = () => {
  const [providers, setProviders] = useState<Provider[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);

  useEffect(() => {
    const fetchProviders = () => {
      invoke("get_banking_providers")
        .then((rustBankingProviders: unknown) => {
          const bankingProviders = rustBankingProviders as Provider[];
          setProviders(bankingProviders);
        })
        .catch((error) => {
          console.error("Failed to fetch providers:", error);
        })
        .finally(() => {
          setIsLoading(false);
        });
    };

    fetchProviders();
  }, []);

  if (isLoading) {
    return <div className="p-4">Loading...</div>;
  }

  return (
    <div className="w-full">
      <Card>
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
                {providers.map((provider, index) => {
                  const isLast = index === providers.length - 1;
                  const classes = isLast
                    ? "p-4"
                    : "border-b border-slate-100 p-4";

                  return (
                    <tr key={provider.id}>
                      <td className={classes}>
                        <span className="font-normal text-slate-700 text-sm">
                          {provider.id}
                        </span>
                      </td>
                      <td className={classes}>
                        <span className="font-normal text-slate-700 text-sm">
                          {provider.title}
                        </span>
                      </td>
                      <td className={classes}>
                        <span className="font-normal text-slate-700 text-sm">
                          {provider.secret_id}
                        </span>
                      </td>
                      <td className={classes}>
                        <span className="font-normal text-slate-700 text-sm">
                          {provider.secret_key}
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
    </div>
  );
};

export default BankAccountDataProviderList;
