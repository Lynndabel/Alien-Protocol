#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

mod events;
mod storage;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PriceData {
    pub price: i128,
    pub timestamp: u64,
}

#[contract]
pub struct OracleContract;

#[contractimpl]
impl OracleContract {
    pub fn initialize(env: Env, admin: Address) {
        if storage::get_admin(&env).is_some() {
            panic!("already initialized");
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
    }

    pub fn get_price(env: Env, asset: Address) -> Option<PriceData> {
        storage::get_price(&env, &asset)
    }

    pub fn set_price(env: Env, asset: Address, price: i128, timestamp: u64) {
        let admin = storage::get_admin(&env).expect("not initialized");
        admin.require_auth();

        assert!(price > 0, "price must be positive");
        assert!(timestamp > 0, "timestamp must be positive");

        let price_data = PriceData { price, timestamp };
        storage::set_price(&env, &asset, &price_data);

        events::PriceUpdated {
            asset,
            price,
            timestamp,
        }
        .publish(&env);
    }
}

mod test;
