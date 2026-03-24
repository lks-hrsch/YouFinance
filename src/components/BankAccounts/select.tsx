"use client";

import { invoke } from "@tauri-apps/api/core";
import type React from "react";
import { useEffect, useState } from "react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import type { BankInfo } from "../../models/typeshare_definitions";

interface BankAccountsSelectProps {
  country: string;
  onBankAccountSelect: (bankAccount: string) => void;
  provider: string;
}

const BankAccountsSelect: React.FC<BankAccountsSelectProps> = ({
  provider,
  country,
  onBankAccountSelect,
}) => {
  const [banks, setBanks] = useState<BankInfo[]>([]);

  useEffect(() => {
    const fetchProviders = () => {
      invoke("get_banks_by_country_handler", {
        providerTitle: provider,
        country,
      })
        .then((rustBanks: unknown) => {
          const fetchedBanks = rustBanks as BankInfo[];
          setBanks(fetchedBanks);
        })
        .catch((error) => {
          console.error("Failed to fetch bank accounts:", error);
        });
    };

    if (provider && country) {
      fetchProviders();
    }
  }, [provider, country]);

  return (
    <div className="w-full max-w-sm">
      <Select onValueChange={onBankAccountSelect}>
        <SelectTrigger>
          <SelectValue placeholder="Select a bank" />
        </SelectTrigger>
        <SelectContent>
          {banks.map((bank) => (
            <SelectItem key={bank.id} value={bank.id}>
              {bank.name}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
};

export default BankAccountsSelect;
