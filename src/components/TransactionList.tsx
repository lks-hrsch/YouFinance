// TransactionListComponent.tsx

import { List, ListItem, Typography } from "@material-tailwind/react";
import { invoke } from "@tauri-apps/api/core";
import type React from "react";
import { useEffect, useState } from "react";
import type { Transaction } from "../models/typeshare_definitions";

const TransactionListComponent: React.FC = () => {
  const [transactions, setTransactions] = useState<Transaction[]>([]);

  useEffect(() => {
    const fetchProviders = () => {
      try {
        invoke("get_transactions").then((rustTransactions: unknown) => {
          const fetchedTransactions = rustTransactions as Transaction[];
          setTransactions(fetchedTransactions);
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
        <Typography
          className="w-1/4"
          onPointerEnterCapture={undefined}
          onPointerLeaveCapture={undefined}
          placeholder={undefined}
          variant="h4"
        >
          Booking Date
        </Typography>
        <Typography
          className="w-1/4"
          onPointerEnterCapture={undefined}
          onPointerLeaveCapture={undefined}
          placeholder={undefined}
          variant="h4"
        >
          Debtor Name
        </Typography>
        <Typography
          className="w-1/4"
          onPointerEnterCapture={undefined}
          onPointerLeaveCapture={undefined}
          placeholder={undefined}
          variant="h4"
        >
          Creditor Name
        </Typography>
        <Typography
          className="w-1/4"
          onPointerEnterCapture={undefined}
          onPointerLeaveCapture={undefined}
          placeholder={undefined}
          variant="h4"
        >
          Ammount
        </Typography>
      </div>

      <List
        onPointerEnterCapture={undefined}
        onPointerLeaveCapture={undefined}
        placeholder={undefined}
      >
        {/* Divide the list in 4 tiles */}

        {transactions.map((transaction) => (
          <ListItem
            key={transaction.date}
            onPointerEnterCapture={undefined}
            onPointerLeaveCapture={undefined}
            onResize={undefined}
            onResizeCapture={undefined}
            placeholder={undefined}
          >
            <div className="w-1/4">{transaction.date}</div>
            <div className="w-1/4 flex-row justify-center">
              <div>{transaction.debitor_name}</div>
              <div>{transaction.debitor_iban}</div>
            </div>
            <div className="w-1/4 flex-row justify-center">
              <div>{transaction.creditor_name}</div>
              <div>{transaction.creditor_iban}</div>
            </div>
            <div className="flex w-1/4 justify-end gap-2">
              {transaction.amount < 0 ? (
                <div className="text-red-500">{transaction.amount}</div>
              ) : (
                <div className="text-green-500">+{transaction.amount}</div>
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
