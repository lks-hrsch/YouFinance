// TransactionComponent.tsx
import React from "react";

import { Transaction } from "../models/typeshare_definitions";

interface TransactionProps {
  transaction: Transaction;
}

const TransactionComponent: React.FC<TransactionProps> = ({ transaction }) => (
  <li>
    <strong>Booking Date:</strong> {transaction.date}
    <br />
    <strong>Debtor Name:</strong> {transaction.debitor_name}
    <br />
    <strong>Verwensungszweck:</strong> {transaction.remittance_information}
    <br />
    <strong>Ammount:</strong> {transaction.ammount} {transaction.currency}
    <br />
  </li>
);

export default TransactionComponent;
