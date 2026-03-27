"use client";

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import { Plus } from "lucide-react";
import { useEffect, useState } from "react";
import BankAccountDataProviderAddModal from "@/components/BankAccountDataProvider/add-modal";
import BankAccountDataProviderList from "@/components/BankAccountDataProvider/list";
import BankAccountDataProviderSelect from "@/components/BankAccountDataProvider/select";
import BankAccountsList from "@/components/BankAccounts/list";
import BankAccountsSelect from "@/components/BankAccounts/select";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import type { BankConnectionInfo } from "@/models/typeshare_definitions";

export default function SettingsPage() {
  const [isModalOpen, setModalOpen] = useState(false);
  const [providerName, setProviderName] = useState<string>("");
  const [institutionID, setInstitutionID] = useState<string>("");
  const [requisitionID, setRequisitionID] = useState<string>("");
  const [bankName, setBankName] = useState<string>("");
  const [iban, setIban] = useState<string>("");
  const [refreshTrigger, setRefreshTrigger] = useState<number>(0);
  const [isConnecting, setIsConnecting] = useState<boolean>(false);

  const handleProviderSelect = (provider: string) => {
    setProviderName(provider);
  };

  const handleBankAccountSelect = (bankAccount: string) => {
    setInstitutionID(bankAccount);
  };

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      const { listen } = await import("@tauri-apps/api/event");
      unlisten = await listen<string>("gocardless-redirect", async (event) => {
        const req_id = event.payload;

        setIsConnecting(true);
        try {
          await invoke("connect_bank_account_phase_2", {
            providerTitle: providerName,
            institutionId: institutionID,
            requisitionId: req_id,
          });
          setRefreshTrigger((prev) => prev + 1);
          setRequisitionID("");
        } catch (e) {
          alert(
            "Failed to complete bank connection. Please check console for details."
          );
        } finally {
          setIsConnecting(false);
        }
      });
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, [providerName, institutionID]);

  const handleConnectBank = async () => {
    setIsConnecting(true);
    try {
      const rustBankConnectionInfo = await invoke(
        "connect_bank_account_phase_1",
        {
          providerTitle: providerName,
          institutionId: institutionID,
        }
      );
      const bankConnectionInfo = rustBankConnectionInfo as BankConnectionInfo;
      await open(bankConnectionInfo.link);
      setRequisitionID(bankConnectionInfo.id);
    } catch (e) {
      setIsConnecting(false);
    }
  };

  const handleAddLocalCSVAccount = async () => {
    try {
      await invoke("add_local_csv_account", {
        bankName,
        iban,
      });
      setBankName("");
      setIban("");
      setRefreshTrigger((prev) => prev + 1);
    } catch (e) {
      // Error handling done by Tauri
    }
  };

  const handleAddLocalMTAAccount = async () => {
    try {
      await invoke("add_local_mta_account", {
        bankName,
        iban,
      });
      setBankName("");
      setIban("");
      setRefreshTrigger((prev) => prev + 1);
    } catch (e) {
      // Error handling done by Tauri
    }
  };

  const openModal = () => setModalOpen(true);
  const closeModal = () => setModalOpen(false);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="font-bold text-3xl tracking-tight">Settings</h1>
      </div>

      <Tabs defaultValue="providers">
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger value="providers">Providers</TabsTrigger>
          <TabsTrigger value="bank-accounts">Bank Accounts</TabsTrigger>
        </TabsList>

        <TabsContent className="space-y-4" value="providers">
          <BankAccountDataProviderList />
          <Button onClick={openModal}>Add New Provider</Button>
          <BankAccountDataProviderAddModal
            closeModal={closeModal}
            isOpen={isModalOpen}
          />
        </TabsContent>

        <TabsContent className="space-y-6" value="bank-accounts">
          <BankAccountsList refreshTrigger={refreshTrigger} />

          <Card>
            <CardHeader>
              <CardTitle>Add New Account</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <label className="font-medium text-sm">Provider</label>
                <BankAccountDataProviderSelect
                  onProviderSelect={handleProviderSelect}
                />
              </div>

              {providerName === "LocalCSV" ? (
                <>
                  <div className="space-y-2">
                    <label className="font-medium text-sm">Bank Name</label>
                    <Input
                      onChange={(e) => setBankName(e.target.value)}
                      placeholder="e.g. MLP"
                      value={bankName}
                    />
                  </div>
                  <div className="space-y-2">
                    <label className="font-medium text-sm">IBAN</label>
                    <Input
                      onChange={(e) => setIban(e.target.value)}
                      placeholder="e.g. DE12 3456..."
                      value={iban}
                    />
                  </div>
                  <Button className="w-full" onClick={handleAddLocalCSVAccount}>
                    <Plus className="mr-2 size-4" />
                    Add Local Account
                  </Button>
                </>
              ) : providerName === "LocalMTA" ? (
                <>
                  <div className="space-y-2">
                    <label className="font-medium text-sm">Bank Name</label>
                    <Input
                      onChange={(e) => setBankName(e.target.value)}
                      placeholder="e.g. MLP"
                      value={bankName}
                    />
                  </div>
                  <div className="space-y-2">
                    <label className="font-medium text-sm">IBAN</label>
                    <Input
                      onChange={(e) => setIban(e.target.value)}
                      placeholder="e.g. DE12 3456..."
                      value={iban}
                    />
                  </div>
                  <Button className="w-full" onClick={handleAddLocalMTAAccount}>
                    <Plus className="mr-2 size-4" />
                    Add Local Account
                  </Button>
                </>
              ) : (
                <>
                  <div className="space-y-2">
                    <label className="font-medium text-sm">Bank</label>
                    <BankAccountsSelect
                      country="de"
                      onBankAccountSelect={handleBankAccountSelect}
                      provider={providerName}
                    />
                  </div>
                  <Button
                    className="w-full"
                    disabled={!institutionID || isConnecting}
                    onClick={handleConnectBank}
                  >
                    <Plus className="mr-2 size-4" />
                    {isConnecting ? "Connecting..." : "Connect Bank"}
                  </Button>
                </>
              )}
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}
