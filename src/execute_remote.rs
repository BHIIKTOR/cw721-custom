use cosmwasm_std::{Env, DepsMut, MessageInfo, Response, Uint128};

use crate::{
  error::ContractError,
  helpers::can_update,
  execute::{execute_mint_batch, execute_burn_batch},
  msg::MintBatchMsg
};

pub fn execute_remote_mint_batch(
  env: Env,
  deps: DepsMut,
  info: MessageInfo,
  amount: Uint128,
  owner: String
) -> Result<Response, ContractError> {
  // ensure the msg comes from the contract owner
  can_update(&deps, &info)?;

  let o = deps.api.addr_validate(&owner)?;

  execute_mint_batch(
    env,
    deps,
    MessageInfo { sender: o, funds: info.funds },
    MintBatchMsg { amount }
  )?;

  Ok(
      Response::new()
          .add_attribute("action", "remote_mint_batch")
  )
}

pub fn execute_remote_burn_batch(
  env: Env,
  deps: DepsMut,
  info: MessageInfo,
  tokens: Vec<String>,
  owner: String
) -> Result<Response, ContractError> {
  // ensure the msg comes from the contract owner
  can_update(&deps, &info)?;

  let o = deps.api.addr_validate(&owner)?;

  execute_burn_batch(
    env,
    deps,
    MessageInfo { sender: o, funds: vec![] },
    tokens
  )?;

  Ok(
      Response::new()
          .add_attribute("action", "remote_burn_batch")
  )
}