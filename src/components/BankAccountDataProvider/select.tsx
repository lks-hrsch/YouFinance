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

type BankingProviderSelectProps = {
  onProviderSelect: (provider: string) => void;
};

const BankAccountDataProviderSelect: React.FC<BankingProviderSelectProps> = ({
  onProviderSelect,
}) => {
  const [providers, setProviders] = useState<string[]>([]);

  useEffect(() => {
    const fetchProviders = () => {
      invoke("list_possible_banking_providers")
        .then((rustBankingProvider: unknown) => {
          setProviders(rustBankingProvider as string[]);
        })
        .catch((error) => {
          console.error("Failed to fetch banking providers:", error);
        });
    };

    fetchProviders();
  }, []);

  return (
    <div className="w-full max-w-sm">
      <Select onValueChange={onProviderSelect}>
        <SelectTrigger>
          <SelectValue placeholder="Select a provider" />
        </SelectTrigger>
        <SelectContent>
          {providers.map((provider) => (
            <SelectItem key={provider} value={provider}>
              {provider}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
};

export default BankAccountDataProviderSelect;
