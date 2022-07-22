use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response, StdError,
};
use cw2::set_contract_version;
use sei_cosmwasm::{SeiQueryWrapper};
use semver::{Version, Error as SemErr};
use crate::error::{ContractError};
use crate::msg::{InstantiateMsg, MigrateMsg, SudoMsg, OrderPlacement, DepositInfo, SettlementEntry};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:clearing-house";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// NOTE: New migrations may need store migrations if store changes are being made
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut<SeiQueryWrapper>, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }

    let storage_version: Version = ver.version.parse().map_err(|err: SemErr| ContractError::SemVer(err.to_string()))?;
    let version: Version = CONTRACT_VERSION.parse().map_err(|err: SemErr| ContractError::SemVer(err.to_string()))?;
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
pub fn sudo(deps: DepsMut<SeiQueryWrapper>, _: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::Settlement { entries, .. } => process_settlements(deps, entries),
        SudoMsg::BulkOrderPlacements { orders, deposits } => {
            process_bulk_order_placements(deps, orders, deposits)
        }
        SudoMsg::BulkOrderCancellations { ids } => {
            process_bulk_order_cancellations(deps, ids)
        }
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
    _: DepsMut<SeiQueryWrapper>,
    _: Vec<OrderPlacement>,
    _: Vec<DepositInfo>,
) -> Result<Response, ContractError> {
    // TODO
    Ok(Response::default())
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
