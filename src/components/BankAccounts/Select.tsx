import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Select, Option } from "@material-tailwind/react";
import { BankInfo } from "../../models/typeshare_definitions";

interface BankAccountsSelectProps {
  provider: string;
  country: string;
  onBankAccountSelect: (bankAccount: string) => void;
}

const BankAccountsSelect: React.FC<BankAccountsSelectProps> = ({
  provider,
  country,
  onBankAccountSelect,
}) => {
  const [banks, setBanks] = useState<BankInfo[]>([]);

  useEffect(() => {
    // Function to fetch banking providers from the Tauri backend
    const fetchProviders = async () => {
      try {
        // Replace 'invoke' with the actual Tauri function call you use to fetch providers
        invoke("get_banks_by_country_handler", {
          providerTitle: provider,
          country: country,
        }).then((rustBanks: unknown) => {
          const banks = rustBanks as BankInfo[];
          setBanks(banks);
        });
      } catch (error) {
        console.error("Failed to fetch bank accounts:", error);
      }
    };

    if (provider && country) {
      fetchProviders();
    }
  }, [provider, country]);

  const handleChange = (value: string | undefined) => {
    if (value !== undefined) {
      onBankAccountSelect(value);
    } else {
      // Handle the undefined case or set a default value
      console.warn("Selected value is undefined.");
      // For example, you might want to call `onProviderSelect` with a default value or perform some other action
    }
  };

  return (
    <>
      <Select
        id="bank-select"
        label="Select a bank"
        onChange={handleChange}
        placeholder={undefined}
      >
        {banks.map((provider, index) => (
          <Option key={index} value={provider.id}>
            {provider.name}
          </Option>
        ))}
      </Select>
    </>
  );
};

export default BankAccountsSelect;
