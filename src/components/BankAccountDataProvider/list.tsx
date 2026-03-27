import { invoke } from "@tauri-apps/api/core";
import { Pencil, Trash2 } from "lucide-react";
import type React from "react";
import { useCallback, useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import type { Provider } from "@/models/typeshare_definitions";
import ProviderEditDialog from "./edit-dialog";

const TABLE_HEAD = ["ID", "Provider", "Actions"];

const BankAccountDataProviderList: React.FC = () => {
  const [providers, setProviders] = useState<Provider[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [editingProvider, setEditingProvider] = useState<Provider | null>(null);

  const fetchProviders = useCallback(() => {
    setIsLoading(true);
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
  }, []);

  useEffect(() => {
    fetchProviders();
  }, [fetchProviders]);

  const handleDelete = (providerId: number) => {
    console.log(`Delete provider with ID: ${providerId}`);
    invoke("delete_banking_provider", { providerId })
      .then(() => fetchProviders())
      .catch((error) => console.error("Failed to delete provider:", error));
  };

  if (isLoading) {
    return (
      <div className="p-4 text-slate-600 text-sm">Loading providers...</div>
    );
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
                          {provider.name}
                        </span>
                      </td>
                      <td className={classes}>
                        <div className="flex gap-2">
                          <Button
                            onClick={() => setEditingProvider(provider)}
                            size="icon"
                            title="Edit Provider"
                            variant="ghost"
                          >
                            <Pencil className="size-4" />
                          </Button>
                          <Button
                            className="text-slate-900 hover:text-red-600"
                            onClick={() => handleDelete(provider.id)}
                            size="icon"
                            title="Delete Provider"
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

      <ProviderEditDialog
        provider={editingProvider}
        onClose={() => setEditingProvider(null)}
        onSaved={fetchProviders}
      />
    </div>
  );
};

export default BankAccountDataProviderList;
