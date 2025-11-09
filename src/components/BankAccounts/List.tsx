import { TrashIcon } from "@heroicons/react/24/outline";
import { Button, Card, Typography } from "@material-tailwind/react";
import { invoke } from "@tauri-apps/api/core";
import type React from "react";
import { useEffect, useState } from "react";
import type { Account } from "../../models/typeshare_definitions";

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
            {} as Record<string, Account[]>
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
      bankConnectionId,
    });
  };

  return (
    <>
      {Object.entries(groupedAccounts).map(([bankConnectionId, accounts]) => (
        <Card
          className="mb-4 h-full w-full overflow-scroll"
          key={bankConnectionId}
          onPointerEnterCapture={undefined}
          onPointerLeaveCapture={undefined}
          placeholder={undefined}
        >
          <div className="flex items-center justify-between p-1">
            <Typography
              color="blue-gray"
              onPointerEnterCapture={undefined}
              onPointerLeaveCapture={undefined}
              placeholder={undefined}
              variant="h6"
            >
              Bank Connection ID: {bankConnectionId}
            </Typography>
            <Button
              color="red"
              onClick={() => handleDelete(bankConnectionId)}
              onPointerEnterCapture={undefined}
              onPointerLeaveCapture={undefined}
              placeholder={undefined}
            >
              <TrashIcon className="h-4 w-4" />
            </Button>
          </div>
          <table className="w-full min-w-max table-auto text-left">
            <thead>
              <tr>
                {TABLE_HEAD.map((head) => (
                  <th
                    className="border-blue-gray-100 border-b bg-blue-gray-50 p-4"
                    key={head}
                  >
                    <Typography
                      className="font-normal leading-none opacity-70"
                      color="blue-gray"
                      onPointerEnterCapture={undefined}
                      onPointerLeaveCapture={undefined}
                      placeholder={undefined}
                      variant="small"
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
