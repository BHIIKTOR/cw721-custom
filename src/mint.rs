use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128, Timestamp, Addr};

const DEFAULT_DENOM: &str = "ujuno";
const DEFAULT_AMOUNT: u64 = 1000000u64;

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

// TODO: implement multiple denoms
// accepted denom and cost of the minting
#[cw_serde]
pub struct Costs {
  pub denom: String,
  pub amount: Uint128,
}

impl Default for Costs {
    fn default() -> Self {
        Self {
          denom: String::from(DEFAULT_DENOM),
          amount: Uint128::from(DEFAULT_AMOUNT)
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

// Who can burn the tokens
#[cw_serde]
#[derive(Default)]
pub struct Burn {
  // peoplo who hold the tokens
  pub owner_can_burn: bool,

  // creator can burn tokens owned by others
  pub can_burn_owned: bool,
}