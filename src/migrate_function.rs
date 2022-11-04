use cosmwasm_std::{DepsMut, Response};

// From the cw2 crate we're loading the following:
use crate::error::ContractError;
use crate::state::{Config, CONFIG};
use cw2::{get_contract_version, set_contract_version};

pub fn try_migrate(
    deps: DepsMut,
    version: String,
    config: Option<Config>,
) -> Result<Response, ContractError> {
    let contract_version = get_contract_version(deps.storage)?;
    set_contract_version(deps.storage, contract_version.contract, version)?;

    if config.is_some() {
        CONFIG.save(deps.storage, &config.unwrap())?
    }

    Ok(Response::new()
        .add_attribute("method", "try_migrate")
        .add_attribute("version", contract_version.version))
}
