use cosmwasm_std::{DepsMut, Response};

use crate::contract::CONTRACT_NAME;
// From the cw2 crate we're loading the following:
use crate::error::ContractError;
use crate::state::{Config, CONFIG};
use cw2::{get_contract_version, set_contract_version};

pub fn migrate_with_conf(
    deps: DepsMut,
    version: String,
    config: Option<Config>,
) -> Result<Response, ContractError> {
    let contract_version = get_contract_version(deps.storage)?;
    let config = config.unwrap();

    let old_name = contract_version.contract.clone();
    let old_version = contract_version.version.clone();

    set_contract_version(deps.storage, CONTRACT_NAME, version.clone())?;

    CONFIG.save(deps.storage, &config)?;

    // TODO: Whipe all old state data

    Ok(Response::new()
        .add_attribute("method", "migration")
        .add_attribute("old_name", old_name)
        .add_attribute("old_version", old_version)
        .add_attribute("name", CONTRACT_NAME)
        .add_attribute("version", version)
    )
}
