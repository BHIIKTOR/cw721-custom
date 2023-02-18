use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128, Timestamp, Addr};

// sends funds to this wallet
#[cw_serde]
pub struct Wallet {
  pub name: String,
  pub wallet: Addr,
}

impl Default for Wallet {
    fn default() -> Self {
        Self {
            name: String::from(""),
            wallet: Addr::unchecked(String::from("")),
        }
    }
}

// accepted denom and cost of the minting
#[cw_serde]
pub struct Costs {
  pub denom: String,
  pub amount: Uint128,
}

impl Default for Costs {
    fn default() -> Self {
        Self {
          denom: String::from("ujuno"),
          amount: Uint128::from(1000000u64)
        }
    }
}

// Start and end dates of the minting, both are optional
// if there is a start it will accept mints in that date
// if there is no end it's endless
#[cw_serde]
#[derive(Default)]
pub struct Dates {
  pub start: Option<Timestamp>,
  pub end: Option<Timestamp>,
}

// impl Default for Dates {
//     fn default() -> Self {
//         Self { start: None, end: None }
//     }
// }

// Who can burn the tokens
#[cw_serde]
#[derive(Default)]
pub struct Burn {
  // peoplo who whold the tokens
  pub owners: bool,

  // the admin of the contract
  pub admin: Option<String>,

  // admin can burn owned
  pub can_burn_owned: bool,

  // allow admin and owners to burn
  pub both_can_burn: Option<bool>
}

// impl Default for Burn {
//   fn default() -> Self {
//       Self {
//         owners: false,
//         admin: None,
//         can_burn_owned: false
//       }
//   }
// }