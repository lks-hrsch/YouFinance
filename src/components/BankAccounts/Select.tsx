import { Option, Select } from "@material-tailwind/react";
import { invoke } from "@tauri-apps/api/core";
import type React from "react";
import { useEffect, useState } from "react";
import type { BankInfo } from "../../models/typeshare_definitions";

type BankAccountsSelectProps = {
  provider: string;
  country: string;
  onBankAccountSelect: (bankAccount: string) => void;
};

const BankAccountsSelect: React.FC<BankAccountsSelectProps> = ({
  provider,
  country,
  onBankAccountSelect,
}) => {
  const [banks, setBanks] = useState<BankInfo[]>([]);

  useEffect(() => {
    // Function to fetch banking providers from the Tauri backend
    const fetchProviders = () => {
      try {
        // Replace 'invoke' with the actual Tauri function call you use to fetch providers
        invoke("get_banks_by_country_handler", {
          providerTitle: provider,
          country,
        }).then((rustBanks: unknown) => {
          const fetchedBanks = rustBanks as BankInfo[];
          setBanks(fetchedBanks);
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
    <Select
        id="bank-select"
        label="Select a bank"
        onChange={handleChange}
        onPointerEnterCapture={undefined}
        onPointerLeaveCapture={undefined}
        placeholder={undefined}
      >
        {banks.map((bank) => (
          <Option key={bank.id} value={bank.id}>
            {bank.name}
          </Option>
        ))}
      </Select>
  );
};

export default BankAccountsSelect;
