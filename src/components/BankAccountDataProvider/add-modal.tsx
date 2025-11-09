import { invoke } from "@tauri-apps/api/core";
import type React from "react";
import { type FormEvent, useState } from "react";
import BankAccountDataProviderSelect from "./select";

type BankAccountDataProviderAddModalProps = {
  isOpen: boolean;
  closeModal: () => void;
};

const BankAccountDataProviderAddModal: React.FC<
  BankAccountDataProviderAddModalProps
> = ({ isOpen, closeModal }) => {
  const [name, setName] = useState<string>("");
  const [sid, setSid] = useState<string>("");
  const [skey, setSkey] = useState<string>("");

  // const handleInputChange = (
  //   event: ChangeEvent<HTMLInputElement>,
  //   setState: React.Dispatch<React.SetStateAction<string>>,
  // ) => {
  //   setState(event.target.value);
  // };

  const handleProviderSelect = (provider: string) => {
    setName(provider);
  };

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    try {
      await invoke("add_banking_provider", {
        name,
        sid,
        skey,
      });
    } catch (error) {
      console.error("Failed to add provider:", error);
      // Handle the error (e.g., show an error message)
    }
  };

  if (!isOpen) {
    return null;
  }

  return (
    <div className="fixed inset-0 z-50 flex overflow-auto bg-smoke-light">
      <div className="relative m-auto flex w-full max-w-md flex-col rounded-lg bg-white p-8">
        <button
          className="absolute top-0 right-0 p-4"
          onClick={closeModal}
          type="button"
        >
          [Close]
        </button>
        <h2>Add New Provider</h2>
        <form className="mt-4" onSubmit={handleSubmit}>
          <div>
            <label htmlFor="provider-name">Name</label>
            <BankAccountDataProviderSelect
              onProviderSelect={handleProviderSelect}
            />
          </div>
          <div className="mt-4">
            <label htmlFor="secret-id">Secret ID (Optional)</label>
            <input
              className="mt-1 w-full rounded border border-gray-300 p-2"
              id="secret-id"
              onChange={(e) => setSid(e.target.value)}
              type="text"
              value={sid}
            />
          </div>
          <div className="mt-4">
            <label htmlFor="secret-key">Secret Key (Optional)</label>
            <input
              className="mt-1 w-full rounded border border-gray-300 p-2"
              id="secret-key"
              onChange={(e) => setSkey(e.target.value)}
              type="text"
              value={skey}
            />
          </div>
          <button
            className="mt-4 rounded bg-blue-500 p-2 text-white"
            type="submit"
          >
            Add Provider
          </button>
        </form>
      </div>
    </div>
  );
};

export default BankAccountDataProviderAddModal;
