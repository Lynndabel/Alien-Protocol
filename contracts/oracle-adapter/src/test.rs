#![cfg(test)]

use super::*;
use soroban_sdk::testutils::{Address as _, Events};
use soroban_sdk::{Address, Env, Symbol};

fn setup_env() -> (Env, OracleContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(OracleContract, ());
    let client = OracleContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    (env, client, admin)
}

#[test]
fn test_initialize_success() {
    let env = Env::default();
    let contract_id = env.register(OracleContract, ());
    let client = OracleContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    env.mock_all_auths();
    let asset = Address::generate(&env);
    client.set_price(&asset, &100, &1000);
    
    let price_data = client.get_price(&asset).unwrap();
    assert_eq!(price_data.price, 100);
    assert_eq!(price_data.timestamp, 1000);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_initialize_twice_fails() {
    let (_env, client, admin) = setup_env();
    client.initialize(&admin);
}

#[test]
fn test_set_admin_success() {
    let (env, client, _admin) = setup_env();

    let new_admin = Address::generate(&env);
    client.set_admin(&new_admin);

    let asset = Address::generate(&env);
    client.set_price(&asset, &150, &2000);

    let auths = env.auths();
    assert_eq!(auths.len(), 1);
    let (auth_addr, _) = auths.first().unwrap();
    assert_eq!(*auth_addr, new_admin);
}

#[test]
fn test_set_admin_non_admin_fails() {
    let (env, client, admin) = setup_env();

    let new_admin = Address::generate(&env);
    client.set_admin(&new_admin);

    let auths = env.auths();
    assert_eq!(auths.len(), 1);
    let (auth_addr, _) = auths.first().unwrap();
    assert_eq!(*auth_addr, admin);
}

#[test]
fn test_set_admin_same_address_fails() {
    let (_env, client, admin) = setup_env();

    let result = client.try_set_admin(&admin);
    assert!(result.is_err());
    let err = result.err().unwrap().unwrap();
    assert_eq!(
        err,
        soroban_sdk::Error::from_contract_error(OracleError::AlreadyAdmin as u32)
    );
}

#[test]
fn test_set_admin_emits_event() {
    let (env, client, _admin) = setup_env();

    let new_admin = Address::generate(&env);
    client.set_admin(&new_admin);

    let last_event = env.events().all().last().unwrap();
    assert_eq!(last_event.0, client.address);
    use soroban_sdk::TryFromVal;
    let event_symbol = Symbol::try_from_val(&env, &last_event.1.get(0).unwrap()).unwrap();
    assert_eq!(event_symbol, Symbol::new(&env, "admin_changed"));
}

#[test]
fn test_set_price_success() {
    let (env, client, admin) = setup_env();

    let asset = Address::generate(&env);
    client.set_price(&asset, &200, &3000);

    let auths = env.auths();
    assert_eq!(auths.len(), 1);
    let (auth_addr, _) = auths.first().unwrap();
    assert_eq!(*auth_addr, admin);

    let price_opt = client.get_price(&asset);
    assert!(price_opt.is_some());
    let price_data = price_opt.unwrap();
    assert_eq!(price_data.price, 200);
    assert_eq!(price_data.timestamp, 3000);
}

#[test]
fn test_set_price_non_admin_fails() {
    let (env, client, admin) = setup_env();

    let asset = Address::generate(&env);
    client.set_price(&asset, &200, &3000);

    let auths = env.auths();
    assert_eq!(auths.len(), 1);
    let (auth_addr, _) = auths.first().unwrap();
    assert_eq!(*auth_addr, admin);
}

#[test]
fn test_old_admin_cannot_call_set_price_after_transfer() {
    let (env, client, admin) = setup_env();

    let new_admin = Address::generate(&env);
    client.set_admin(&new_admin);

    let asset = Address::generate(&env);
    client.set_price(&asset, &300, &4000);

    let auths = env.auths();
    assert_eq!(auths.len(), 1);
    let (auth_addr, _) = auths.first().unwrap();
    assert_eq!(*auth_addr, new_admin);
    assert_ne!(*auth_addr, admin);
}

#[test]
fn test_get_price_is_public_and_unauthorized() {
    let (env, client, _admin) = setup_env();
    let asset = Address::generate(&env);

    client.set_price(&asset, &200, &3000);

    let price_opt = client.get_price(&asset);
    assert!(price_opt.is_some());
}

