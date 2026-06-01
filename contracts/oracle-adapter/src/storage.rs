use soroban_sdk::{contracttype, Address, Env};

use crate::PriceData;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DataKey {
    Admin,
    Price(Address),
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&DataKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&DataKey::Admin, admin);
}

pub fn get_price(env: &Env, asset: &Address) -> Option<PriceData> {
    env.storage()
        .persistent()
        .get(&DataKey::Price(asset.clone()))
}

pub fn set_price(env: &Env, asset: &Address, price_data: &PriceData) {
    env.storage()
        .persistent()
        .set(&DataKey::Price(asset.clone()), price_data);
}
