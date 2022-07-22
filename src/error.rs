use cosmwasm_std::{StdError, Decimal};
use thiserror::Error;
use semver::Error as SemError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Semver parsing error: {0}")]
    SemVer(String),
}