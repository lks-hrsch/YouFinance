import { Card, Typography } from "@material-tailwind/react";
import { invoke } from "@tauri-apps/api/core";
import type React from "react";
import { useEffect, useState } from "react";
import type { Provider } from "../../models/typeshare_definitions";

const TABLE_HEAD = ["ID", "Provider", "Secret ID", "Secret Key"];

const BankAccountDataProviderList: React.FC = () => {
  const [providers, setProviders] = useState<Provider[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);

  useEffect(() => {
    const fetchProviders = async () => {
      try {
        // Replace 'get_providers' with your actual Tauri command
        invoke("get_banking_providers").then(
          (rustBankingProviders: unknown) => {
            const bankingProviders = rustBankingProviders as Provider[];
            setProviders(bankingProviders);
          }
        );
      } catch (error) {
        console.error("Failed to fetch providers:", error);
        // Handle error (e.g., show an error message to the user)
      } finally {
        setIsLoading(false);
      }
    };

    fetchProviders();
  }, []);

  if (isLoading) return <div>Loading...</div>;

  return (
    <div>
      <Card
        className="h-full w-full overflow-scroll"
        onPointerEnterCapture={undefined}
        onPointerLeaveCapture={undefined}
        placeholder={undefined}
      >
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
            {providers.map((provider, index) => {
              const isLast = index === providers.length - 1;
              const classes = isLast
                ? "p-4"
                : "p-4 border-b border-blue-gray-50";

              return (
                <tr key={provider.id}>
                  <td className={classes}>
                    <Typography
                      className="font-normal"
                      color="blue-gray"
                      onPointerEnterCapture={undefined}
                      onPointerLeaveCapture={undefined}
                      placeholder={undefined}
                      variant="small"
                    >
                      {provider.id}
                    </Typography>
                  </td>
                  <td className={classes}>
                    <Typography
                      className="font-normal"
                      color="blue-gray"
                      onPointerEnterCapture={undefined}
                      onPointerLeaveCapture={undefined}
                      placeholder={undefined}
                      variant="small"
                    >
                      {provider.title}
                    </Typography>
                  </td>
                  <td className={classes}>
                    <Typography
                      className="font-normal"
                      color="blue-gray"
                      onPointerEnterCapture={undefined}
                      onPointerLeaveCapture={undefined}
                      placeholder={undefined}
                      variant="small"
                    >
                      {provider.secret_id}
                    </Typography>
                  </td>
                  <td className={classes}>
                    <Typography
                      className="font-normal"
                      color="blue-gray"
                      onPointerEnterCapture={undefined}
                      onPointerLeaveCapture={undefined}
                      placeholder={undefined}
                      variant="small"
                    >
                      {provider.secret_key}
                    </Typography>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </Card>
    </div>
  );
};

export default BankAccountDataProviderList;
