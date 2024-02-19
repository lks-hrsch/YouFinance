import React, { useState, ChangeEvent, FormEvent } from "react";
import BankAccountDataProviderSelect from "./Select";
import { invoke } from "@tauri-apps/api/tauri";

interface BankAccountDataProviderAddModalProps {
  isOpen: boolean;
  closeModal: () => void;
}

const BankAccountDataProviderAddModal: React.FC<
  BankAccountDataProviderAddModalProps
> = ({ isOpen, closeModal }) => {
  const [name, setName] = useState<string>("");
  const [sid, setSid] = useState<string>("");
  const [skey, setSkey] = useState<string>("");

  const handleInputChange = (
    event: ChangeEvent<HTMLInputElement>,
    setState: React.Dispatch<React.SetStateAction<string>>,
  ) => {
    setState(event.target.value);
  };

  const handleProviderSelect = (provider: string) => {
    setName(provider);
  };

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    try {
      await invoke("add_banking_provider", {
        name: name,
        sid: sid,
        skey: skey,
      });
    } catch (error) {
      console.error("Failed to add provider:", error);
      // Handle the error (e.g., show an error message)
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 overflow-auto bg-smoke-light flex">
      <div className="relative p-8 bg-white w-full max-w-md m-auto flex-col flex rounded-lg">
        <span className="absolute top-0 right-0 p-4" onClick={closeModal}>
          <button>[Close]</button>
        </span>
        <h2>Add New Provider</h2>
        <form onSubmit={handleSubmit} className="mt-4">
          <div>
            <label>Name</label>
            <BankAccountDataProviderSelect
              onProviderSelect={handleProviderSelect}
            />
          </div>
          <div className="mt-4">
            <label>Secret ID (Optional)</label>
            <input
              type="text"
              value={sid}
              onChange={(e) => setSid(e.target.value)}
              className="w-full p-2 border border-gray-300 rounded mt-1"
            />
          </div>
          <div className="mt-4">
            <label>Secret Key (Optional)</label>
            <input
              type="text"
              value={skey}
              onChange={(e) => setSkey(e.target.value)}
              className="w-full p-2 border border-gray-300 rounded mt-1"
            />
          </div>
          <button
            type="submit"
            className="mt-4 bg-blue-500 text-white p-2 rounded"
          >
            Add Provider
          </button>
        </form>
      </div>
    </div>
  );
};

export default BankAccountDataProviderAddModal;
