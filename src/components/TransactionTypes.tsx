// TypeScript interfaces mirroring the Rust structs

export interface TransactionAmount {
  currency: string;
  amount: string;
}

export interface DebtorAccount {
  iban: string;
}

export interface Transaction {
  transactionId?: string | null;
  debtorName?: string | null;
  debtorAccount?: DebtorAccount | null;
  transactionAmount: TransactionAmount;
  bookingDate: string;
  valueDate: string;
  remittanceInformationUnstructured?: string | null;
  bankTransactionCode?: string | null;
}

export interface Transactions {
  booked: Transaction[];
  pending: Transaction[];
}

export interface BankTransactions {
  transactions: Transactions;
}
