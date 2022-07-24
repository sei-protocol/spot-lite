use crate::state::Order as OrderState;
use cosmwasm_std::Storage;

use cw_storage_plus::Map;

pub const ORDER: Map<u64, OrderState> = Map::new("order");
pub const ORDER_ACCOUNT_INDEX: Map<(String, u64), OrderState> = Map::new("oai");

pub fn save_order(storage: &mut dyn Storage, order: &OrderState) {
    ORDER.save(storage, order.id, order).unwrap();
    ORDER_ACCOUNT_INDEX
        .save(storage, (order.account.to_owned(), order.id), order)
        .unwrap();
}

pub fn get_order(storage: &dyn Storage, id: u64) -> Result<OrderState, cosmwasm_std::StdError> {
    ORDER.load(storage, id)
}

pub fn delete_order(storage: &mut dyn Storage, id: u64) {
    ORDER.remove(storage, id)
}
