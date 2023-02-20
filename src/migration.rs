use cosmwasm_std::{Response, Storage};

use crate::contract::CONTRACT_NAME;

use crate::error::ContractError;
use crate::state::{Config, CONFIG};

// From the cw2 crate we're loading the following:
use cw2::{get_contract_version, set_contract_version};

pub fn migrate_with_conf(
    storage: &mut dyn Storage,
    version: String,
    config: Config,
) -> Result<Response, ContractError> {
    let contract_version = get_contract_version(storage)?;

    let old_name = contract_version.contract;
    let old_version = contract_version.version;

    set_contract_version(storage, CONTRACT_NAME, version.clone())?;

    CONFIG.save(storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "migration")
        .add_attribute("old_name", old_name)
        .add_attribute("old_version", old_version)
        .add_attribute("name", CONTRACT_NAME)
        .add_attribute("version", version)
    )
}
