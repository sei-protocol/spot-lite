use cosmwasm_std::Decimal;

use crate::order::{delete_order, get_order, save_order};
use crate::state::{Order, OrderType, PositionDirection, PositionEffect};
use crate::testing::mock_querier::mock_dependencies;

const TEST_ACCOUNT: &str = "account";
const TEST_PRICE_DENOM: &str = "price";
const TEST_ASSET_DENOM: &str = "asset";

#[test]
fn test_save_get_order() {
    let mut deps = mock_dependencies(&vec![]);
    let order = Order {
        id: 0,
        account: TEST_ACCOUNT.to_owned(),
        price_denom: TEST_PRICE_DENOM.to_owned(),
        asset_denom: TEST_ASSET_DENOM.to_owned(),
        price: Decimal::from_atomics(123u128, 0).unwrap(),
        quantity: Decimal::from_atomics(456u128, 0).unwrap(),
        remaining_quantity: Decimal::from_atomics(456u128, 0).unwrap(),
        direction: PositionDirection::Long,
        effect: PositionEffect::Open,
        order_type: OrderType::Limit,
    };
    save_order(deps.as_mut().storage, &order);
    let stored_order = get_order(deps.as_ref().storage, 0).unwrap();
    assert_eq!(order, stored_order);
}

#[test]
fn test_delete_order() {
    let mut deps = mock_dependencies(&vec![]);
    let order = Order {
        id: 0,
        account: TEST_ACCOUNT.to_owned(),
        price_denom: TEST_PRICE_DENOM.to_owned(),
        asset_denom: TEST_ASSET_DENOM.to_owned(),
        price: Decimal::from_atomics(123u128, 0).unwrap(),
        quantity: Decimal::from_atomics(456u128, 0).unwrap(),
        remaining_quantity: Decimal::from_atomics(456u128, 0).unwrap(),
        direction: PositionDirection::Long,
        effect: PositionEffect::Open,
        order_type: OrderType::Limit,
    };
    save_order(deps.as_mut().storage, &order);
    delete_order(deps.as_mut().storage, 0);
    match get_order(deps.as_ref().storage, 0) {
        Ok(_) => panic!("Order should have been deleted"),
        Err(_) => (),
    };
}
