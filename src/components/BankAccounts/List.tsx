import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import { Card, Typography } from "@material-tailwind/react";
import { Account } from "../../models/typeshare_definitions";

const TABLE_HEAD = [
  "ID",
  "Title",
  "Institution ID",
  "Bank Connection ID",
  "Account ID",
  "IBAN",
];

const BankAccountsList: React.FC = () => {
  const [accounts, setAccounts] = useState<Account[]>([]);

  useEffect(() => {
    const fetchProviders = async () => {
      try {
        // Replace 'get_providers' with your actual Tauri command
        invoke("get_banking_accounts").then((rustBankAccounts: unknown) => {
          const bankAccounts = rustBankAccounts as Account[];
          setAccounts(bankAccounts);
        });
      } catch (error) {
        console.error("Failed to fetch accounts:", error);
      }
    };

    fetchProviders();
  }, []);

  return (
    <>
      <Card className="h-full w-full overflow-scroll" placeholder={undefined}>
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
                  <td className={classes}>
                    <Typography
                      variant="small"
                      color="blue-gray"
                      className="font-normal"
                      placeholder={undefined}
                    >
                      {account.id}
                    </Typography>
                  </td>

                  <td className={classes}>
                    <Typography
                      variant="small"
                      color="blue-gray"
                      className="font-normal"
                      placeholder={undefined}
                    >
                      {account.title}
                    </Typography>
                  </td>

                  <td className={classes}>
                    <Typography
                      variant="small"
                      color="blue-gray"
                      className="font-normal"
                      placeholder={undefined}
                    >
                      {account.institution_id}
                    </Typography>
                  </td>

                  <td className={classes}>
                    <Typography
                      variant="small"
                      color="blue-gray"
                      className="font-normal"
                      placeholder={undefined}
                    >
                      {account.bank_connection_id}
                    </Typography>
                  </td>

                  <td className={classes}>
                    <Typography
                      variant="small"
                      color="blue-gray"
                      className="font-normal"
                      placeholder={undefined}
                    >
                      {account.account_id}
                    </Typography>
                  </td>

                  <td className={classes}>
                    <Typography
                      variant="small"
                      color="blue-gray"
                      className="font-normal"
                      placeholder={undefined}
                    >
                      {account.iban}
                    </Typography>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </Card>
    </>
  );
};

export default BankAccountsList;
