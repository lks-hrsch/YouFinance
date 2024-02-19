/*
 Generated by typeshare 1.7.0
*/

export interface Provider {
  id: number;
  title: string;
  secret_id?: string;
  secret_key?: string;
}

export interface NewProvider {
  title: string;
  secret_id?: string;
  secret_key?: string;
}

export interface BankInfo {
  id: string;
  name: string;
}

export interface BankConnectionInfo {
  id: string;
  link: string;
}

export interface Account {
  id: number;
  provider_id: number;
  title: string;
  institution_id?: string;
  bank_connection_id?: string;
  account_id?: string;
  iban?: string;
}

export interface NewAccount {
  title: string;
  provider_id: number;
  institution_id: string;
  bank_connection_id: string;
  account_id: string;
}

export interface Transaction {
  id: number;
  title: string;
  debitor_name?: string;
  debitor_iban?: string;
  creditor_name?: string;
  creditor_iban?: string;
  amount: number;
  currency: string;
  date: string;
  remittance_information?: string;
  account_id: number;
}

export interface NewTransaction {
  title: string;
  debitor_name?: string;
  debitor_iban?: string;
  creditor_name?: string;
  creditor_iban?: string;
  amount: number;
  currency: string;
  date: string;
  remittance_information?: string;
  account_id: number;
}

export interface Tag {
  id: number;
  title: string;
}

export interface TransactionTag {
  transaction_id: number;
  tag_id: number;
}

export enum BankingProviders {
  GoCardless = "GoCardless",
}
