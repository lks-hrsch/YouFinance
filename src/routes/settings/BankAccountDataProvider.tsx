import { Button } from "@material-tailwind/react";
import type React from "react";
import { useState } from "react";
import BankAccountDataProviderAddModal from "../../components/BankAccountDataProvider/AddModal";
import BankAccountDataProviderList from "../../components/BankAccountDataProvider/List";

const BankAccountDataProvider: React.FC = () => {
  const [isModalOpen, setModalOpen] = useState(false);

  const openModal = () => setModalOpen(true);
  const closeModal = () => setModalOpen(false);

  return (
    <>
      <BankAccountDataProviderList />
      <Button
        onClick={openModal}
        onPointerEnterCapture={undefined}
        onPointerLeaveCapture={undefined}
        placeholder={undefined}
      >
        Add New Provider
      </Button>
      <BankAccountDataProviderAddModal
        closeModal={closeModal}
        isOpen={isModalOpen}
      />
    </>
  );
};

export default BankAccountDataProvider;
