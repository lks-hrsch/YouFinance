import { PlusIcon } from "@heroicons/react/24/outline";
import { Button } from "@material-tailwind/react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import type React from "react";
import { useState } from "react";
import BankAccountDataProviderSelect from "../../components/BankAccountDataProvider/Select";
import BankAccountsList from "../../components/BankAccounts/List";
import BankAccountsSelect from "../../components/BankAccounts/Select";
import type { BankConnectionInfo } from "../../models/typeshare_definitions";

const BankAccounts: React.FC = () => {
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
    invoke("connect_bank_account_phase_1", {
      providerTitle: providerName,
      institutionId: institutionID,
    }).then(async (rustBankConnectionInfo: unknown) => {
      const bankConnectionInfo = rustBankConnectionInfo as BankConnectionInfo;
      console.log(bankConnectionInfo.link);
      await open(bankConnectionInfo.link);
      setRequisitionID(bankConnectionInfo.id);
    });
  };

  const handleAddAccounts = async () => {
    await invoke("connect_bank_account_phase_2", {
      providerTitle: providerName,
      institutionId: institutionID,
      requisitionId: requisitionID,
    });
  };

  return (
    <>
      <BankAccountsList />
      <BankAccountDataProviderSelect onProviderSelect={handleProviderSelect} />
      <BankAccountsSelect
        country="de"
        onBankAccountSelect={handleBankAccountSelect}
        provider={providerName}
      />
      <Button
        className="flex items-center gap-3"
        onClick={handleConnectBank}
        onPointerEnterCapture={undefined}
        onPointerLeaveCapture={undefined}
        placeholder={undefined}
      >
        <PlusIcon />
        Connect bank
      </Button>
      <Button
        className="flex items-center gap-3"
        onClick={handleAddAccounts}
        onPointerEnterCapture={undefined}
        onPointerLeaveCapture={undefined}
        placeholder={undefined}
      >
        <PlusIcon />
        Add accounts
      </Button>
    </>
  );
};

export default BankAccounts;
