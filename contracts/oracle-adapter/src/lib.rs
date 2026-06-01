#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contractevent, contracttype, Address, Env};

#[contracterror]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum OracleError {
    NotInitialized = 1,
    AlreadyAdmin = 2,
}

#[contractevent]
#[derive(Clone, Debug, PartialEq)]
pub struct AdminChanged {
    pub old_admin: Address,
    pub new_admin: Address,
}

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
        storage::set_admin(&env, &admin);
    }

    pub fn get_price(env: Env, asset: Address) -> Option<PriceData> {
        env.storage().persistent().get(&asset)
    }

    pub fn set_price(env: Env, asset: Address, price: i128, timestamp: u64) {
        let admin = match storage::get_admin(&env) {
            Some(addr) => addr,
            None => soroban_sdk::panic_with_error!(&env, OracleError::NotInitialized),
        };
        admin.require_auth();

        let price_data = PriceData { price, timestamp };
        env.storage().persistent().set(&asset, &price_data);
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        let current_admin = match storage::get_admin(&env) {
            Some(addr) => addr,
            None => soroban_sdk::panic_with_error!(&env, OracleError::NotInitialized),
        };
        current_admin.require_auth();

        if current_admin == new_admin {
            soroban_sdk::panic_with_error!(&env, OracleError::AlreadyAdmin);
        }

        storage::set_admin(&env, &new_admin);

        AdminChanged {
            old_admin: current_admin,
            new_admin,
        }
        .publish(&env);
    }
}

mod storage;
mod test;

