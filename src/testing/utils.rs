use crate::state::{OrderPlacement, OrderType, PositionDirection, SettlementEntry};
use cosmwasm_std::Decimal;

pub fn vanilla_order_placement() -> OrderPlacement {
    OrderPlacement {
        id: 0,
        status: 0,
        account: "test".to_owned(),
        contract_address: "test".to_owned(),
        price_denom: "usei".to_owned(),
        asset_denom: "uatom".to_owned(),
        price: Decimal::one(),
        quantity: Decimal::one(),
        order_type: 0,
        position_direction: 0,
        data: "{\"position_effect\":\"Open\"}".to_owned(),
    }
}

pub fn vanilla_settlement_entry() -> SettlementEntry {
    SettlementEntry {
        account: "test".to_owned(),
        price_denom: "usei".to_owned(),
        asset_denom: "uatom".to_owned(),
        quantity: Decimal::one(),
        execution_cost_or_proceed: Decimal::one(),
        expected_cost_or_proceed: Decimal::one(),
        position_direction: PositionDirection::Long,
        order_type: OrderType::Limit,
        order_id: 0,
    }
}
