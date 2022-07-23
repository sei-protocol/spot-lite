use crate::balance::get_balance;
use crate::contract::{instantiate, migrate, sudo};
use crate::error::ContractError;
use crate::msg::{BulkOrderPlacementsResponse, InstantiateMsg, MigrateMsg, SudoMsg};
use crate::order::get_order;
use crate::state::{DepositInfo, OrderPlacement};
use crate::testing::mock_querier::mock_dependencies;
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{Decimal, StdError};
use std::str;

#[test]
fn test_bulk_order_placements() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    instantiate(
        deps.as_mut(),
        env,
        mock_info("admin", &[]),
        InstantiateMsg {},
    )
    .unwrap();

    let buy_order_placement = OrderPlacement {
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
    // unsuccessful due to insufficient funds
    let msg = SudoMsg::BulkOrderPlacements {
        orders: vec![buy_order_placement.to_owned()],
        deposits: vec![],
    };
    let res = sudo(deps.as_mut(), mock_env(), msg).unwrap();

    let data = res.data.unwrap();
    let base64 = data.to_base64();
    let deserialized = base64::decode(base64).unwrap();
    let str_data = str::from_utf8(&deserialized).unwrap();
    let res: BulkOrderPlacementsResponse = serde_json::from_str(str_data).unwrap();
    assert_eq!(1, res.unsuccessful_order_ids.len());
    assert_eq!(0, res.unsuccessful_order_ids[0]);
    match get_order(deps.as_mut().storage, 0) {
        Ok(_) => panic!("Order shouldn't exist"),
        Err(_) => (),
    };

    // successful because of deposit
    let msg = SudoMsg::BulkOrderPlacements {
        orders: vec![buy_order_placement.to_owned()],
        deposits: vec![DepositInfo {
            account: "test".to_owned(),
            denom: "usei".to_owned(),
            amount: Decimal::one(),
        }],
    };
    let res = sudo(deps.as_mut(), mock_env(), msg).unwrap();

    let data = res.data.unwrap();
    let base64 = data.to_base64();
    let deserialized = base64::decode(base64).unwrap();
    let str_data = str::from_utf8(&deserialized).unwrap();
    let res: BulkOrderPlacementsResponse = serde_json::from_str(str_data).unwrap();
    assert_eq!(0, res.unsuccessful_order_ids.len());
    let order = get_order(deps.as_mut().storage, 0).unwrap();
    assert_eq!(0, order.id);
    let balance = get_balance(deps.as_ref().storage, "test".to_owned(), "usei".to_owned());
    assert_eq!(Decimal::one(), balance.amount);
    assert_eq!(Decimal::one(), balance.withheld);

    // unsuccessful due to withholding
    let buy_order_placement_2 = OrderPlacement {
        id: 1,
        ..buy_order_placement.to_owned()
    };
    let msg = SudoMsg::BulkOrderPlacements {
        orders: vec![buy_order_placement_2.to_owned()],
        deposits: vec![],
    };
    let res = sudo(deps.as_mut(), mock_env(), msg).unwrap();

    let data = res.data.unwrap();
    let base64 = data.to_base64();
    let deserialized = base64::decode(base64).unwrap();
    let str_data = str::from_utf8(&deserialized).unwrap();
    let res: BulkOrderPlacementsResponse = serde_json::from_str(str_data).unwrap();
    assert_eq!(1, res.unsuccessful_order_ids.len());
    assert_eq!(1, res.unsuccessful_order_ids[0]);
    match get_order(deps.as_mut().storage, 1) {
        Ok(_) => panic!("Order shouldn't exist"),
        Err(_) => (),
    };

    // unsuccessful due to wrong deposit
    let sell_order_placement = OrderPlacement {
        id: 2,
        position_direction: 1,
        ..buy_order_placement.to_owned()
    };
    let msg = SudoMsg::BulkOrderPlacements {
        orders: vec![sell_order_placement.to_owned()],
        deposits: vec![DepositInfo {
            account: "test".to_owned(),
            denom: "usei".to_owned(),
            amount: Decimal::one(),
        }],
    };
    let res = sudo(deps.as_mut(), mock_env(), msg).unwrap();

    let data = res.data.unwrap();
    let base64 = data.to_base64();
    let deserialized = base64::decode(base64).unwrap();
    let str_data = str::from_utf8(&deserialized).unwrap();
    let res: BulkOrderPlacementsResponse = serde_json::from_str(str_data).unwrap();
    assert_eq!(1, res.unsuccessful_order_ids.len());
    assert_eq!(2, res.unsuccessful_order_ids[0]);
    match get_order(deps.as_mut().storage, 2) {
        Ok(_) => panic!("Order shouldn't exist"),
        Err(_) => (),
    };

    // successful sell
    let msg = SudoMsg::BulkOrderPlacements {
        orders: vec![sell_order_placement.to_owned()],
        deposits: vec![DepositInfo {
            account: "test".to_owned(),
            denom: "uatom".to_owned(),
            amount: Decimal::one(),
        }],
    };
    let res = sudo(deps.as_mut(), mock_env(), msg).unwrap();

    let data = res.data.unwrap();
    let base64 = data.to_base64();
    let deserialized = base64::decode(base64).unwrap();
    let str_data = str::from_utf8(&deserialized).unwrap();
    let res: BulkOrderPlacementsResponse = serde_json::from_str(str_data).unwrap();
    assert_eq!(0, res.unsuccessful_order_ids.len());
    let order = get_order(deps.as_mut().storage, 2).unwrap();
    assert_eq!(2, order.id);
    let balance = get_balance(deps.as_ref().storage, "test".to_owned(), "uatom".to_owned());
    assert_eq!(Decimal::one(), balance.amount);
    assert_eq!(Decimal::one(), balance.withheld);
}

#[test]
fn test_migration() {
    let mut deps = mock_dependencies(&vec![]);
    let instantiate_msg = InstantiateMsg {};
    let info = mock_info("", &vec![]);
    instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

    // test incorrect contract name to assert error
    cw2::set_contract_version(&mut deps.storage, "this_is_the_wrong_contract", "0.0.1").unwrap();
    let res = migrate(deps.as_mut(), mock_env(), MigrateMsg {});
    match res {
        Err(ContractError::Std(x)) => {
            assert_eq!(x, StdError::generic_err("Can only upgrade from same type"))
        }
        _ => panic!("This should raise error on contract type mismatch"),
    };
}