use crate::state::Balance;
use cosmwasm_std::{Decimal, Storage};
use cw_storage_plus::Map;

// addr, denom -> balance
pub const BALANCE: Map<(String, String), Balance> = Map::new("bal");

pub fn save_balance(storage: &mut dyn Storage, account: String, symbol: String, balance: &Balance) {
    BALANCE
        .save(storage, (account.to_owned(), symbol.to_owned()), balance)
        .unwrap();
}

pub fn get_balance(storage: &dyn Storage, account: String, symbol: String) -> Balance {
    match BALANCE.may_load(storage, (account.to_owned(), symbol)) {
        Ok(balance_opt) => match balance_opt {
            Some(balance) => balance,
            None => Balance {
                amount: Decimal::zero(),
                withheld: Decimal::zero(),
            },
        },
        Err(error) => panic!("Problem parsing balance: {:?}", error),
    }
}
