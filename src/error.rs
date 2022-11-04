use cosmwasm_std::StdError;
use cw721_base::ContractError as CW721ContractError;
use thiserror::Error;

/// This overrides the ContractError enum defined in cw721-base
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized to execute request")]
    Unauthorized {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },

    #[error("Token already claimed")]
    Claimed {},

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Cant mint yet")]
    CantMintYet {},

    #[error("Token not found")]
    TokenNotFound {},

    #[error("Cannot update frozen contract")]
    ContractFrozen {},

    #[error("Token supply exhausted")]
    MaxTokenSupply {},

    #[error("Token total reached")]
    MaxTokens {},

    #[error("Request too large {size}")]
    RequestTooLarge { size: usize },

    #[error("Request too small {size}")]
    RequestTooSmall { size: usize },

    //NotEnoughFunds
    #[error("Not enough funds")]
    NotEnoughFunds {},

    #[error("Amount of funds sends are not equal to the required total")]
    IncorrectFunds {},

    //NoFundsSent
    #[error("No funds sent")]
    NoFundsSent {},

    #[error("Token does not exists")]
    DontExists {},

    #[error("Token exists")]
    Exists {},

    // WrongToken
    #[error("Wrong token")]
    WrongToken {},

    // NoConfiguration
    #[error("No Configuration")]
    NoConfiguration {},

    #[error("There is nothing to mint")]
    CantMintNothing {},
}

impl From<CW721ContractError> for ContractError {
    fn from(msg: CW721ContractError) -> ContractError {
        match msg {
            CW721ContractError::ApprovalNotFound{spender} => ContractError::ApprovalNotFound{spender},
            CW721ContractError::Unauthorized {} => ContractError::Unauthorized {},
            //CW721ContractError::NonTransferable {} => ContractError::NonTransferable {},
            CW721ContractError::Claimed {} => ContractError::Claimed {},
            CW721ContractError::Expired {} => ContractError::Expired {},
            CW721ContractError::Std(e) => ContractError::Std(e),
        }
    }
}
