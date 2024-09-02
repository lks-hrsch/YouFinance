use super::{
    apierror::*,
    providers::gocardless::structs::*,
};

pub trait BankingApi {
    fn new(secret_id: &str, secret_key: &str)
        -> impl std::future::Future<Output = Result<GoCardless, ApiError>> + Send;
    fn get_access_token(&self) -> impl std::future::Future<Output = Result<AccessToken, ApiError>> + Send;
    fn get_banks_by_country(
        &self,
        country: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Bank>, ApiError>> + Send;
    fn connect_bank(
        &self,
        redirect: &str,
        institution_id: &str,
    ) -> impl std::future::Future<Output = Result<BankConnection, ApiError>> + Send;
    fn disconnect_bank(
        &self,
        bank_connection_id: &str,
    ) -> impl std::future::Future<Output = Result<(), ApiError>> + Send;
    fn get_bank_accounts(
        &self,
        bank_connection_id: &str,
    ) -> impl std::future::Future<Output = Result<BankAccounts, ApiError>> + Send;
    fn get_account_transactions(
        &self,
        account_id: &str,
    ) -> impl std::future::Future<Output = Result<BankTransactions, ApiError>> + Send;
}
