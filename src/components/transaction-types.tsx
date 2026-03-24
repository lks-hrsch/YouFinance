// TypeScript interfaces mirroring the Rust structs

export interface TransactionAmount {
  amount: string;
  currency: string;
}

export interface DebtorAccount {
  iban: string;
}

export interface Transaction {
  bankTransactionCode?: string | null;
  bookingDate: string;
  debtorAccount?: DebtorAccount | null;
  debtorName?: string | null;
  remittanceInformationUnstructured?: string | null;
  transactionAmount: TransactionAmount;
  transactionId?: string | null;
  valueDate: string;
}

export interface Transactions {
  booked: Transaction[];
  pending: Transaction[];
}

export interface BankTransactions {
  transactions: Transactions;
}
