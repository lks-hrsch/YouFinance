import React, { useState } from "react";

import BankAccountDataProviderSelect from "../../components/BankAccountDataProvider/Select";
import BankAccountsSelect from "../../components/BankAccounts/Select";
import BankAccountsList from "../../components/BankAccounts/List";
import { BankConnectionInfo } from "../../models/typeshare_definitions";

import { Button } from "@material-tailwind/react";
import { PlusIcon } from "@heroicons/react/24/outline";
import { invoke } from "@tauri-apps/api/tauri";

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
    }).then((rustBankConnectionInfo: unknown) => {
      const bankConnectionInfo = rustBankConnectionInfo as BankConnectionInfo;
      console.log(bankConnectionInfo.link);
      window.open(bankConnectionInfo.link, "_blank");
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
        provider={providerName}
        country="de"
        onBankAccountSelect={handleBankAccountSelect}
      />
      <Button
        className="flex items-center gap-3"
        onClick={handleConnectBank}
        placeholder={undefined}
      >
        <PlusIcon />
        Connect bank
      </Button>
      <Button
        className="flex items-center gap-3"
        onClick={handleAddAccounts}
        placeholder={undefined}
      >
        <PlusIcon />
        Add accounts
      </Button>
    </>
  );
};

export default BankAccounts;
