// @generated automatically by Diesel CLI.

diesel::table! {
    bank_account_providers (id) {
        id -> Integer,
        bank_account_id -> Integer,
        provider_id -> Integer,
        bank_connection_id -> Text,
        institution_id -> Nullable<Text>,
        external_account_id -> Nullable<Text>,
        last_synced_at -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

diesel::table! {
    bank_accounts (id) {
        id -> Integer,
        name -> Text,
        iban -> Nullable<Text>,
        bic -> Nullable<Text>,
        owner_name -> Nullable<Text>,
        currency_code -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

diesel::table! {
    providers (id) {
        id -> Integer,
        name -> Text,
        config_json -> Text,
        created_at -> Text,
        updated_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        created_at -> Text,
        updated_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

diesel::table! {
    transaction_providers (transaction_id, provider_id) {
        transaction_id -> Integer,
        provider_id -> Integer,
        created_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

diesel::table! {
    transaction_tags (transaction_id, tag_id) {
        transaction_id -> Integer,
        tag_id -> Integer,
        created_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        booking_text -> Text,
        debtor_name -> Nullable<Text>,
        debtor_iban -> Nullable<Text>,
        debtor_bic -> Nullable<Text>,
        creditor_name -> Nullable<Text>,
        creditor_iban -> Nullable<Text>,
        creditor_bic -> Nullable<Text>,
        amount_minor -> Integer,
        currency_code -> Text,
        booking_date -> Text,
        value_date -> Nullable<Text>,
        balance_after_minor -> Nullable<Integer>,
        mandate_reference -> Nullable<Text>,
        remittance_information -> Nullable<Text>,
        bank_account_id -> Integer,
        created_at -> Text,
        updated_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

diesel::joinable!(bank_account_providers -> bank_accounts (bank_account_id));
diesel::joinable!(bank_account_providers -> providers (provider_id));
diesel::joinable!(transaction_providers -> providers (provider_id));
diesel::joinable!(transaction_providers -> transactions (transaction_id));
diesel::joinable!(transaction_tags -> tags (tag_id));
diesel::joinable!(transaction_tags -> transactions (transaction_id));
diesel::joinable!(transactions -> bank_accounts (bank_account_id));

diesel::allow_tables_to_appear_in_same_query!(
    bank_account_providers,
    bank_accounts,
    providers,
    tags,
    transaction_providers,
    transaction_tags,
    transactions,
);
