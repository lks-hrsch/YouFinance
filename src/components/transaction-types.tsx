// TypeScript interfaces mirroring the Rust structs

export type TransactionAmount = {
  currency: string;
  amount: string;
};

export type DebtorAccount = {
  iban: string;
};

export type Transaction = {
  transactionId?: string | null;
  debtorName?: string | null;
  debtorAccount?: DebtorAccount | null;
  transactionAmount: TransactionAmount;
  bookingDate: string;
  valueDate: string;
  remittanceInformationUnstructured?: string | null;
  bankTransactionCode?: string | null;
};

export type Transactions = {
  booked: Transaction[];
  pending: Transaction[];
};

export type BankTransactions = {
  transactions: Transactions;
};
