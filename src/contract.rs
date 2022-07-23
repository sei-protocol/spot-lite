use crate::balance::{get_balance, save_balance};
use crate::error::ContractError;
use crate::msg::{BulkOrderPlacementsResponse, InstantiateMsg, MigrateMsg, SudoMsg};
use crate::order::save_order;
use crate::state::{DepositInfo, OrderPlacement, PositionDirection, SettlementEntry};
use cosmwasm_std::{entry_point, Binary, DepsMut, Env, MessageInfo, Response, StdError};
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
    _: DepsMut<SeiQueryWrapper>,
    _: Vec<SettlementEntry>,
) -> Result<Response, ContractError> {
    // TODO
    Ok(Response::default())
}

fn process_bulk_order_placements(
    deps: DepsMut<SeiQueryWrapper>,
    orders: Vec<OrderPlacement>,
    deposits: Vec<DepositInfo>,
) -> Result<Response, ContractError> {
    let mut unsuccessful_order_ids = vec![];
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
            unsuccessful_order_ids.push(order.id);
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
        unsuccessful_order_ids: unsuccessful_order_ids,
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
    _: DepsMut<SeiQueryWrapper>,
    _: Vec<u64>,
) -> Result<Response, ContractError> {
    // TODO
    Ok(Response::default())
}

fn process_bulk_liquidation() -> Result<Response, ContractError> {
    // spot market doesn't need liquidation for now since it doesn't support short selling or margin
    Ok(Response::default())
}
