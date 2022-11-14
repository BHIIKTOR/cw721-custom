use cosmwasm_std::{
    Uint128,
    Addr,
    Deps,
    StdResult,
    StdError
};

use crate::state::{
    Extension,
    CW721Contract
};

use cw721_base::state::{ TokenInfo };

use crate::state::{
    CONFIG,
    Config,
    BURNT_AMOUNT,
    BURNT_LIST,
    BURNED
};

pub fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_nft_info_batch(
    deps: Deps,
    tokens: Vec<String>
) -> StdResult<Vec<TokenInfo<Extension>>> {
    if tokens.len() > 30 {
        return Err(StdError::generic_err("request too large"))
    }

    let contract = CW721Contract::default();
    let mut data: Vec<TokenInfo<Extension>> = vec![];

    for token in tokens {
        // match contract.tokens.may_load(deps.storage, &token).unwrap() {
        //     Some(item) => data.push(item),
        //     None => {}
        // }
        if let Some(item) = contract.tokens.may_load(deps.storage, &token).unwrap() {
            data.push(item)
        }
    }

    Ok(data)
}

pub fn query_burnt_amount(
    deps: Deps,
    address: Addr,
) -> StdResult<Option<Uint128>> {
    BURNT_AMOUNT.may_load(deps.storage, &address)
}

pub fn query_burnt_list(
    deps: Deps,
    address: Addr,
) -> StdResult<Option<Vec<String>>> {
    BURNT_LIST.may_load(deps.storage, &address)
}

pub fn query_burned(
    deps: Deps,
    tokens: Vec<String>
) -> StdResult<Vec<(String, bool)>> {
    if tokens.len() > 30 {
        return Err(StdError::generic_err("request too large"))
    }

    let mut data: Vec<(String, bool)> = vec![];

    for token in tokens {
        // match BURNED.may_load(deps.storage, token.clone()).unwrap() {
        //     Some(item) => data.push((token, item)),
        //     None => {}
        // }
        if let Some(item) = BURNED.may_load(deps.storage, token.clone()).unwrap() { data.push((token, item)) }
    }

    Ok(data)
}