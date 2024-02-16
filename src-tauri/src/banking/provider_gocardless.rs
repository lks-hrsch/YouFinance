extern crate reqwest;

use super::apierror::*;
use super::provider_gocardless_structs::*;
use super::trait_banking_api::BankingApi;
use std::collections::HashMap;

impl GoCardless {
    // Check if access_token is available
    fn add_authorization_header(&self, headers: &mut reqwest::header::HeaderMap) -> Result<(), ApiError> {
        if let Some(access_token) = &self.access_token {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", access_token.access))?,
            );
            Ok(())
        } else {
            Err(ApiError::Custom("Access token not available".into()))
        }
    }
}

impl BankingApi for GoCardless {
    async fn new(secret_id: &str, secret_key: &str) -> Result<GoCardless, ApiError> {
        let mut this = GoCardless {
            base_url: "https://bankaccountdata.gocardless.com/api/v2/".to_string(),
            secret_id: secret_id.to_string(),
            secret_key: secret_key.to_string(),
            access_token: None,
        };

        this.access_token = Some(this.get_access_token().await?);

        Ok(this)
    }

    async fn get_access_token(&self) -> Result<AccessToken, ApiError> {
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("accept", reqwest::header::HeaderValue::from_static("application/json"));
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let mut map = HashMap::new();
        map.insert("secret_id", &self.secret_id);
        map.insert("secret_key", &self.secret_key);

        let res = client
            .post(&format!("{}{}", self.base_url.to_owned(), "token/new/"))
            .headers(headers)
            .json(&map)
            .send()
            .await?;

        let body = res.text().await?;
        let access_token: AccessToken = serde_json::from_str(&body)?;

        Ok(access_token)
    }

    async fn get_banks_by_country(&self, country: &str) -> Result<Vec<Bank>, ApiError> {
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("accept", reqwest::header::HeaderValue::from_static("application/json"));
        self.add_authorization_header(&mut headers)?;

        let res = client
            .get(&format!(
                "{}institutions/?country={}",
                self.base_url.to_owned(),
                country
            ))
            .headers(headers)
            .send()
            .await?;

        let body = res.text().await?;
        let banks: Vec<Bank> = serde_json::from_str(&body)?;

        Ok(banks)
    }

    async fn connect_bank(&self, redirect: &str, institution_id: &str) -> Result<BankConnection, ApiError> {
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("accept", reqwest::header::HeaderValue::from_static("application/json"));
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        self.add_authorization_header(&mut headers)?;

        let mut map = HashMap::new();
        map.insert("redirect", redirect);
        map.insert("institution_id", institution_id);
        // map.insert("reference", reference);
        // map.insert("agreement", agreement);
        // map.insert("user_language", user_language);

        let res = client
            .post(&format!("{}requisitions/", self.base_url.to_owned()))
            .headers(headers)
            .json(&map)
            .send()
            .await?;

        let body = res.text().await?;
        let bank_connection: BankConnection = serde_json::from_str(&body)?;

        Ok(bank_connection)
    }

    async fn get_bank_accounts(&self, bank_connection_id: &str) -> Result<BankAccounts, ApiError> {
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("accept", reqwest::header::HeaderValue::from_static("application/json"));
        self.add_authorization_header(&mut headers)?;

        let res = client
            .get(&format!(
                "{}requisitions/{}",
                self.base_url.to_owned(),
                bank_connection_id
            ))
            .headers(headers)
            .send()
            .await?;

        let body = res.text().await?;
        let bank_accounts: BankAccounts = serde_json::from_str(&body)?;

        Ok(bank_accounts)
    }

    async fn get_account_transactions(&self, account_id: &str) -> Result<BankTransactions, ApiError> {
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("accept", reqwest::header::HeaderValue::from_static("application/json"));
        self.add_authorization_header(&mut headers)?;

        let res = client
            .get(&format!(
                "{}accounts/{}/transactions/",
                self.base_url.to_owned(),
                account_id
            ))
            .headers(headers)
            .send()
            .await?;

        let body = res.text().await?;
        let bank_transactions: BankTransactions = serde_json::from_str(&body)?;

        Ok(bank_transactions)
    }
}
