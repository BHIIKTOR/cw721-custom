use cosmwasm_schema::cw_serde;
// use crate::error::ContractError;
use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Addr, Empty, Uint128};

pub type Extension = Option<Metadata>;
pub type CW721Contract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;

pub const CONFIG: Item<Config> = Item::new("conf");

pub const BURNT_AMOUNT: Map<&Addr, Uint128> = Map::new("burnt");
pub const PLEDGED_TOKENS_BY_ADDR: Map<&Addr, Vec<String>> = Map::new("pba");
pub const PLEDGED_TOKENS: Map<String, bool> = Map::new("pledged");

// use cw_utils::{Expiration, Scheduled};
use crate::{
    msg::StoreConf,
    types_mint
};

#[cw_serde]
pub struct Config {
    pub creator: String,
    pub name: String,
    pub token_supply: Uint128,
    pub token_total: Uint128,
    pub cost: types_mint::Costs,
    pub dates: types_mint::Dates,
    pub max_mint_batch: Option<Uint128>,
    pub burn: types_mint::Burn,
    pub wallet: types_mint::Wallet,
    pub store_conf: StoreConf,
    pub frozen: bool,
    pub paused: bool,
}

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
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
