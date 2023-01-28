use cosmwasm_std::{
  DepsMut,
  MessageInfo,
  Coin,
  Uint128,
  Storage,
  Addr,
  Timestamp,
  Env,
  BlockInfo, StdError
};

use cw721_base::{
  state::TokenInfo,
  MintMsg
};

use crate::{
  error::ContractError,
  state::{
    CW721Contract,
    Extension,
    CONFIG,
    Config,
    BURNT_AMOUNT,
    BURNT_LIST,
    BURNED,
    Metadata
  }
};

// Check if sender is the configured contract owner or minter so they can update data
pub fn can_update(
  deps: &DepsMut,
  info: &MessageInfo
) -> Result<(), ContractError> {
  let cw721_contract = CW721Contract::default();

  let minter = cw721_contract.minter.load(deps.storage)?;

  if info.sender != minter {
      return Err(ContractError::Unauthorized {});
  }

  Ok(())
}

// Update the amount of burnt tokens by a given address
pub fn update_burnt_amount(
  storage: &mut dyn Storage,
  sender: &Addr,
) -> Result<(), ContractError> {
  match BURNT_AMOUNT.load(storage, sender) {
    Ok(mut amount) => {
        amount += Uint128::from(1u32);
        BURNT_AMOUNT.save(storage, sender, &amount)?;
        Ok(())
    },
    Err(_) => {
        BURNT_AMOUNT.save(storage, sender, &Uint128::from(1u32))?;
        Ok(())
    }
  }
}

// Update list of burnt tokens by given address
pub fn update_burnt_list(
  storage: &mut dyn Storage,
  sender: &Addr,
  token: &str,
) -> Result<(), ContractError> {
  match BURNT_LIST.load(storage, sender) {
    Ok(mut list) => {
        list.push(String::from(token));
        BURNT_LIST.save(storage, sender, &list)?;
        Ok(())
    },
    Err(_) => {
        BURNT_LIST.save(storage, sender, &vec![String::from(token)])?;
        Ok(())
    }
  }
}

pub fn check_token_exists_or_err(
  contract: &CW721Contract,
  storage: &mut dyn Storage,
  token_id: &String
) -> Result<(), ContractError> {
  if !contract.tokens.has(storage, token_id) {
    return Err(ContractError::TokenNotFound { token_id: token_id.to_string() })
  }
  Ok(())
}

// burn a token
pub fn burn_token(
  contract: &CW721Contract,
  storage: &mut dyn Storage,
  token_id: &String,
  sender: &Addr,
  check_owner: bool
) -> Result<(), ContractError> {
  check_token_exists_or_err(contract, storage, token_id)?;

  if check_owner {
    check_token_ownership_basic(contract, storage, sender, token_id)?;
  }

  contract.tokens.remove(storage, token_id)?;

  contract.decrement_tokens(storage)?;

  BURNED.save(storage, token_id.clone(), &true)?;

  Ok(())
}

// burn a token and update totals
pub fn burn_and_update(
  contract: &CW721Contract,
  storage: &mut dyn Storage,
  token_id: &String,
  sender: &Addr,
  _block: &BlockInfo,
  check_owner: bool
) -> Result<(), ContractError> {
  burn_token(contract, storage, token_id, sender, check_owner)?;

  update_burnt_amount(storage, sender)?;

  // add logs for whom of the owners burnt tokens
  if check_owner {
    update_burnt_list(storage, sender, token_id)?;
  }

  Ok(())
}

// Check if we can store tokens meta-data
pub fn can_store(
  deps: &DepsMut,
  info: &MessageInfo
) -> Result<(), ContractError> {
  // call can update to verify rights
  can_update(deps, info)?;

  let config = CONFIG.load(deps.storage)?;

  // check if contract has been frozen
  if config.frozen {
    return Err(ContractError::ContractFrozen{})
  }

  // check if token total is not above token supply
  if config.token_total >= config.token_supply {
      return Err(ContractError::MaxTokenSupply {});
  }

  Ok(())
}

// Check if
// sender can pay for the token(s)
// correct number of denoms
// correct amount is sent
pub fn can_pay(
  config: &Config,
  info: &MessageInfo,
  amount: &Uint128
) -> Result<Coin, ContractError> {
  let mut coin_found: Coin = Coin::new(0, "none");

  if info.funds.len() > 1 {
    return Err(ContractError::TooManyDenoms {})
  }

  if let Some(coin) = info.funds.first() {
      if coin.denom != config.cost_denom {
        Err(ContractError::WrongToken {})
      } else {
          let total = config.cost_amount * amount;

          if coin.amount < total {
              return Err(ContractError::NotEnoughFunds {})
          }

          if coin.amount != total {
            return Err(ContractError::IncorrectFunds {})
          }

          coin_found.denom = coin.denom.clone();
          coin_found.amount = coin.amount;

          Ok(coin_found)
      }
  } else {
    Err(ContractError::NoFundsSent {})
  }
}

// Check if all conditions are meet and sender can mint
pub fn can_mint(
  count: &u64,
  time: &Timestamp,
  config: &Config,
  mint_amount: &Uint128,
  minter: &Addr,
  sender: &Addr
) -> Result<Uint128, ContractError> {
  // check if contract is frozen
  if config.frozen {
    return Err(ContractError::ContractFrozen{})
  }

  // check if contract has been paused
  if config.paused {
    return Err(ContractError::ContractPaused{})
  }

  // check if contract contain token data
  if config.token_total == Uint128::from(0u32) {
      return Err(ContractError::CantMintNothing {});
  }

  // check if mint amount is zero
  if mint_amount.is_zero() {
    return Err(ContractError::MintZero {  })
  }

  // validate max mint amount
  if let Some(max_size) = &config.max_mint_batch {
    if mint_amount > max_size {
      return Err(ContractError::MintAmountLargerThanAllowed {  })
    }
  }

  // check if start_mint date is some and if is correct
  if let Some(stamp) = &config.start_mint {
    if time < stamp {
      return Err(ContractError::CantMintYet {})
    }
  }

  // check if end_mint date is some and if it has pass over the limit
  if let Some(stamp) = &config.end_mint {
    if time > stamp {
      return Err(ContractError::MintEnded {})
    }
  }

  let current_count = Uint128::from(*count);

  // we have hit current token supply
  if current_count == config.token_supply {
    return Err(ContractError::MaxTokenSupply {});
  }

  // we have hit max total tokens in the collection
  if current_count == config.token_total {
      return Err(ContractError::MaxTokens {});
  }

  // dont allow contract admin to become owner of tokens
  if sender == minter {
      return Err(ContractError::Unauthorized {})
  }

  Ok(current_count)
}

pub fn check_token_ownership_basic(
  contract: &CW721Contract,
  storage: &mut dyn Storage,
  sender: &Addr,
  token_id: &String,
) -> Result<TokenInfo<Option<Metadata>>, ContractError> {
  let token = contract.tokens.load(storage, token_id)?;

  // owner can send
  if token.owner == sender.clone() {
    Ok(token)
  } else {
    Err(ContractError::Unauthorized {})
  }
}

pub fn check_token_ownership_approvals(
  token: &TokenInfo<Option<Metadata>>,
  sender: &Addr,
  block: &BlockInfo,
) -> Result<(),ContractError> {
  if token
    .approvals
    .iter()
    .any(|apr| apr.spender == sender.clone() && !apr.is_expired(&block))
  {
    Ok(())
  } else {
    Err(ContractError::Unauthorized {})
  }
}

pub fn check_token_ownership_operators(
  contract: &CW721Contract,
  storage: &mut dyn Storage,
  owner: &Addr,
  sender: &Addr,
  block: &BlockInfo,
) -> Result<(), StdError> {
  let op = contract
    .operators
    .may_load(storage, (&owner, &sender));

  if op.is_err() {
    return Err(op.unwrap_err())
  }

  if op.is_ok() {
    let ex = op.unwrap().unwrap();
    if ex.is_expired(&block) {
      return Err(StdError::GenericErr { msg: "expired block".to_string() })
    }
  }

  Ok(())
}

pub fn check_token_ownership_complete(
  contract: &CW721Contract,
  storage: &mut dyn Storage,
  block: &BlockInfo,
  sender: &Addr,
  token_id: &String,
) -> Result<TokenInfo<Option<Metadata>>, ContractError> {
  let check_basic = check_token_ownership_basic(&contract, storage, &sender, &token_id);

  // owner can send
  if check_basic.is_ok() {
    return Ok(check_basic.unwrap())
  }

  let token = check_basic.unwrap();
  // any non-expired token approval can send
  if check_token_ownership_approvals(&token, &sender, &block).is_ok() {
    return Ok(token)
  }

  // operator can send
  if check_token_ownership_operators(&contract, storage, &token.owner, sender, block).is_ok() {
    return Ok(token)
  }

  Err(ContractError::Unauthorized {  })
}

pub fn transfer_nft(
  storage: &mut dyn Storage,
  env: &Env,
  contract: &CW721Contract,
  info: &MessageInfo,
  recipient: &Addr,
  token_id: &String,
) -> Result<String, ContractError> {
  check_token_exists_or_err(contract, storage, &token_id)?;

  // ensure we have permissions
  let mut token = check_token_ownership_complete(contract, storage, &env.block, &info.sender, token_id)?;

  // set owner and remove existing approvals
  token.owner = recipient.clone();
  token.approvals = vec![];

  contract.tokens.save(storage, token_id, &token)?;

  Ok(token_id.to_string())
}

// Update total real tokens in the collection
pub fn update_total(
  storage: &mut dyn Storage,
  amount: &Uint128
) -> Result<Uint128, ContractError> {
  let mut config = CONFIG.load(storage)?;

  let total = config.token_total.checked_add(amount.clone());

  if total.is_err() {
    return Err(ContractError::CantUpdateTotal {})
  }

  config.token_total = total.unwrap();
  CONFIG.save(storage, &config)?;

  Ok(config.token_total)
}

// Attempt to store a token's meta-data
pub fn try_store(
  storage: &mut dyn Storage,
  nft_data: &MintMsg<Extension>,
  minter: &Addr,
  contract: &CW721Contract,
) -> Result<(), ContractError> {
  let token_id = nft_data.token_id.clone();

  // create the token
  let token = TokenInfo {
      owner: minter.clone(),
      approvals: vec![],
      token_uri: nft_data.token_uri.clone(),
      extension: nft_data.extension.clone(),
  };

  contract.tokens.save(storage, &token_id, &token)?;

  Ok(())
}

// Attempt to mint a token
pub fn try_mint(
  storage: &mut dyn Storage,
  sender: &Addr,
  minter: &Addr,
  contract: &CW721Contract,
  token_id: &String
) -> Result<(), ContractError> {
  check_token_exists_or_err(contract, storage, token_id)?;

  let old_token = contract.tokens.load(storage, token_id)?;

  if old_token.owner != minter.clone() {
    return Err(ContractError::Claimed {})
  }

  let mut new_token = old_token.clone();
  new_token.owner = sender.clone();
  contract.tokens.replace(storage, token_id, Some(&new_token), Some(&old_token))?;
  contract.increment_tokens(storage)?;

  Ok(())
}
