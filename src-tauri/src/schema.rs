// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Integer,
        provider_id -> Integer,
        title -> Text,
        institution_id -> Nullable<Text>,
        bank_connection_id -> Nullable<Text>,
        account_id -> Nullable<Text>,
        iban -> Nullable<Text>,
    }
}

diesel::table! {
    providers (id) {
        id -> Integer,
        title -> Text,
        secret_id -> Nullable<Text>,
        secret_key -> Nullable<Text>,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        title -> Text,
    }
}

diesel::table! {
    transaction_tags (transaction_id, tag_id) {
        transaction_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        title -> Text,
        debitor_name -> Nullable<Text>,
        debitor_iban -> Nullable<Text>,
        creditor_name -> Nullable<Text>,
        creditor_iban -> Nullable<Text>,
        amount -> Double,
        currency -> Text,
        date -> Text,
        remittance_information -> Nullable<Text>,
        account_id -> Integer,
    }
}

diesel::joinable!(accounts -> providers (provider_id));
diesel::joinable!(transaction_tags -> tags (tag_id));
diesel::joinable!(transaction_tags -> transactions (transaction_id));
diesel::joinable!(transactions -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    providers,
    tags,
    transaction_tags,
    transactions,
);
