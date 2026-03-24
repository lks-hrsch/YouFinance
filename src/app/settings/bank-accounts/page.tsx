"use client";

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import { Plus } from "lucide-react";
import { useState } from "react";
import BankAccountDataProviderSelect from "@/components/BankAccountDataProvider/select";
import BankAccountsList from "@/components/BankAccounts/list";
import BankAccountsSelect from "@/components/BankAccounts/select";
import { Button } from "@/components/ui/button";
import type { BankConnectionInfo } from "../../../models/typeshare_definitions";

export default function BankAccountsPage() {
  const [providerName, setProviderName] = useState<string>("");
  const [institutionID, setInstitutionID] = useState<string>("");
  const [requisitionID, setRequisitionID] = useState<string>("");

  const handleProviderSelect = (provider: string) => {
    setProviderName(provider);
  };

  const handleBankAccountSelect = (bankAccount: string) => {
    setInstitutionID(bankAccount);
  };

  const handleConnectBank = async () => {
    const rustBankConnectionInfo = await invoke(
      "connect_bank_account_phase_1",
      {
        providerTitle: providerName,
        institutionId: institutionID,
      }
    );
    const bankConnectionInfo = rustBankConnectionInfo as BankConnectionInfo;
    console.log(bankConnectionInfo.link);
    await open(bankConnectionInfo.link);
    setRequisitionID(bankConnectionInfo.id);
  };

  const _handleAddAccounts = async () => {
    await invoke("connect_bank_account_phase_2", {
      providerTitle: providerName,
      institutionId: institutionID,
      requisitionId: requisitionID,
    });
  };

  return (
    <div className="space-y-4">
      <BankAccountsList />
      <BankAccountDataProviderSelect onProviderSelect={handleProviderSelect} />
      <BankAccountsSelect
        country="de"
        onBankAccountSelect={handleBankAccountSelect}
        provider={providerName}
      />
      <div className="flex items-center justify-between">
        <h1 className="font-bold text-2xl">Bank Accounts</h1>
        <Button onClick={handleConnectBank}>
          <Plus className="mr-2 size-4" />
          Connect Bank
        </Button>
      </div>
    </div>
  );
}
