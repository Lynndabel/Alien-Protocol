use soroban_sdk::{contractevent, Address};

#[contractevent]
#[derive(Clone, Debug, PartialEq)]
pub struct PriceUpdated {
    pub asset: Address,
    pub price: i128,
    pub timestamp: u64,
}
