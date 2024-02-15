import React, { useState } from "react";
import BankAccountDataProviderAddModal from "../../components/BankAccountDataProvider/AddModal";
import BankAccountDataProviderList from "../../components/BankAccountDataProvider/List";

import { Button } from "@material-tailwind/react";

const BankAccountDataProvider: React.FC = () => {
  const [isModalOpen, setModalOpen] = useState(false);

  const openModal = () => setModalOpen(true);
  const closeModal = () => setModalOpen(false);

  return (
    <>
      <BankAccountDataProviderList />
      <Button onClick={openModal} placeholder={undefined}>
        Add New Provider
      </Button>
      <BankAccountDataProviderAddModal
        isOpen={isModalOpen}
        closeModal={closeModal}
      />
    </>
  );
};

export default BankAccountDataProvider;
