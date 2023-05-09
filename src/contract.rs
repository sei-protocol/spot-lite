use crate::balance::{get_balance, save_balance};
use crate::error::ContractError;
use crate::msg::{
    BulkOrderPlacementsResponse, ExecuteMsg, GetBalanceResponse, GetOrderResponse, InstantiateMsg,
    MigrateMsg, QueryMsg, SudoMsg, UnsuccessfulOrder,
};
use crate::order::{delete_order, get_order, save_order};
use crate::state::{
    DepositInfo, LiquidationResponse, OrderPlacement, PositionDirection, SettlementEntry,
};
use crate::utils::decimal_to_u128;
use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Coin, Decimal, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult,
};
use cw2::set_contract_version;
use sei_cosmwasm::SeiQueryWrapper;
use semver::{Error as SemErr, Version};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:clearing-house";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// NOTE: New migrations may need store migrations if store changes are being made
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }

    let storage_version: Version = ver
        .version
        .parse()
        .map_err(|err: SemErr| ContractError::SemVer(err.to_string()))?;
    let version: Version = CONTRACT_VERSION
        .parse()
        .map_err(|err: SemErr| ContractError::SemVer(err.to_string()))?;
    if storage_version >= version {
        return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    }

    // set the new version
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _: MessageInfo,
    _: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[entry_point]
pub fn sudo(
    deps: DepsMut<SeiQueryWrapper>,
    _: Env,
    msg: SudoMsg,
) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::Settlement { entries, .. } => process_settlements(deps, entries),
        SudoMsg::BulkOrderPlacements { orders, deposits } => {
            process_bulk_order_placements(deps, orders, deposits)
        }
        SudoMsg::BulkOrderCancellations { ids } => process_bulk_order_cancellations(deps, ids),
        SudoMsg::Liquidation { .. } => process_bulk_liquidation(),
    }
}

fn process_settlements(
    deps: DepsMut<SeiQueryWrapper>,
    settlements: Vec<SettlementEntry>,
) -> Result<Response, ContractError> {
    let mut response: Response = Response::new();
    for settlement in settlements {
        let order_result = get_order(deps.storage, settlement.order_id);
        if let Ok(mut order) = order_result {
            if settlement.quantity > order.remaining_quantity {
                // This should never happen unless there is a bug in the contract.
                return Err(ContractError::InvalidSettlement(
                    "Quantity too large".to_owned(),
                ));
            }

            // update order state
            if settlement.quantity == order.remaining_quantity {
                delete_order(deps.storage, settlement.order_id);
            } else {
                order.remaining_quantity -= settlement.quantity;
                save_order(deps.storage, &order);
            }

            let is_buy = order.direction == PositionDirection::Long;
            let withheld_denom = if is_buy {
                order.price_denom.to_owned()
            } else {
                order.asset_denom.to_owned()
            };
            let withheld_delta = if is_buy {
                settlement.execution_cost_or_proceed * settlement.quantity
            } else {
                settlement.quantity
            };
            let proceed_denom = if is_buy {
                order.asset_denom.to_owned()
            } else {
                order.price_denom.to_owned()
            };
            let proceed_amount = if is_buy {
                settlement.quantity
            } else {
                settlement.quantity * settlement.execution_cost_or_proceed
            };

            // update balance state
            let mut withheld_balance = get_balance(
                deps.storage,
                order.account.to_owned(),
                withheld_denom.to_owned(),
            );
            if withheld_delta > withheld_balance.amount
                || withheld_delta > withheld_balance.withheld
            {
                // This should never happen unless there is a bug in the contract.
                return Err(ContractError::InvalidSettlement(
                    "Insufficient withheld balance".to_owned(),
                ));
            }
            withheld_balance.amount -= withheld_delta;
            withheld_balance.withheld -= withheld_delta;
            save_balance(
                deps.storage,
                order.account.to_owned(),
                withheld_denom.to_owned(),
                &withheld_balance,
            );

            // send Bank messages to settle the trade
            let funds_to_send = vec![Coin::new(
                decimal_to_u128(proceed_amount),
                proceed_denom.to_owned(),
            )];
            response = response.add_message(BankMsg::Send {
                to_address: order.account.to_owned(),
                amount: funds_to_send,
            });
        } else {
            deps.api.debug(&format!(
                "Order {} not found. Skipping settlement.",
                settlement.order_id
            ));
        }
    }
    Ok(response)
}

fn process_bulk_order_placements(
    deps: DepsMut<SeiQueryWrapper>,
    orders: Vec<OrderPlacement>,
    deposits: Vec<DepositInfo>,
) -> Result<Response, ContractError> {
    // panic!("process_bulk_order_placements");
    return Err(ContractError::InvalidSettlement(
        "Test Invalid Settlement".to_owned(),
    ));
    let mut unsuccessful_orders = vec![];
    for deposit in deposits {
        let mut balance = get_balance(
            deps.storage,
            deposit.account.to_owned(),
            deposit.denom.to_owned(),
        );
        balance.amount += deposit.amount;
        save_balance(
            deps.storage,
            deposit.account.to_owned(),
            deposit.denom.to_owned(),
            &balance,
        );
    }
    for order_placement in orders {
        // There will always be exactly one pair between two assets. For example, between
        // SEI and ATOM, there will only be SEI/ATOM, in which case SEI is the price denom
        // and ATOM is the asset denom. `PositionDirection::Long` on SEI/ATOM is equivalent
        // to buying ATOM with SEI, whereas `PositionDirection::Short` on SEI/ATOM is
        // equivalent to selling ATOM into SEI (or, buying SEI with ATOM).
        let order = order_placement.to_order()?;
        let denom = if order.direction == PositionDirection::Long {
            order.price_denom.to_owned()
        } else {
            order.asset_denom.to_owned()
        };
        let nominal = if order.direction == PositionDirection::Long {
            order.price * order.quantity
        } else {
            order.quantity
        };
        let mut balance = get_balance(deps.storage, order.account.to_owned(), denom.to_owned());
        let usage_balance = balance.amount - balance.withheld;
        if usage_balance < nominal {
            deps.api.debug(&format!(
                "Insufficient usage balance of {} for {} to place order {}",
                usage_balance,
                order.price_denom.to_owned(),
                order.id
            ));
            unsuccessful_orders.push(UnsuccessfulOrder {
                id: order.id,
                reason: "Insufficient balance".to_owned(),
            });
            continue;
        }
        balance.withheld += nominal;
        save_balance(
            deps.storage,
            order.account.to_owned(),
            denom.to_owned(),
            &balance,
        );
        save_order(deps.storage, &order);
    }

    let response = BulkOrderPlacementsResponse {
        unsuccessful_orders: unsuccessful_orders,
    };
    let serialized_json = match serde_json::to_string(&response) {
        Ok(val) => val,
        Err(error) => panic!("Problem parsing response: {:?}", error),
    };
    let base64_json_str = base64::encode(serialized_json);
    let binary = match Binary::from_base64(base64_json_str.as_ref()) {
        Ok(val) => val,
        Err(error) => panic!("Problem converting binary for order request: {:?}", error),
    };

    let mut response: Response = Response::new();
    response = response.set_data(binary);
    Ok(response)
}

fn process_bulk_order_cancellations(
    deps: DepsMut<SeiQueryWrapper>,
    ids_to_cancel: Vec<u64>,
) -> Result<Response, ContractError> {
    for id in ids_to_cancel {
        let order_result = get_order(deps.storage, id);
        if let Ok(order) = order_result {
            let withheld_denom = if order.direction == PositionDirection::Long {
                order.price_denom.to_owned()
            } else {
                order.asset_denom.to_owned()
            };
            let withheld_delta = if order.direction == PositionDirection::Long {
                order.remaining_quantity * order.price
            } else {
                order.remaining_quantity
            };
            let mut withheld_balance = get_balance(
                deps.storage,
                order.account.to_owned(),
                withheld_denom.to_owned(),
            );
            withheld_balance.withheld -= withheld_delta;
            save_balance(
                deps.storage,
                order.account.to_owned(),
                withheld_denom.to_owned(),
                &withheld_balance,
            );

            delete_order(deps.storage, id);
        } else {
            deps.api
                .debug(&format!("Attempting to cancel non-existent order {}", id));
        }
    }
    Ok(Response::default())
}

fn process_bulk_liquidation() -> Result<Response, ContractError> {
    // spot market doesn't need liquidation for now since it doesn't support short selling or margin
    let response = LiquidationResponse {
        successful_accounts: vec![],
        liquidation_orders: vec![],
    };
    let serialized_json = serde_json::to_string(&response).unwrap();
    let base64_json_str = base64::encode(serialized_json);
    let binary = Binary::from_base64(base64_json_str.as_ref()).unwrap();

    let mut response: Response = Response::new();
    response = response.set_data(binary);
    Ok(response)
}

#[entry_point]
pub fn query(deps: Deps<SeiQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance { account, denom } => get_balance_query(deps, account, denom),
        QueryMsg::GetOrder { id } => get_order_query(deps, id),
    }
}

fn get_balance_query(
    deps: Deps<SeiQueryWrapper>,
    account: String,
    denom: String,
) -> StdResult<Binary> {
    let balance = get_balance(deps.storage, account.to_owned(), denom.to_owned());
    let resp = GetBalanceResponse { balance: balance };
    to_binary(&resp)
}

fn get_order_query(deps: Deps<SeiQueryWrapper>, id: u64) -> StdResult<Binary> {
    let order = get_order(deps.storage, id)?;
    let resp = GetOrderResponse { order: order };
    to_binary(&resp)
}

#[entry_point]
pub fn execute(
    deps: DepsMut<SeiQueryWrapper>,
    _: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {} => deposit(deps, info),
        ExecuteMsg::Panic {} => panic!("Panic Execute Msg"),
        ExecuteMsg::SettlementError {} => Err(ContractError::InvalidSettlement("Quantity too large".to_owned())),
        ExecuteMsg::ErrorContract {} => Err(StdError::generic_err("Error Contract Execute Msg").into()),
        ExecuteMsg::Withdraw { coins } => withdraw(deps, info, coins),
    }
}

fn deposit(deps: DepsMut<SeiQueryWrapper>, info: MessageInfo) -> Result<Response, ContractError> {
    let account = info.sender.into_string();
    for coin in info.funds {
        let mut balance = get_balance(deps.storage, account.to_owned(), coin.denom.to_owned());
        balance.amount += Decimal::from_atomics(coin.amount, 0).unwrap();
        save_balance(
            deps.storage,
            account.to_owned(),
            coin.denom.to_owned(),
            &balance,
        )
    }
    Ok(Response::default())
}

fn withdraw(
    deps: DepsMut<SeiQueryWrapper>,
    info: MessageInfo,
    coins: Vec<Coin>,
) -> Result<Response, ContractError> {
    let account = info.sender.into_string();
    for coin in coins.to_owned() {
        let mut balance = get_balance(deps.storage, account.to_owned(), coin.denom.to_owned());
        let amount = Decimal::from_atomics(coin.amount, 0).unwrap();
        if balance.amount - balance.withheld < amount {
            return Err(ContractError::InsufficientFund());
        }
        balance.amount -= amount;
        save_balance(
            deps.storage,
            account.to_owned(),
            coin.denom.to_owned(),
            &balance,
        )
    }
    let response = Response::new().add_message(BankMsg::Send {
        to_address: account.to_owned(),
        amount: coins,
    });
    Ok(response)
}
