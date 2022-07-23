use cosmwasm_std::Decimal;

use crate::{
    state::{Order, OrderType, PositionDirection, PositionEffect},
    testing::utils::vanilla_order_placement,
};

#[test]
fn test_order_placement_to_order() {
    let order_placement = vanilla_order_placement();
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
