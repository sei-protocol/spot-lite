use cosmwasm_std::Decimal;

use crate::state::{Order, OrderPlacement, OrderType, PositionDirection, PositionEffect};

#[test]
fn test_order_placement_to_order() {
    let order_placement = OrderPlacement {
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
    };
    let expected_order = Order {
        id: 0,
        account: "test".to_owned(),
        price_denom: "usei".to_owned(),
        asset_denom: "uatom".to_owned(),
        price: Decimal::one(),
        quantity: Decimal::one(),
        remaining_quantity: Decimal::one(),
        direction: PositionDirection::Long,
        effect: PositionEffect::Open,
        order_type: OrderType::Limit,
    };
    assert_eq!(order_placement.to_order().unwrap(), expected_order);
}
