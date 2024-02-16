use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};
use std::fmt;
use typeshare::typeshare;

use super::trait_banking_api::BankingApi;

#[typeshare]
#[derive(Debug)]
pub enum BankingProviders {
    GoCardless, // Add other providers here as needed
}

impl fmt::Display for BankingProviders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BankingProviders::GoCardless => "GoCardless",
            }
        )
    }
}

impl BankingProviders {
    pub fn list_providers() -> Vec<String> {
        vec![
            format!("{}", BankingProviders::GoCardless.to_string()),
            // Add other variants here as needed
        ]
    }

    pub fn from_string(str: &str) -> Option<BankingProviders> {
        match str {
            "GoCardless" => Some(BankingProviders::GoCardless),
            _ => None,
        }
    }

    pub async fn connect_provider(&self) -> Result<crate::banking::provider_gocardless_structs::GoCardless, String> {
        match self {
            BankingProviders::GoCardless => {
                let connection = &mut crate::database::establish_db_connection();
                let provider = crate::schema::providers::table
                    .filter(crate::schema::providers::title.eq(self.to_string()))
                    .first::<crate::Provider>(connection)
                    .map_err(|e| e.to_string())?;

                let sid = provider.secret_id.ok_or("Secret ID not found for provider")?;
                let skey = provider.secret_key.ok_or("Secret Key not found for provider")?;

                let gocardless = crate::banking::provider_gocardless_structs::GoCardless::new(&sid, &skey)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(gocardless)
            }
        }
    }
}
