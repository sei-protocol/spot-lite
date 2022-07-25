use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("Invalid order data")]
    InvalidOrderData(),

    #[error("Invalid settlement: {0}")]
    InvalidSettlement(String),

    #[error("Insufficient fund")]
    InsufficientFund(),
}
