#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw721::ContractInfoResponse;
// use cw721::Cw721Query;

use crate::helpers::clear_state;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg };
use crate::state::{Config, CW721Contract, CONFIG};

use cw2::{get_contract_version, set_contract_version};

pub use cw721_base::{
    MintMsg,
    MinterResponse,
    InstantiateMsg as CW721InitMsg
};

use crate::execute::{
    execute_transfer_batch,
    execute_freeze,
    execute_update_conf,
    execute_burn,
    execute_burn_batch,
    execute_mint,
    execute_mint_batch,
    execute_store,
    execute_store_batch,
    execute_store_conf,
    execute_pause,
    execute_unpause,
    execute_unfreeze, execute_pledge,
};

use crate::error::ContractError;

use crate::migration::migrate_with_conf;

use crate::query::{
    query_config,
    query_nft_info_batch,
    query_burnt_amount,
    // query_burnt_list,
    // query_burned,
};

// version info for migration info
pub const CONTRACT_NAME: &str = "crates.io:cw721-custom";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let cw721_contract = CW721Contract::default();

    let config = Config {
        creator: msg.creator,
        name: msg.name.clone(),
        token_supply: msg.token_supply,
        token_total: Uint128::zero(),
        cost: msg.cost,
        dates: Some(msg.dates).unwrap_or_default(),
        max_mint_batch: Some(msg.max_mint_batch).unwrap_or_else(|| Some(Uint128::from(10u128))),
        burn: msg.burn,
        wallet: msg.wallet,
        store_conf: Some(msg.store_conf).unwrap(),
        frozen: false,
        paused: false,
    };

    // We use the set_contract_version function that we loaded above using cw2
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // We save the config to the state
    CONFIG.save(deps.storage, &config)?;

    let info = ContractInfoResponse {
        name: msg.name,
        symbol: msg.symbol,
    };

    cw721_contract.contract_info.save(deps.storage, &info)?;
    let minter = deps.api.addr_validate(&config.creator)?;
    cw721_contract.minter.save(deps.storage, &minter)?;

    Ok(Response::default().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Freeze{} => execute_freeze(deps, info),
        ExecuteMsg::Unfreeze{} => execute_unfreeze(deps, info),

        ExecuteMsg::Pause{} => execute_pause(deps, info),
        ExecuteMsg::Unpause{} => execute_unpause(deps, info),

        ExecuteMsg::Mint{} => execute_mint(env, deps, info),
        ExecuteMsg::MintBatch(mint_msg) => execute_mint_batch(env, deps, info, mint_msg),

        ExecuteMsg::Pledge { tokens } => execute_pledge(env, deps, info, tokens),

        ExecuteMsg::Burn { token_id } => execute_burn(env, deps, info, token_id),
        ExecuteMsg::BurnBatch { tokens } => execute_burn_batch(env, deps, info, tokens),

        ExecuteMsg::Store(store_msg) => execute_store(deps, info, store_msg),
        ExecuteMsg::StoreBatch(store_msg) => execute_store_batch(deps, info, store_msg),
        ExecuteMsg::StoreConf(msg) => execute_store_conf(deps, info, msg),

        ExecuteMsg::TransferBatch(transfer) => execute_transfer_batch(env, deps, info, transfer),

        ExecuteMsg::UpdateConf(msg) => execute_update_conf(deps, info, msg),

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
        // QueryMsg::BurntList { address } => to_binary(&query_burnt_list(deps, address)?),
        // QueryMsg::Burned { tokens } => to_binary(&query_burned(deps, tokens)?),
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
    match msg {
        MigrateMsg::WithConfig { version, config } => {
            let current = get_contract_version(deps.storage)?;

            if current.version != version {
                let res = migrate_with_conf(deps.storage, version.clone(), config);

                if res.is_ok() {
                    return Ok(
                        Response::default()
                            .add_attribute("action", "migration")
                            .add_attribute("new_version", version)
                            .add_attribute("old_version", current.version)
                    )
                }
            }

            Err(ContractError::MigrationSameVersion { version })
        },
        MigrateMsg::WithConfigClearState { version, config } => {
            let current = get_contract_version(deps.storage)?;

            if current.version != version {
                let cw721_contract = CW721Contract::default();

                clear_state(deps.storage)?;

                cw721_contract.tokens.clear(deps.storage);

                let res = migrate_with_conf(deps.storage, version.clone(), config);

                if res.is_ok() {
                    return Ok(
                        Response::default()
                            .add_attribute("action", "migration")
                            .add_attribute("new_version", version)
                            .add_attribute("old_version", current.version)
                    )
                }
            }

            Err(ContractError::MigrationSameVersion { version })
        },
        _ => Err(ContractError::MigrationWrongStrategy {})
    }
}
