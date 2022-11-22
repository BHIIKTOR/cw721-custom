#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg };
use crate::state::{Config, CW721Contract, CONFIG};

use cw2::set_contract_version;

pub use cw721_base::{
    MintMsg,
    MinterResponse,
    InstantiateMsg as CW721InitMsg
};

use crate::execute::{
    execute_burn,
    execute_burn_batch,
    execute_mint,
    execute_mint_batch,
    execute_store,
    execute_store_batch,
    execute_store_conf,
};

use crate::error::ContractError;

use crate::migrate_function::try_migrate;

use crate::query::{
    query_config,
    query_nft_info_batch,
    query_burnt_amount,
    query_burnt_list,
    query_burned,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw721-offchain-randomization";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        frozen: false,
        token_supply: msg.token_supply,
        token_total: Uint128::from(0u128),
        cost_denom: msg.cost_denom,
        cost_amount: msg.cost_amount,
        start_mint: Some(msg.start_mint).unwrap_or_default(),
        end_mint: Some(msg.end_mint).unwrap_or_default(),
        max_mint_batch: Some(msg.max_mint_batch).unwrap_or_else(|| Some(Uint128::from(10u128))),
        owners_can_burn: msg.owners_can_burn,
        minter_can_burn: msg.minter_can_burn,
        funds_wallet: msg.funds_wallet,
        store_conf: Some(msg.store_conf).unwrap(),
    };

    // We use the set_contract_version function that we loaded above using cw2
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // We save the state to the database
    CONFIG.save(deps.storage, &config)?;

    CW721Contract::default()
        .instantiate(
            deps,
            env,
            info,
            CW721InitMsg {
                name: msg.name,
                symbol: msg.symbol,
                minter: msg.minter
            }
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint{} => execute_mint(env, deps, info),
        ExecuteMsg::MintBatch(mint_msg) => execute_mint_batch(env, deps, info, mint_msg),
        ExecuteMsg::Burn { token_id } => execute_burn(deps, info, token_id),
        ExecuteMsg::BurnBatch { tokens } => execute_burn_batch(deps, info, tokens),
        ExecuteMsg::Store(store_msg) => execute_store(deps, info, store_msg),
        ExecuteMsg::StoreBatch(store_msg) => execute_store_batch(deps, info, store_msg),
        ExecuteMsg::StoreConf(msg) => execute_store_conf(deps, info, msg),
        // CW721 methods
        _ => CW721Contract::default()
            .execute(deps, env, info, msg.into())
            .map_err(|err| err.into()),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::NftInfoBatch { tokens } => to_binary(&query_nft_info_batch(deps, tokens)?),
        QueryMsg::BurntAmount { address } => to_binary(&query_burnt_amount(deps, address)?),
        QueryMsg::BurntList { address } => to_binary(&query_burnt_list(deps, address)?),
        QueryMsg::Burned { tokens } => to_binary(&query_burned(deps, tokens)?),
        // CW721 methods
        _ => CW721Contract::default().query(deps, env, msg.into()),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    msg: MigrateMsg<Config>,
) -> Result<Response, ContractError> {
    let MigrateMsg { version, config } = msg;
    try_migrate(deps, version, config)
    // match msg {
    //     MigrateMsg { version, config } => try_migrate(deps, version, config),
    // }
}
