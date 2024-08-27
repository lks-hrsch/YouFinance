import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

import { Card, Typography, Button } from "@material-tailwind/react";
import { Account } from "../../models/typeshare_definitions";

import { TrashIcon } from "@heroicons/react/24/outline";

const TABLE_HEAD = ["ID", "Title", "Institution ID", "Account ID", "IBAN"];

const BankAccountsList: React.FC = () => {
  const [groupedAccounts, setGroupedAccounts] = useState<
    Record<string, Account[]>
  >({});

  useEffect(() => {
    const fetchProviders = async () => {
      try {
        invoke("get_banking_accounts").then((rustBankAccounts: unknown) => {
          const bankAccounts = rustBankAccounts as Account[];
          const grouped = bankAccounts.reduce(
            (acc, account) => {
              (acc[account.bank_connection_id] =
                acc[account.bank_connection_id] || []).push(account);
              return acc;
            },
            {} as Record<string, Account[]>,
          );
          setGroupedAccounts(grouped);
        });
      } catch (error) {
        console.error("Failed to fetch accounts:", error);
      }
    };

    fetchProviders();
  }, []);

  const handleDelete = async (bankConnectionId: string) => {
    // Logic to delete bank connection by bankConnectionId
    console.log(`Delete bank connection with ID: ${bankConnectionId}`);
    // You may want to call an API or a local function to delete the bank connection
    // After deletion, update your state to reflect the changes
    await invoke("disconnect_bank_account", {
      providerTitle: "GoCardless",
      bankConnectionId: bankConnectionId,
    });
  };

  return (
    <>
      {Object.entries(groupedAccounts).map(([bankConnectionId, accounts]) => (
        <Card
          key={bankConnectionId}
          className="h-full w-full overflow-scroll mb-4"
          placeholder={undefined}
          onPointerEnterCapture={undefined}
          onPointerLeaveCapture={undefined}
        >
          <div className="flex justify-between items-center p-1">
            <Typography
              variant="h6"
              color="blue-gray"
              placeholder={undefined}
              onPointerEnterCapture={undefined}
              onPointerLeaveCapture={undefined}
            >
              Bank Connection ID: {bankConnectionId}
            </Typography>
            <Button
              color="red"
              onClick={() => handleDelete(bankConnectionId)}
              placeholder={undefined}
              onPointerEnterCapture={undefined}
              onPointerLeaveCapture={undefined}
            >
              <TrashIcon className="w-4 h-4" />
            </Button>
          </div>
          <table className="w-full min-w-max table-auto text-left">
            <thead>
              <tr>
                {TABLE_HEAD.map((head) => (
                  <th
                    key={head}
                    className="border-b border-blue-gray-100 bg-blue-gray-50 p-4"
                  >
                    <Typography
                      variant="small"
                      color="blue-gray"
                      className="font-normal leading-none opacity-70"
                      placeholder={undefined}
                      onPointerEnterCapture={undefined}
                      onPointerLeaveCapture={undefined}
                    >
                      {head}
                    </Typography>
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {accounts.map((account, index) => {
                const isLast = index === accounts.length - 1;
                const classes = isLast
                  ? "p-4"
                  : "p-4 border-b border-blue-gray-50";

                return (
                  <tr key={account.id}>
                    <td className={classes}>{account.id}</td>
                    <td className={classes}>{account.title}</td>
                    <td className={classes}>{account.institution_id}</td>
                    <td className={classes}>{account.account_id}</td>
                    <td className={classes}>{account.iban}</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </Card>
      ))}
    </>
  );
};

export default BankAccountsList;
