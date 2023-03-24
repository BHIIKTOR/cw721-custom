#[cfg(test)]
pub mod tests_helpers {

  use cosmwasm_std::{
    Uint128,
    Timestamp, Addr,
  };

  use cw721_base::MintMsg;

  use crate::{
    types_mint,
    msg::{
      InstantiateMsg,
      BatchStoreMsg,
    }
  };

  use crate::state::Extension;

  const ADMIN: &str = "admin";

  const FUNDWALLET: &str = "wallet";

  const DENOM: &str = "ujuno";
  const SUPPLY: u128 = 20u128;
  const COST: u128 = 4000000u128;

  pub fn now() -> Timestamp {
    Timestamp::from_seconds(0)
  }

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
          name: "nft".to_string(),
          symbol: "NFT".to_string(),
          creator: String::from(ADMIN),
          wallet: types_mint::Wallet { name: "admin".to_string(), wallet: Addr::unchecked(FUNDWALLET.to_string()) },
          token_supply: Uint128::from(SUPPLY),
          max_mint_batch: None,
          cost: types_mint::Costs {
            denom: DENOM.to_string(),
            amount: Uint128::from(COST),
          },
          dates: types_mint::Dates {
            start: Some(Timestamp::from_seconds(star_mint)),
            end: Some(Timestamp::from_seconds(0).plus_seconds(end_mint))
          },
          burn: types_mint::Burn {
            owner_can_burn: true,
            can_burn_owned: false,
          },
          store_conf: Default::default(),
      }
  }

  pub fn get_store_batch_msg (
    num: usize
  ) -> BatchStoreMsg {
      let mut batch: Vec<MintMsg<Extension>> = vec![];

      for elem in 0..num {
          batch.push(get_mint_msg(format!("{}", elem)));
      }

      BatchStoreMsg {
          batch
      }
  }
}