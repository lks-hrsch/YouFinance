import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Select, Option } from "@material-tailwind/react";

interface BankingProviderSelectProps {
  onProviderSelect: (provider: string) => void;
}

const BankAccountDataProviderSelect: React.FC<BankingProviderSelectProps> = ({
  onProviderSelect,
}) => {
  const [providers, setProviders] = useState<string[]>([]);

  useEffect(() => {
    // Function to fetch banking providers from the Tauri backend
    const fetchProviders = async () => {
      try {
        // Replace 'invoke' with the actual Tauri function call you use to fetch providers
        invoke("list_possible_banking_providers").then(
          (rustBankingProvider: any) => {
            setProviders(rustBankingProvider);
          },
        );
      } catch (error) {
        console.error("Failed to fetch banking providers:", error);
      }
    };

    fetchProviders();
  }, []); // Empty dependency array means this effect runs once on mount

  const handleChange = (value: string | undefined) => {
    if (value !== undefined) {
      onProviderSelect(value);
    } else {
      // Handle the undefined case or set a default value
      console.warn("Selected value is undefined.");
      // For example, you might want to call `onProviderSelect` with a default value or perform some other action
    }
  };

  return (
    <>
      <Select
        id="provider-select"
        label="Select a provider"
        onChange={handleChange}
        placeholder={undefined}
      >
        {providers.map((provider, index) => (
          <Option key={index} value={provider}>
            {provider}
          </Option>
        ))}
      </Select>
    </>
  );
};

export default BankAccountDataProviderSelect;
