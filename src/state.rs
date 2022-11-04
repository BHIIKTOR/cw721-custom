use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use crate::error::ContractError;
use cw_storage_plus::{Item, Map};
// use cw_storage_plus::{Item};
// use cosmwasm_std::{Empty, Uint128, StdResult, StdError};
// use cosmwasm_std::{Empty, Uint128, Storage};
use cosmwasm_std::{Addr, Empty, Uint128, Timestamp};

pub type Extension = Option<Metadata>;
pub type CW721Contract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;

pub const CONFIG: Item<Config> = Item::new("config");

pub const BURNT_AMOUNT: Map<&Addr, Uint128> = Map::new("burnt_amount");
pub const BURNT_LIST: Map<&Addr, Vec<String>> = Map::new("burnt_list");
pub const BURNED: Map<String, bool> = Map::new("burned");

// use cw_utils::{Expiration, Scheduled};
use crate::msg::StoreConf;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub frozen: bool,
    pub token_supply: Uint128,
    pub token_total: Uint128,
    pub cost_denom: String,
    pub cost_amount: Uint128,
    pub start_mint: Option<Timestamp>,
    pub max_mint_batch: Uint128,
    pub owners_can_burn: bool,
    pub minter_can_burn: bool,
    pub funds_wallet: String,
    pub store_conf: Option<StoreConf>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}
