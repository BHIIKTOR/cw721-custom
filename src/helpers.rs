use std::time::{
  SystemTime,
  UNIX_EPOCH
};

use cosmwasm_std::{
  DepsMut,
  MessageInfo,
  Coin,
  Uint128,
  Storage,
  Addr,
  Timestamp
};

use cw721_base::{ MintMsg };
use cw721_base::state::{ TokenInfo };

// use crate::msg::StoreConf;

use crate::state::{
  CW721Contract,
  Extension,
  CONFIG,
  Config,
  BURNT_AMOUNT,
  BURNT_LIST,
  BURNED
};

use crate::error::ContractError;

pub fn _now() -> Timestamp {
  Timestamp::from_seconds(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
}

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

pub fn burn_token(
  contract: &CW721Contract,
  storage: &mut dyn Storage,
  token_id: String
) -> Result<(), ContractError> {
  contract.tokens.remove(storage, &token_id)?;
  contract.decrement_tokens(storage)?;
  BURNED.save(storage, token_id, &true)?;
  Ok(())
}

pub fn burn_and_update(
  contract: &CW721Contract,
  storage: &mut dyn Storage,
  token_id: String,
  sender: Addr,
  update_list: bool
) -> Result<(), ContractError> {
  burn_token(contract, storage, token_id.clone())?;
  update_burnt_amount(storage, &sender)?;
  if update_list {
    update_burnt_list(storage, &sender, &token_id)?;
  }
  Ok(())
}

pub fn can_store(
  deps: &DepsMut,
  info: &MessageInfo
) -> Result<(), ContractError> {
  can_update(deps, info)?;

  let config = CONFIG.load(deps.storage)?;
  if config.token_total >= config.token_supply {
      return Err(ContractError::MaxTokenSupply {});
  }

  Ok(())
}

pub fn can_pay(
  config: &Config,
  info: &MessageInfo,
  amount: Uint128
) -> Result<Coin, ContractError> {
  let mut coin_found: Coin = Coin::new(0, "none");

  if let Some(coin) = info.funds.first() {
      if coin.denom != config.cost_denom {
        Err(ContractError::WrongToken {})
      } else {
          let total = config.cost_amount * amount;
          println!("NotEnoughFunds: {} {} {}", coin.amount < total, coin.amount, total);
          if coin.amount < total {
              return Err(ContractError::NotEnoughFunds {});
          }

          if coin.amount == total {
              coin_found.denom = coin.denom.clone();
              coin_found.amount = coin.amount;

              Ok(coin_found)
          } else {
            Err(ContractError::IncorrectFunds {})
          }
      }
  } else {
    Err(ContractError::NoFundsSent {})
  }
}

pub fn can_mint(
  count: &u64,
  time: &Timestamp,
  start_mint: &Option<Timestamp>,
  token_total: Uint128,
  token_supply: Uint128,
  minter: &Addr,
  sender: &Addr
) -> Result<Uint128, ContractError> {
  if token_total == Uint128::from(0u32) {
      return Err(ContractError::CantMintNothing {});
  }

  if let Some(stamp) = start_mint {
    if time < stamp {
      return Err(ContractError::CantMintYet {})
    }
  }

  let current_count = Uint128::from(*count);

  if current_count == token_supply {
    return Err(ContractError::MaxTokenSupply {});
  }

  if current_count == token_total {
      return Err(ContractError::MaxTokens {});
  }

  // dont allow contract minter to become owner of tokens
  if sender == minter {
      return Err(ContractError::Unauthorized {})
  }

  Ok(current_count)
}

pub fn update_total(
  storage: &mut dyn Storage,
  amount: Uint128
) -> Result<(), ContractError> {
  let mut config = CONFIG.load(storage)?;
  config.token_total += amount;
  CONFIG.save(storage, &config)?;
  Ok(())
}

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

pub fn try_mint(
  storage: &mut dyn Storage,
  sender: &Addr,
  minter: &Addr,
  contract: &CW721Contract,
  current: &String
) -> Result<(), ContractError> {
  let old_token = contract.tokens.load(storage, current)?;
  if old_token.owner != minter.clone() {
    return Err(ContractError::Claimed {})
  }
  let mut new_token = old_token.clone();
  new_token.owner = sender.clone();
  contract.tokens.replace(storage, current, Some(&new_token), Some(&old_token))?;
  contract.increment_tokens(storage)?;
  Ok(())
}
