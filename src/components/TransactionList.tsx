// TransactionListComponent.tsx
import React, { useState, useEffect } from "react";
import TransactionComponent from "./Transaction";
import { invoke } from "@tauri-apps/api/tauri";

import { Transaction } from "../models/typeshare_definitions";

const TransactionListComponent: React.FC = () => {
  const [transactions, setTransactions] = useState<Transaction[]>([]);

  useEffect(() => {
    const fetchProviders = async () => {
      try {
        // Replace 'get_providers' with your actual Tauri command
        invoke("get_transactions_handler").then((rustTransactions: any) => {
          let transactions = rustTransactions as Transaction[];
          setTransactions(transactions);
        });
      } catch (error) {
        console.error("Failed to fetch accounts:", error);
      }
    };

    fetchProviders();
  }, []);

  return (
    <div>
      <h2>Transactions:</h2>
      <ul>
        {transactions.map((transaction, index) => (
          <TransactionComponent key={index} transaction={transaction} />
        ))}
      </ul>
    </div>
  );
};

export default TransactionListComponent;
