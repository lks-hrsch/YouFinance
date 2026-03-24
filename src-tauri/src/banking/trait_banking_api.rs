use super::{
    apierror::*,
    providers::gocardless::structs::*,
};

#[allow(async_fn_in_trait)]
pub trait BankingApi: Send + Sync {
    async fn get_access_token(&self) -> Result<AccessToken, ApiError>;
    async fn get_banks_by_country(
        &self,
        country: &str,
    ) -> Result<Vec<Bank>, ApiError>;
    async fn connect_bank(
        &self,
        redirect: &str,
        institution_id: &str,
    ) -> Result<BankConnection, ApiError>;
    async fn disconnect_bank(
        &self,
        bank_connection_id: &str,
    ) -> Result<(), ApiError>;
    async fn get_bank_accounts(
        &self,
        bank_connection_id: &str,
    ) -> Result<BankAccounts, ApiError>;
    async fn get_account_transactions(
        &self,
        account_id: &str,
    ) -> Result<BankTransactions, ApiError>;
}
