use crate::balance::get_balance;
use crate::contract::{execute, instantiate, migrate, query, sudo};
use crate::error::ContractError;
use crate::msg::{
    BulkOrderPlacementsResponse, ExecuteMsg, GetBalanceResponse, GetOrderResponse, InstantiateMsg,
    MigrateMsg, QueryMsg, SudoMsg,
};
use crate::order::get_order;
use crate::state::{Balance, DepositInfo, OrderPlacement, PositionDirection, SettlementEntry};
use crate::testing::mock_querier::mock_dependencies;
use crate::testing::utils::vanilla_order_placement;
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{from_binary, BankMsg, Coin, CosmosMsg, Decimal, StdError};
use std::str;

use super::utils::vanilla_settlement_entry;

#[test]
fn test_query() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    instantiate(
        deps.as_mut(),
        env,
        mock_info("admin", &[]),
        InstantiateMsg {},
    )
    .unwrap();

    let buy_order_placement = vanilla_order_placement();
    // place a buy order
    let msg = SudoMsg::BulkOrderPlacements {
        orders: vec![buy_order_placement.to_owned()],
        deposits: vec![DepositInfo {
            account: "test".to_owned(),
            denom: "usei".to_owned(),
            amount: Decimal::one(),
        }],
    };
    sudo(deps.as_mut(), mock_env(), msg).unwrap();

    let balance_query = QueryMsg::GetBalance {
        account: "test".to_owned(),
        denom: "usei".to_owned(),
    };
    let balance_bin = query(deps.as_ref(), mock_env(), balance_query).unwrap();
    let resp: GetBalanceResponse = from_binary(&balance_bin).unwrap();
    assert_eq!(
        Balance {
            amount: Decimal::one(),
            withheld: Decimal::one()
        },
        resp.balance
    );

    let order_query = QueryMsg::GetOrder { id: 0 };
    let order_bin = query(deps.as_ref(), mock_env(), order_query).unwrap();
    let resp: GetOrderResponse = from_binary(&order_bin).unwrap();
    assert_eq!(buy_order_placement.to_order().unwrap(), resp.order);
}

#[test]
fn test_deposit_withdraw() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    instantiate(
        deps.as_mut(),
        env,
        mock_info("admin", &[]),
        InstantiateMsg {},
    )
    .unwrap();

    let msg = ExecuteMsg::Deposit {};
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("test", &vec![Coin::new(100, "usei")]),
        msg,
    )
    .unwrap();
    let balance = get_balance(deps.as_ref().storage, "test".to_owned(), "usei".to_owned());
    assert_eq!(balance.amount, Decimal::from_atomics(100u128, 0).unwrap());
    assert_eq!(balance.withheld, Decimal::zero());

    let msg = ExecuteMsg::Withdraw {
        coins: vec![Coin::new(150, "usei")],
    };
    match execute(deps.as_mut(), mock_env(), mock_info("test", &[]), msg) {
        Ok(_) => panic!("Withdrawing more than you have is no ok"),
        Err(_) => (),
    };
    let msg = ExecuteMsg::Withdraw {
        coins: vec![Coin::new(99, "usei")],
    };
    execute(deps.as_mut(), mock_env(), mock_info("test", &[]), msg).unwrap();
    let balance = get_balance(deps.as_ref().storage, "test".to_owned(), "usei".to_owned());
    assert_eq!(balance.amount, Decimal::one());
    assert_eq!(balance.withheld, Decimal::zero());
}

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

    let buy_order_placement = vanilla_order_placement();
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
    assert_eq!(1, res.unsuccessful_orders.len());
    assert_eq!(0, res.unsuccessful_orders[0].id);
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
    assert_eq!(0, res.unsuccessful_orders.len());
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
    assert_eq!(1, res.unsuccessful_orders.len());
    assert_eq!(1, res.unsuccessful_orders[0].id);
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
    assert_eq!(1, res.unsuccessful_orders.len());
    assert_eq!(2, res.unsuccessful_orders[0].id);
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
    assert_eq!(0, res.unsuccessful_orders.len());
    let order = get_order(deps.as_mut().storage, 2).unwrap();
    assert_eq!(2, order.id);
    let balance = get_balance(deps.as_ref().storage, "test".to_owned(), "uatom".to_owned());
    assert_eq!(Decimal::one(), balance.amount);
    assert_eq!(Decimal::one(), balance.withheld);
}

#[test]
fn test_process_settlements() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    instantiate(
        deps.as_mut(),
        env,
        mock_info("admin", &[]),
        InstantiateMsg {},
    )
    .unwrap();

    let buy_order_placement = vanilla_order_placement();
    // place a buy order
    let msg = SudoMsg::BulkOrderPlacements {
        orders: vec![buy_order_placement.to_owned()],
        deposits: vec![DepositInfo {
            account: "test".to_owned(),
            denom: "usei".to_owned(),
            amount: Decimal::one(),
        }],
    };
    sudo(deps.as_mut(), mock_env(), msg).unwrap();

    // settle buy order
    let buy_settlement = vanilla_settlement_entry();
    let msg = SudoMsg::Settlement {
        epoch: 0,
        entries: vec![buy_settlement],
    };
    let res = sudo(deps.as_mut(), mock_env(), msg).unwrap();
    assert_eq!(1, res.messages.len());
    if let CosmosMsg::Bank(bank_msg) = res.messages[0].msg.to_owned() {
        assert_eq!(
            BankMsg::Send {
                to_address: "test".to_owned(),
                amount: vec![Coin::new(1, "uatom")]
            },
            bank_msg
        );
    } else {
        panic!("Should have sent bank message");
    }
    match get_order(deps.as_ref().storage, 0) {
        Ok(_) => panic!("Order shouldn't exist"),
        Err(_) => (),
    };
    let balance = get_balance(deps.as_ref().storage, "test".to_owned(), "usei".to_owned());
    assert_eq!(Decimal::zero(), balance.amount);
    assert_eq!(Decimal::zero(), balance.withheld);

    // place a sell order
    let sell_order_placement = OrderPlacement {
        id: 1,
        position_direction: 1,
        ..vanilla_order_placement()
    };
    let msg = SudoMsg::BulkOrderPlacements {
        orders: vec![sell_order_placement.to_owned()],
        deposits: vec![DepositInfo {
            account: "test".to_owned(),
            denom: "uatom".to_owned(),
            amount: Decimal::one(),
        }],
    };
    sudo(deps.as_mut(), mock_env(), msg).unwrap();

    // settle sell order
    let sell_settlement = SettlementEntry {
        order_id: 1,
        position_direction: PositionDirection::Short,
        ..vanilla_settlement_entry()
    };
    let msg = SudoMsg::Settlement {
        epoch: 0,
        entries: vec![sell_settlement],
    };
    let res = sudo(deps.as_mut(), mock_env(), msg).unwrap();
    assert_eq!(1, res.messages.len());
    if let CosmosMsg::Bank(bank_msg) = res.messages[0].msg.to_owned() {
        assert_eq!(
            BankMsg::Send {
                to_address: "test".to_owned(),
                amount: vec![Coin::new(1, "usei")]
            },
            bank_msg
        );
    } else {
        panic!("Should have sent bank message");
    }
    match get_order(deps.as_ref().storage, 1) {
        Ok(_) => panic!("Order shouldn't exist"),
        Err(_) => (),
    };
    let balance = get_balance(deps.as_ref().storage, "test".to_owned(), "uatom".to_owned());
    assert_eq!(Decimal::zero(), balance.amount);
    assert_eq!(Decimal::zero(), balance.withheld);
}

#[test]
fn test_bulk_order_cancellations() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    instantiate(
        deps.as_mut(),
        env,
        mock_info("admin", &[]),
        InstantiateMsg {},
    )
    .unwrap();

    let buy_order_placement = vanilla_order_placement();
    // place a buy order
    let msg = SudoMsg::BulkOrderPlacements {
        orders: vec![buy_order_placement.to_owned()],
        deposits: vec![DepositInfo {
            account: "test".to_owned(),
            denom: "usei".to_owned(),
            amount: Decimal::one(),
        }],
    };
    sudo(deps.as_mut(), mock_env(), msg).unwrap();

    // cancel buy order
    let msg = SudoMsg::BulkOrderCancellations { ids: vec![0] };
    sudo(deps.as_mut(), mock_env(), msg).unwrap();
    match get_order(deps.as_ref().storage, 0) {
        Ok(_) => panic!("Order shouldn't exist"),
        Err(_) => (),
    };
    let balance = get_balance(deps.as_ref().storage, "test".to_owned(), "usei".to_owned());
    assert_eq!(Decimal::one(), balance.amount);
    assert_eq!(Decimal::zero(), balance.withheld);

    // place a sell order
    let sell_order_placement = OrderPlacement {
        id: 1,
        position_direction: 1,
        ..vanilla_order_placement()
    };
    let msg = SudoMsg::BulkOrderPlacements {
        orders: vec![sell_order_placement.to_owned()],
        deposits: vec![DepositInfo {
            account: "test".to_owned(),
            denom: "uatom".to_owned(),
            amount: Decimal::one(),
        }],
    };
    sudo(deps.as_mut(), mock_env(), msg).unwrap();

    // cancel sell order
    let msg = SudoMsg::BulkOrderCancellations { ids: vec![1] };
    sudo(deps.as_mut(), mock_env(), msg).unwrap();
    match get_order(deps.as_ref().storage, 1) {
        Ok(_) => panic!("Order shouldn't exist"),
        Err(_) => (),
    };
    let balance = get_balance(deps.as_ref().storage, "test".to_owned(), "uatom".to_owned());
    assert_eq!(Decimal::one(), balance.amount);
    assert_eq!(Decimal::zero(), balance.withheld);
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
