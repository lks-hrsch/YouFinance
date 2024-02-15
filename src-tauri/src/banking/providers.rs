use std::fmt;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug)]
pub enum BankingProviders {
    GoCardless
    // Add other providers here as needed
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
}