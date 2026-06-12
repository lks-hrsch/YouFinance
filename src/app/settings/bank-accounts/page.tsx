"use client";

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import { Plus } from "lucide-react";
import { useEffect, useState } from "react";
import BankAccountDataProviderSelect from "@/components/BankAccountDataProvider/select";
import BankAccountsList from "@/components/BankAccounts/list";
import BankAccountsSelect from "@/components/BankAccounts/select";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import type { BankConnectionInfo } from "../../../models/typeshare_definitions";

export default function BankAccountsPage() {
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
        console.log(
          "Received GoCardless redirect event with ref:",
          event.payload
        );
        const req_id = event.payload;

        setIsConnecting(true);
        try {
          await invoke("connect_bank_account_phase_2", {
            providerTitle: providerName,
            institutionId: institutionID,
            requisitionId: req_id,
          });
          console.log("Phase 2 complete. Bank account connected successfully.");
          setRefreshTrigger((prev) => prev + 1);
          setRequisitionID("");
        } catch (e) {
          console.error("Failed to complete bank connection phase 2:", e);
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
      console.log("Opening connection link:", bankConnectionInfo.link);
      await open(bankConnectionInfo.link);
      setRequisitionID(bankConnectionInfo.id);
    } catch (e) {
      console.error("Failed to start bank connection phase 1:", e);
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
      console.error("Failed to add local account:", e);
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
      console.error("Failed to add local account:", e);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="font-bold text-2xl tracking-tight">Bank Accounts</h2>
      </div>
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
    </div>
  );
}
