#[cfg(test)]
pub mod tests_helpers {
  use cosmwasm_std::{
    Uint128,
    Timestamp,
  };

  use cw721_base::MintMsg;

  use crate::msg::{
    InstantiateMsg,
    BatchStoreMsg,
  };

  use crate::state::Extension;

  const ADMIN: &str = "admin";
  // const MINTER: &str = "minter";

  const FUNDWALLET: &str = "wallet";

  const DENOM: &str = "uluna";
  const SUPPLY: u128 = 20u128;
  const COST: u128 = 4000000u128;

  pub fn get_mint_msg (
    id: String
  ) -> MintMsg<Extension> {
      MintMsg {
          token_id: id,
          owner: ADMIN.to_string(),
          token_uri: None,
          extension: None,
      }
  }

  pub fn get_init_msg (
      star_mint: u64,
      end_mint: u64,
  ) -> InstantiateMsg {
      InstantiateMsg {
          name: "nftt".to_string(),
          symbol: "NFTT".to_string(),
          minter: ADMIN.to_string(),
          funds_wallet: FUNDWALLET.to_string(),
          token_supply: Uint128::from(SUPPLY),
          cost_denom: DENOM.to_string(),
          cost_amount: Uint128::from(COST),
          max_mint_batch: None,
          owners_can_burn: false,
          minter_can_burn: true,
          start_mint: Some(Timestamp::from_seconds(star_mint)),
          end_mint: Some(Timestamp::from_seconds(0).plus_seconds(end_mint)),
          store_conf: None,
      }
  }

  pub fn get_store_batch_msg () -> BatchStoreMsg {
      let mut batch: Vec<MintMsg<Extension>> = vec![];
      let num: usize = 20;

      for elem in 0..num {
          batch.push(get_mint_msg(format!("{}", elem)));
      }

      BatchStoreMsg {
          batch
      }
  }
}