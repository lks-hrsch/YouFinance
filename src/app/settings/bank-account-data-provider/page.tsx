"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import BankAccountDataProviderAddModal from "@/components/BankAccountDataProvider/add-modal";
import BankAccountDataProviderList from "@/components/BankAccountDataProvider/list";

export default function BankAccountDataProviderPage() {
  const [isModalOpen, setModalOpen] = useState(false);

  const openModal = () => setModalOpen(true);
  const closeModal = () => setModalOpen(false);

  return (
    <div className="space-y-4">
      <BankAccountDataProviderList />
      <Button onClick={openModal}>Add New Provider</Button>
      <BankAccountDataProviderAddModal
        closeModal={closeModal}
        isOpen={isModalOpen}
      />
    </div>
  );
}
