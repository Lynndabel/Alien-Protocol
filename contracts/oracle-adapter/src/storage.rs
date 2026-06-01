use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DataKey {
    Admin,
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&DataKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&DataKey::Admin, admin);
}
