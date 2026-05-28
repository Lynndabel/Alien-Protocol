<<<<<<< HEAD
use soroban_sdk::{contractevent, Address};

#[contractevent]
#[derive(Clone, Debug, PartialEq)]
pub struct Deposited {
    pub user: Address,
    pub asset: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, PartialEq)]
pub struct AssetAdded {
    pub asset: Address,
}

#[contractevent]
#[derive(Clone, Debug, PartialEq)]
pub struct AssetRemoved {
    pub asset: Address,
}

#[contractevent]
#[derive(Clone, Debug, PartialEq)]
=======
use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
>>>>>>> ad858a3 (Implement set_admin Function)
pub struct AdminChanged {
    pub old_admin: Address,
    pub new_admin: Address,
}

<<<<<<< HEAD
#[contractevent]
#[derive(Clone, Debug, PartialEq)]
pub struct Paused {
    pub paused: bool,
}

#[contractevent]
#[derive(Clone, Debug, PartialEq)]
pub struct Unpaused {
    pub paused: bool,
=======
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddCollateralEvent {
    pub from: Address,
    pub amount: i128,
>>>>>>> ad858a3 (Implement set_admin Function)
}
