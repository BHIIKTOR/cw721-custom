use cosmwasm_std::{
    Env,
    DepsMut,
    MessageInfo,
    Response,
    BankMsg,
    CosmosMsg,
    Uint128,
};

use cw721_base::{ MintMsg };
use cw721_base::state::{ TokenInfo };

use crate::state::{
    CW721Contract,
    Extension,
    CONFIG,
    Metadata,
    Trait,
    Config,
};

use crate::helpers::{
    can_mint,
    can_pay,
    can_store,
    can_update,
    try_mint,
    try_store,
    burn_and_update,
    update_total,
};

use crate::error::ContractError;

use crate::msg::{
    BatchStoreMsg,
    BatchMintMsg,
    StoreConfMsg,
    InstantiateMsg
};

pub fn execute_freeze(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    can_update(&deps, &info)?;

    let mut config = CONFIG.load(deps.storage)?;
    config.frozen = !config.frozen;

    CONFIG.save(deps.storage, &config)?;

    Ok(
        Response::new()
            .add_attribute("action", "freeze")
            .add_attribute("result", "success")
    )
}

pub fn execute_update_conf(
    deps: DepsMut,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    can_update(&deps, &info)?;

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

    CONFIG.save(deps.storage, &config)?;

    Ok(
        Response::new()
            .add_attribute("action", "update_config")
            .add_attribute("result", "success")
    )
}

pub fn execute_burn(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    let cw721_contract = CW721Contract::default();
    let config = CONFIG.load(deps.storage)?;
    let minter = cw721_contract.minter.load(deps.storage).unwrap();

    if config.owners_can_burn {
        let token = cw721_contract.tokens.load(deps.storage, &token_id)?;

        if token.owner == minter {
            return Err(ContractError::Unauthorized {})
        }

        if token.owner != info.sender {
            return Err(ContractError::Unauthorized {})
        }

        burn_and_update(
            &cw721_contract,
            deps.storage,
            &token_id,
            &info.sender,
            true
        )?;

        return Ok(Response::new()
            .add_attribute("action", "burn")
            .add_attribute("type", "owner_burn")
            .add_attribute("token_id", token_id))
    }

    if config.minter_can_burn {
        // validate sender permissions
        can_update(&deps, &info)?;

        burn_and_update(
            &cw721_contract,
            deps.storage,
            &token_id,
            &info.sender,
            true
        )?;

        return Ok(Response::new()
            .add_attribute("action", "burn")
            .add_attribute("type", "minter_burn")
            .add_attribute("token_id", token_id))
    }

    Ok(Response::new()
        .add_attribute("action", "burn_nothing")
        .add_attribute("why", "configuration")
        .add_attribute("owners_can_burn", config.owners_can_burn.to_string())
        .add_attribute("minter_can_burn", config.minter_can_burn.to_string())
    )
}

pub fn execute_burn_batch(
    deps: DepsMut,
    info: MessageInfo,
    tokens: Vec<String>
) -> Result<Response, ContractError> {
    let cw721_contract = CW721Contract::default();
    let config = CONFIG.load(deps.storage)?;
    let minter = cw721_contract.minter.load(deps.storage).unwrap();

    if tokens.len() > 30 {
        return Err(ContractError::RequestTooLarge{ size: tokens.len() })
    }

    if tokens.is_empty() {
        return Err(ContractError::RequestTooSmall{ size: tokens.len() })
    }

    if config.owners_can_burn {
        let mut burnt_tokens = Vec::with_capacity(tokens.len());
        let mut not_burnt_tokens = Vec::with_capacity(tokens.len());
        let mut errors : Vec<String> = Vec::with_capacity(tokens.len());

        for token_id in tokens {
            let token = cw721_contract.tokens.load(deps.storage, &token_id)?;

            if token.owner == minter || token.owner != info.sender {
                errors.push(ContractError::Unauthorized {}.to_string())
            } else {
                let res = burn_and_update(
                    &cw721_contract,
                    deps.storage,
                    &token_id,
                    &info.sender,
                    true
                );

                if let Err(e) = res {
                    not_burnt_tokens.push(token_id);
                    errors.push(e.to_string())
                } else {
                    burnt_tokens.push(token_id);
                }
            }
        }

        let mut res = Response::new();

        res = res
            .add_attribute("action", "burn_batch")
            .add_attribute("type", "owner_burn");

        if !burnt_tokens.is_empty() {
            res = res.add_attribute("burnt_tokens", format!("[{}]", burnt_tokens.join(",")));
        }

        if !not_burnt_tokens.is_empty() {
            res = res.add_attribute("not_burnt", format!("[{}]", not_burnt_tokens.join(",")));
        }

        if !errors.is_empty() {
            res = res.add_attribute("errors", format!("[{}]", errors.join(",")));
        }

        return Ok(res)
    }

    if config.minter_can_burn {
        // validate sender permissions
        can_update(&deps, &info)?;

        let mut burnt_tokens = Vec::with_capacity(tokens.len());
        let mut not_burnt_tokens = Vec::with_capacity(tokens.len());

        for token_id in tokens {
            let res = burn_and_update(
                &cw721_contract,
                deps.storage,
                &token_id,
                &info.sender,
                false
            );

            if res.is_err() {
                not_burnt_tokens.push(token_id)
            } else {
                burnt_tokens.push(token_id);
            }
        }

        let mut res = Response::new();

        res = res
            .add_attribute("action", "burn_batch")
            .add_attribute("type", "owner_burn")
            .add_attribute("tokens", format!("[{}]", burnt_tokens.join(",")));

        if !not_burnt_tokens.is_empty() {
            res = res.add_attribute("not_burn", format!("[{}]", not_burnt_tokens.join(",")));
        }

        return Ok(res)
    }

    Ok(Response::new()
        .add_attribute("action", "burn_nothing")
        .add_attribute("why", "configuration")
        .add_attribute("owners_can_burn", config.owners_can_burn.to_string())
        .add_attribute("minter_can_burn", config.minter_can_burn.to_string())
    )
}

pub fn execute_mint(
    env: Env,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let cw721_contract = CW721Contract::default();
    let config = CONFIG.load(deps.storage)?;
    let minter = cw721_contract.minter.load(deps.storage)?;
    let current_count = cw721_contract.token_count(deps.storage)?;
    let mint_amount = Uint128::from(1u32);

    // check if we can mint
    let current_token_id = can_mint(
        &current_count,
        &env.block.time,
        &config,
        &mint_amount,
        &minter,
        &info.sender
    )?;

    // validate funds according to set price
    let coin_found = can_pay(&config, &info, &mint_amount)?;

    try_mint(
        deps.storage,
        &info.sender,
        &minter,
        &cw721_contract,
        &current_token_id.to_string()
    )?;

    // send funds to the configured funds wallet
    // send the info below
    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("owner", info.sender)
        .add_attribute("token_id", current_count.to_string())
        .add_message(
            CosmosMsg::Bank(BankMsg::Send {
                to_address: config.funds_wallet,
                amount: vec![coin_found],
            })
        )
    )
}

pub fn execute_mint_batch(
    env: Env,
    deps: DepsMut,
    info: MessageInfo,
    msg: BatchMintMsg,
) -> Result<Response, ContractError> {
    let cw721_contract = CW721Contract::default();

    let config = CONFIG.load(deps.storage)?;

    let minted_total = cw721_contract.token_count(deps.storage)?;
    let minter = cw721_contract.minter.load(deps.storage)?;

    let mint_amount = msg.amount;

    // check if we can mint
    let mut current_token_id = can_mint(
        &minted_total,
        &env.block.time,
        &config,
        &msg.amount,
        &minter,
        &info.sender
    )?;

    // validate funds according to set price and total to mint
    let mut coin_found = can_pay(&config, &info, &mint_amount)?;

    let mut total_minted = 0u32;

    let mut ids: Vec<String> = vec![];

    let mut response_msg = Response::new();

    response_msg = response_msg.add_attribute("action", "mint_batch")
        .add_attribute("owner", &info.sender)
        .add_attribute("requested", msg.amount.to_string());

    let mut errors : Vec<String> = Vec::new();

    while Uint128::from(total_minted) < mint_amount {
        let res = try_mint(
            deps.storage,
            &info.sender,
            &minter,
            &cw721_contract,
            &current_token_id.to_string()
        );

        // push token id to the ids list
        if res.is_ok() {
            total_minted += 1;
            current_token_id += Uint128::from(1u32);
            ids.push(current_token_id.to_string())
        }

        // push error msg to response msg
        if res.is_err() {
            errors.push(format!("{}:{}", current_token_id, res.unwrap_err().to_string()))
        }
    }

    coin_found.amount = config.cost_amount * Uint128::from(total_minted);

    response_msg = response_msg.add_attribute("minted", total_minted.to_string())
        .add_attribute("cost", coin_found.amount.to_string())
        .add_attribute("list", format!("[{}]", ids.join(",")));

    if !errors.is_empty() {
        response_msg = response_msg.add_attribute("errors", format!("[{}]", errors.join(",")));
    }

    // send funds to the configured funds wallet
    response_msg = response_msg.add_message(
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.funds_wallet,
            amount: vec![coin_found],
        })
    );

    Ok(response_msg)
}

pub fn execute_store(
    deps: DepsMut,
    info: MessageInfo,
    nft_data: MintMsg<Extension>,
) -> Result<Response, ContractError> {
    // validate sender permissions
    can_store(&deps, &info)?;

    let cw721_contract = CW721Contract::default();
    let minter = cw721_contract.minter.load(deps.storage)?;

    try_store(deps.storage, &nft_data, &minter, &cw721_contract)?;

    let total = CONFIG.load(deps.storage)?.token_total + Uint128::from(1u8);
    update_total(deps.storage, &total)?;

    Ok(Response::new()
        .add_attribute("action", "store")
        .add_attribute("token_total", total.to_string())
    )
}

pub fn execute_store_batch(
    deps: DepsMut,
    info: MessageInfo,
    data: BatchStoreMsg,
) -> Result<Response, ContractError> {
    // validate sender permissions
    can_store(&deps, &info)?;

    let cw721_contract = CW721Contract::default();
    let minter = cw721_contract.minter.load(deps.storage)?;

    for nft_data in &data.batch {
        try_store(deps.storage, nft_data, &minter, &cw721_contract)?;
    }

    let batch_total = Uint128::from(data.batch.len() as u32);

    // batch size is summed in the operation below and returns the new total
    let total = update_total(deps.storage, &batch_total)?;

    Ok(Response::new()
        .add_attribute("action", "store_batch")
        .add_attribute("token_total", total.to_string())
    )
}

pub fn execute_store_conf(
    deps: DepsMut,
    info: MessageInfo,
    msg: StoreConfMsg,
)-> Result<Response, ContractError> {
    // validate sender permissions
    can_store(&deps, &info)?;

    let cw721_contract = CW721Contract::default();
    let minter = cw721_contract.minter.load(deps.storage)?;
    let conf = CONFIG.load(deps.storage)?;

    // check if we have store_conf from any of this two sources
    // first the states CONFIG and second the StoreConfMsg in the TX
    if conf.store_conf.is_none() && msg.conf.is_none() {
        return Err(ContractError::NoConfiguration {})
    }

    let mut store_conf = conf.store_conf.unwrap();

    // overwrite the original if we have data in the msg
    if msg.conf.is_some() {
        store_conf = msg.conf.unwrap()
    }

    // unwrap store_conf
    let store_data = store_conf;

    let mut total = conf.token_total;

    for attr_values in msg.attributes {
        let name = format!("{} #{}", store_data.name, total);

        let mut attr : Vec<Trait> = vec![];
        for (index, value) in attr_values.iter().enumerate() {
            attr.push(Trait {
                display_type: None,
                value: value.clone(),
                trait_type: store_data.attributes[index].clone()
            })
        }

        let token = TokenInfo {
            owner: minter.clone(),
            approvals: vec![],
            token_uri: None,
            extension: Some(Metadata {
                name: Some(name.clone()),
                description: Some(store_data.desc.to_string()),
                image: Some(format!("{}/{}.png", store_data.ipfs, total)),
                attributes: Some(attr),
                animation_url: None,
                background_color: None,
                image_data: None,
                external_url: None,
                youtube_url: None,
            })
        };

        cw721_contract.tokens.save(deps.storage, &total.to_string(), &token)?;

        total += Uint128::from(1u8)
    }

    total = update_total(deps.storage, &total)?;

    Ok(Response::new()
        .add_attribute("action", "store_conf")
        .add_attribute("token_total", total.to_string())
    )
}
