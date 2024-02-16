// TransactionListComponent.tsx
import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import { Transaction } from "../models/typeshare_definitions";

import { List, ListItem, Typography, Card } from "@material-tailwind/react";

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
    <>
      <div className="flex justify-between">
        <Typography className="w-1/4" variant="h4" placeholder={undefined}>
          Booking Date
        </Typography>
        <Typography className="w-1/4" variant="h4" placeholder={undefined}>
          Debtor Name
        </Typography>
        <Typography className="w-1/4" variant="h4" placeholder={undefined}>
          Creditor Name
        </Typography>
        <Typography className="w-1/4" variant="h4" placeholder={undefined}>
          Ammount
        </Typography>
      </div>

      <List placeholder={undefined}>
        {/* Divide the list in 4 tiles */}

        {transactions.map((transaction, index) => (
          <ListItem placeholder={undefined}>
            <div className="w-1/4">{transaction.date}</div>
            <div className="flex-row justify-center w-1/4">
              <div>{transaction.debitor_name}</div>
              <div>{transaction.debitor_iban}</div>
            </div>
            <div className="flex-row justify-center w-1/4">
              <div>{transaction.creditor_name}</div>
              <div>{transaction.creditor_iban}</div>
            </div>
            <div className="flex justify-end gap-2 w-1/4">
              {transaction.ammount < 0 ? (
                <div className="text-red-500">{transaction.ammount}</div>
              ) : (
                <div className="text-green-500">+{transaction.ammount}</div>
              )}
              <div>{transaction.currency}</div>
            </div>
          </ListItem>
        ))}
      </List>
    </>
  );
};

export default TransactionListComponent;
