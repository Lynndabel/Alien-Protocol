#![cfg(test)]

use super::*;
use soroban_sdk::testutils::{Address as _, Events};
use soroban_sdk::{Address, Env, Symbol, TryFromVal};

fn setup_env() -> (Env, OracleContractClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(OracleContract, ());
    let client = OracleContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let asset = Address::generate(&env);

    client.initialize(&admin);

    (env, client, admin, asset)
}

#[test]
fn test_initialize_sets_admin() {
    let (env, _client, admin, _asset) = setup_env();

    let auths = env.auths();
    assert_eq!(auths.len(), 1);
    let (auth_addr, _) = auths.first().unwrap();
    assert_eq!(*auth_addr, admin);
}

#[test]
fn test_initialize_twice_fails() {
    let (env, client, _admin, _asset) = setup_env();

    let other_admin = Address::generate(&env);
    let result = client.try_initialize(&other_admin);
    assert!(result.is_err());
}

#[test]
fn test_set_price_success() {
    let (_env, client, _admin, asset) = setup_env();

    client.set_price(&asset, &1000_i128, &100_000_u64);

    let price_data = client.get_price(&asset).unwrap();
    assert_eq!(price_data.price, 1000);
    assert_eq!(price_data.timestamp, 100_000);
}

#[test]
fn test_set_price_updates_existing_price() {
    let (_env, client, _admin, asset) = setup_env();

    client.set_price(&asset, &1000_i128, &100_000_u64);
    client.set_price(&asset, &2000_i128, &200_000_u64);

    let price_data = client.get_price(&asset).unwrap();
    assert_eq!(price_data.price, 2000);
    assert_eq!(price_data.timestamp, 200_000);
}

#[test]
fn test_set_price_zero_price_fails() {
    let (_env, client, _admin, asset) = setup_env();

    let result = client.try_set_price(&asset, &0_i128, &100_000_u64);
    assert!(result.is_err());
}

#[test]
fn test_set_price_negative_price_fails() {
    let (_env, client, _admin, asset) = setup_env();

    let result = client.try_set_price(&asset, &(-100_i128), &100_000_u64);
    assert!(result.is_err());
}

#[test]
fn test_set_price_zero_timestamp_fails() {
    let (_env, client, _admin, asset) = setup_env();

    let result = client.try_set_price(&asset, &1000_i128, &0_u64);
    assert!(result.is_err());
}

#[test]
fn test_set_price_emits_event() {
    let (env, client, _admin, asset) = setup_env();

    client.set_price(&asset, &1000_i128, &100_000_u64);

    let last_event = env.events().all().last().unwrap();
    assert_eq!(last_event.0, client.address);

    let event_symbol = Symbol::try_from_val(&env, &last_event.1.get(0).unwrap()).unwrap();
    assert_eq!(event_symbol, Symbol::new(&env, "price_updated"));
}

#[test]
fn test_get_price_returns_none_for_unset_asset() {
    let (_env, client, _admin, _asset) = setup_env();

    let unknown_asset = Address::generate(&_env);
    let result = client.get_price(&unknown_asset);
    assert!(result.is_none());
}

#[test]
fn test_set_price_different_assets() {
    let (_env, client, _admin, asset1) = setup_env();

    let asset2 = Address::generate(&_env);

    client.set_price(&asset1, &1000_i128, &100_000_u64);
    client.set_price(&asset2, &2000_i128, &200_000_u64);

    let price1 = client.get_price(&asset1).unwrap();
    assert_eq!(price1.price, 1000);

    let price2 = client.get_price(&asset2).unwrap();
    assert_eq!(price2.price, 2000);
}
