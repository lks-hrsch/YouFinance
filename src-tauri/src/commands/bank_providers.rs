use diesel::{
    associations::HasTable,
    QueryDsl,
    RunQueryDsl,
    SelectableHelper,
};

use crate::{
    banking::providers::BankingProviders,
    model::*,
};

#[tauri::command]
pub fn list_possible_banking_providers() -> Vec<String> {
    BankingProviders::list_providers()
}

#[tauri::command]
pub fn get_banking_providers() -> Vec<Provider> {
    use crate::schema::providers::dsl::*;

    let connection = &mut crate::database::establish_db_connection();
    let p: Vec<Provider> = providers
        .select(Provider::as_select())
        .load(connection)
        .expect("error loading providers");
    p
}

#[tauri::command]
pub fn add_banking_provider(name: String, sid: Option<String>, skey: Option<String>) {
    use crate::schema::providers::dsl::*;

    let connection = &mut crate::database::establish_db_connection();
    let new_provider = NewProvider {
        title: name,
        secret_id: sid,
        secret_key: skey,
    };
    diesel::insert_into(providers::table())
        .values(&new_provider)
        .execute(connection)
        .expect("error saving new provider");
}
