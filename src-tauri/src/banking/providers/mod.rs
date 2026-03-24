pub mod gocardless;
pub mod local_csv;

use std::fmt;

use diesel::{
    query_dsl::methods::FilterDsl,
    ExpressionMethods,
    RunQueryDsl,
    SqliteConnection,
};
use typeshare::typeshare;

use super::trait_banking_api::BankingApi;

#[typeshare]
#[derive(Debug)]
pub enum BankingProviders {
    GoCardless,
    LocalCSV,
}

impl fmt::Display for BankingProviders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BankingProviders::GoCardless => "GoCardless",
                BankingProviders::LocalCSV => "LocalCSV",
            }
        )
    }
}

pub enum ProviderInstance {
    GoCardless(gocardless::structs::GoCardless),
    LocalCSV(local_csv::LocalCSV),
}

impl BankingApi for ProviderInstance {
    async fn get_access_token(&self) -> Result<crate::banking::providers::gocardless::structs::AccessToken, crate::banking::apierror::ApiError> {
        match self {
            Self::GoCardless(g) => g.get_access_token().await,
            Self::LocalCSV(l) => l.get_access_token().await,
        }
    }

    async fn get_banks_by_country(
        &self,
        country: &str,
    ) -> Result<Vec<crate::banking::providers::gocardless::structs::Bank>, crate::banking::apierror::ApiError> {
        match self {
            Self::GoCardless(g) => g.get_banks_by_country(country).await,
            Self::LocalCSV(l) => l.get_banks_by_country(country).await,
        }
    }

    async fn connect_bank(
        &self,
        redirect: &str,
        institution_id: &str,
    ) -> Result<crate::banking::providers::gocardless::structs::BankConnection, crate::banking::apierror::ApiError> {
        match self {
            Self::GoCardless(g) => g.connect_bank(redirect, institution_id).await,
            Self::LocalCSV(l) => l.connect_bank(redirect, institution_id).await,
        }
    }

    async fn disconnect_bank(&self, bank_connection_id: &str) -> Result<(), crate::banking::apierror::ApiError> {
        match self {
            Self::GoCardless(g) => g.disconnect_bank(bank_connection_id).await,
            Self::LocalCSV(l) => l.disconnect_bank(bank_connection_id).await,
        }
    }

    async fn get_bank_accounts(
        &self,
        bank_connection_id: &str,
    ) -> Result<crate::banking::providers::gocardless::structs::BankAccounts, crate::banking::apierror::ApiError> {
        match self {
            Self::GoCardless(g) => g.get_bank_accounts(bank_connection_id).await,
            Self::LocalCSV(l) => l.get_bank_accounts(bank_connection_id).await,
        }
    }

    async fn get_account_transactions(
        &self,
        account_id: &str,
    ) -> Result<crate::banking::providers::gocardless::structs::BankTransactions, crate::banking::apierror::ApiError> {
        match self {
            Self::GoCardless(g) => g.get_account_transactions(account_id).await,
            Self::LocalCSV(l) => l.get_account_transactions(account_id).await,
        }
    }
}

impl BankingProviders {
    pub fn list_providers() -> Vec<String> {
        vec![
            format!("{}", BankingProviders::GoCardless.to_string()),
            format!("{}", BankingProviders::LocalCSV.to_string()),
        ]
    }

    pub fn from_string(str: &str) -> Option<BankingProviders> {
        match str {
            "GoCardless" => Some(BankingProviders::GoCardless),
            "LocalCSV" => Some(BankingProviders::LocalCSV),
            _ => None,
        }
    }

    pub async fn connect_provider(
        &self,
        connection: &mut SqliteConnection,
        app_data_path: std::path::PathBuf,
    ) -> Result<ProviderInstance, String> {
        match self {
            BankingProviders::GoCardless => {
                let provider = crate::schema::providers::table
                    .filter(crate::schema::providers::title.eq(self.to_string()))
                    .first::<crate::model::Provider>(connection)
                    .map_err(|e| e.to_string())?;

                let sid = provider.secret_id.ok_or("Secret ID not found for provider")?;
                let skey = provider.secret_key.ok_or("Secret Key not found for provider")?;

                let gocardless = crate::banking::providers::gocardless::structs::GoCardless::new(&sid, &skey)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(ProviderInstance::GoCardless(gocardless))
            }
            BankingProviders::LocalCSV => {
                let mut local_csv = crate::banking::providers::local_csv::LocalCSV::new();
                local_csv.set_data_dir(app_data_path);
                Ok(ProviderInstance::LocalCSV(local_csv))
            }
        }
    }
}
