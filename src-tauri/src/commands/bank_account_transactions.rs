use diesel::{
    associations::HasTable,
    ExpressionMethods,
    QueryDsl,
    RunQueryDsl,
    SelectableHelper,
    OptionalExtension,
};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_log::log::debug;
use tokio::sync::Mutex;
use chrono::Utc;

use crate::{
    banking::{
        apierror::ApiError,
        providers::BankingProviders,
        sync::{classify_dedup_outcome, DeduplicationOutcome},
        trait_banking_api::BankingApi,
        utils::normalize_date,
    },
    database::DatabaseState,
    model::*,
};

async fn sync_accounts_internal(
    app_handle: AppHandle,
    database_state: &Mutex<DatabaseState>,
    accounts_to_sync: Vec<BankAccountProvider>,
) -> Result<(), String> {
    use crate::schema::{
        bank_account_providers::dsl as bap_dsl,
        providers::dsl as providers_dsl,
    };

    // TODO: The database connection is held for the duration of all HTTP calls.
    // For better performance and responsiveness, consider releasing the lock between
    // HTTP calls and re-acquiring it only when needed for database operations.
    let connection = &mut database_state.lock().await.connection();

    fn transform_transaction(
        old_trans: &crate::banking::providers::gocardless::structs::Transaction,
        bank_account_id: i32,
    ) -> Result<NewTransaction, String> {
        let debtor_iban = old_trans.debtor_account.as_ref().map(|account| account.iban.clone()).flatten();
        let creditor_iban = old_trans
            .creditor_account
            .as_ref()
            .map(|account| account.iban.clone())
            .flatten();
        let amount_f64: f64 = old_trans.transaction_amount.amount.parse()
            .map_err(|_| format!("Invalid amount format: '{}'", old_trans.transaction_amount.amount))?;
        let amount_minor: i32 = (amount_f64 * 100.0).round() as i32;
        let booking_date = old_trans.booking_date.clone().unwrap_or_default();
        let booking_text = old_trans.bank_transaction_code.clone().unwrap_or_default();

        Ok(NewTransaction {
            booking_text,
            debtor_name: old_trans.debtor_name.clone(),
            debtor_iban,
            debtor_bic: None,
            creditor_name: old_trans.creditor_name.clone(),
            creditor_iban,
            creditor_bic: None,
            amount_minor,
            currency_code: old_trans.transaction_amount.clone().currency,
            booking_date: normalize_date(&booking_date),
            value_date: old_trans.value_date.clone(),
            balance_after_minor: old_trans.balance_after_minor,
            mandate_reference: None,
            remittance_information: old_trans.remittance_information_unstructured.clone(),
            bank_account_id,
        })
    }

    let app_data_dir = app_handle.path().app_data_dir().map_err(|e: tauri::Error| e.to_string())?;

    for account in accounts_to_sync {
        let provider_data: Provider = providers_dsl::providers
            .filter(providers_dsl::id.eq(account.provider_id))
            .first::<Provider>(connection)
            .map_err(|e| format!("Failed to load provider for account {}: {}", account.id, e))?;

        let provider_enum = BankingProviders::from_string(&provider_data.name)
            .ok_or_else(|| format!("Invalid provider: {}", provider_data.name))?;

        let provider_instance = provider_enum.connect_provider(connection, app_data_dir.clone()).await?;

        let account_id_str = account.external_account_id.clone().unwrap_or_default();
        let account_transactions = provider_instance
            .get_account_transactions(&account_id_str)
            .await
            .map_err(|e: ApiError| format!("Failed to fetch transactions from provider for account {}: {}", account.id, e))?;

        let transactions_to_save: Vec<NewTransaction> = account_transactions
            .transactions
            .booked
            .iter()
            .map(|elem| transform_transaction(elem, account.bank_account_id))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to transform transactions: {}", e))?;

        let mut conflicts: Vec<String> = Vec::new();

        for transaction in transactions_to_save {
            let maybe_existing = find_existing_transaction(connection, &transaction)?;
            let outcome = classify_dedup_outcome(maybe_existing.as_ref(), account.bank_account_id);

            match outcome {
                DeduplicationOutcome::New => {
                    insert_transaction_and_link(connection, &transaction, provider_data.id)?;
                }
                DeduplicationOutcome::SameAccount { existing_id } => {
                    if let Some(conflict_msg) = check_transaction_conflict(maybe_existing.as_ref().unwrap(), &transaction) {
                        conflicts.push(format!(
                            "[account {}] transaction {} {} {}: {}",
                            account.id, transaction.booking_date, transaction.amount_minor, transaction.currency_code, conflict_msg
                        ));
                    } else {
                        upsert_provider_link(connection, existing_id, provider_data.id)?;
                    }
                }
                DeduplicationOutcome::CrossAccount { existing_id, existing_account_id } => {
                    debug!(
                        "cross-account dedup: {} {}{} matches existing id={}",
                        transaction.booking_date, transaction.amount_minor, transaction.currency_code,
                        existing_id
                    );
                    upsert_provider_link(connection, existing_id, provider_data.id)?;
                }
            }
        }

        if !conflicts.is_empty() {
            return Err(format!(
                "Sync completed with data conflicts:\n{}",
                conflicts.join("\n")
            ));
        }

        // Update last_synced_at for this bank_account_provider
        let now = Utc::now().to_rfc3339();
        diesel::update(bap_dsl::bank_account_providers.filter(bap_dsl::id.eq(account.id)))
            .set(bap_dsl::last_synced_at.eq(&now))
            .execute(connection)
            .map_err(|e| format!("Failed to update last_synced_at for account {}: {}", account.id, e))?;
    }

    Ok(())
}

/// Inserts a new transaction and adds the initial provider link in one go.
fn insert_transaction_and_link(
    connection: &mut diesel::sqlite::SqliteConnection,
    transaction: &NewTransaction,
    provider_id: i32,
) -> Result<(), String> {
    use crate::schema::{
        transaction_providers::dsl as tp_dsl,
        transactions::dsl as transactions_dsl,
    };

    diesel::insert_into(transactions_dsl::transactions::table())
        .values(transaction)
        .execute(connection)
        .map_err(|e| format!("Failed to save transaction: {}", e))?;

    let inserted = find_existing_transaction(connection, transaction)?
        .ok_or_else(|| format!(
            "Failed to retrieve transaction after insert: {} {}",
            transaction.booking_date, transaction.amount_minor
        ))?;

    diesel::insert_into(tp_dsl::transaction_providers::table())
        .values(&NewTransactionProvider {
            transaction_id: inserted.id,
            provider_id,
        })
        .on_conflict_do_nothing()
        .execute(connection)
        .map_err(|e| format!("Failed to save transaction_provider link: {}", e))?;

    Ok(())
}

/// Idempotently adds a provider link to an existing transaction.
fn upsert_provider_link(
    connection: &mut diesel::sqlite::SqliteConnection,
    transaction_id: i32,
    provider_id: i32,
) -> Result<(), String> {
    use crate::schema::transaction_providers::dsl as tp_dsl;

    diesel::insert_into(tp_dsl::transaction_providers::table())
        .values(&NewTransactionProvider {
            transaction_id,
            provider_id,
        })
        .on_conflict_do_nothing()
        .execute(connection)
        .map_err(|e| format!("Failed to upsert transaction_provider link: {}", e))?;

    Ok(())
}

/// Finds an existing transaction that matches the financial fingerprint: (date, amount, debtor_iban, creditor_iban).
///
/// Note: Does NOT filter by account_id — matches globally across accounts to detect
/// the same real-world transaction imported from multiple providers.
///
/// Intentionally excludes `remittance_information` — it is not part of the constraint
/// and may legitimately vary between providers (truncation, encoding differences).
fn find_existing_transaction(
    connection: &mut diesel::sqlite::SqliteConnection,
    transaction: &NewTransaction,
) -> Result<Option<Transaction>, String> {
    use crate::schema::transactions::dsl as transactions_dsl;
    use diesel::QueryDsl;

    let mut query = transactions_dsl::transactions
        .filter(transactions_dsl::booking_date.eq(&transaction.booking_date))
        .filter(transactions_dsl::amount_minor.eq(transaction.amount_minor))
        .into_boxed();

    if let Some(ref d_iban) = transaction.debtor_iban {
        query = query.filter(transactions_dsl::debtor_iban.eq(d_iban));
    } else {
        query = query.filter(transactions_dsl::debtor_iban.is_null());
    }

    if let Some(ref c_iban) = transaction.creditor_iban {
        query = query.filter(transactions_dsl::creditor_iban.eq(c_iban));
    } else {
        query = query.filter(transactions_dsl::creditor_iban.is_null());
    }

    query
        .select(Transaction::as_select())
        .first::<Transaction>(connection)
        .optional()
        .map_err(|e| e.to_string())
}

/// Checks if the stored transaction matches the incoming transaction on immutable financial fields.
///
/// Only checks `amount` and `currency` — the financial identity that must never change.
/// Ignores `title`, `remittance_information`, `debitor_name`, `creditor_name` as these
/// legitimately vary between providers (truncation, language, encoding).
fn check_transaction_conflict(existing: &Transaction, incoming: &NewTransaction) -> Option<String> {
    if existing.amount_minor != incoming.amount_minor {
        return Some(format!(
            "amount mismatch: stored={}, incoming={}",
            existing.amount_minor, incoming.amount_minor
        ));
    }
    if existing.currency_code != incoming.currency_code {
        return Some(format!(
            "currency mismatch: stored={}, incoming={}",
            existing.currency_code, incoming.currency_code
        ));
    }
    None
}

#[tauri::command]
pub async fn sync_all_accounts(
    app_handle: AppHandle,
    database_state: State<'_, Mutex<DatabaseState>>,
) -> Result<(), String> {
    debug!("commands::bank_account_transactions::sync_all_accounts");
    use crate::schema::bank_account_providers::dsl::*;

    let all_accounts: Vec<BankAccountProvider>;
    {
        let connection = &mut database_state.lock().await.connection();
        all_accounts = bank_account_providers
            .select(BankAccountProvider::as_select())
            .load(connection)
            .map_err(|e| format!("Failed to load accounts: {}", e))?;
    }

    sync_accounts_internal(app_handle, &database_state, all_accounts).await
}

#[tauri::command]
pub async fn sync_provider_accounts(
    app_handle: AppHandle,
    database_state: State<'_, Mutex<DatabaseState>>,
    p_id: i32,
) -> Result<(), String> {
    debug!("commands::bank_account_transactions::sync_provider_accounts: {}", p_id);
    use crate::schema::bank_account_providers::dsl::*;

    let filtered_accounts: Vec<BankAccountProvider>;
    {
        let connection = &mut database_state.lock().await.connection();
        filtered_accounts = bank_account_providers
            .filter(provider_id.eq(p_id))
            .select(BankAccountProvider::as_select())
            .load(connection)
            .map_err(|e| format!("Failed to load accounts for provider {}: {}", p_id, e))?;
    }

    sync_accounts_internal(app_handle, &database_state, filtered_accounts).await
}

#[tauri::command]
pub async fn sync_account(
    app_handle: AppHandle,
    database_state: State<'_, Mutex<DatabaseState>>,
    target_account_id: i32,
) -> Result<(), String> {
    debug!("commands::bank_account_transactions::sync_account");
    use crate::schema::bank_account_providers::dsl::*;

    let filtered_accounts: Vec<BankAccountProvider>;
    {
        let connection = &mut database_state.lock().await.connection();
        filtered_accounts = bank_account_providers
            .filter(id.eq(target_account_id))
            .select(BankAccountProvider::as_select())
            .load(connection)
            .map_err(|e| format!("Failed to load account {}: {}", target_account_id, e))?;
    }

    sync_accounts_internal(app_handle, &database_state, filtered_accounts).await
}



#[tauri::command]
pub async fn get_transactions(database_state: State<'_, Mutex<DatabaseState>>) -> Result<Vec<Transaction>, String> {
    debug!("commands::bank_account_transactions::get_transactions");
    use crate::schema::transactions::dsl::*;

    let connection = &mut database_state.lock().await.connection();

    let all_transactions: Vec<Transaction> = transactions
        .select(Transaction::as_select())
        .order_by(booking_date.desc())
        .load(connection)
        .map_err(|e| format!("Failed to load transactions: {}", e))?;

    Ok(all_transactions)
}

#[tauri::command]
pub async fn get_transactions_with_providers(database_state: State<'_, Mutex<DatabaseState>>) -> Result<Vec<TransactionWithProviders>, String> {
    debug!("commands::bank_account_transactions::get_transactions_with_providers");
    use crate::schema::{
        providers::dsl as providers_dsl,
        transaction_providers::dsl as tp_dsl,
        transactions::dsl as transactions_dsl,
    };

    let connection = &mut database_state.lock().await.connection();

    let all_transactions: Vec<Transaction> = transactions_dsl::transactions
        .select(Transaction::as_select())
        .order_by(transactions_dsl::booking_date.desc())
        .load(connection)
        .map_err(|e| format!("Failed to load transactions: {}", e))?;

    let mut result = Vec::new();

    for transaction in all_transactions {
        let provider_names: Vec<String> = tp_dsl::transaction_providers
            .filter(tp_dsl::transaction_id.eq(transaction.id))
            .inner_join(providers_dsl::providers)
            .select(providers_dsl::name)
            .load::<String>(connection)
            .unwrap_or_default();

        result.push(TransactionWithProviders {
            transaction,
            providers: provider_names,
        });
    }

    Ok(result)
}
