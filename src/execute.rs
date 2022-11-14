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
};

use crate::helpers::{
    _can_mint,
    _can_pay,
    _can_store,
    _can_update,
    _try_mint,
    _try_store,
    __update_total,
    __burn_token,
    __update_burnt_amount,
    __update_burnt_list
};

use crate::error::ContractError;

use crate::msg::{ BatchStoreMsg, BatchMintMsg, StoreConfMsg };

pub fn execute_burn(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    let cw721_contract = CW721Contract::default();
    let config = CONFIG.load(deps.storage)?;

    if config.owners_can_burn {
        let token = cw721_contract.tokens.load(deps.storage, &token_id)?;

        if token.owner != info.sender {
            return Err(ContractError::Unauthorized {})
        }

        __burn_token(&cw721_contract, deps.storage, token_id.clone())?;

        __update_burnt_amount(deps.storage, &info.sender)?;

        __update_burnt_list(deps.storage, &info.sender, &token_id)?;

        return Ok(Response::new()
            .add_attribute("action", "burn")
            .add_attribute("type", "owner_burn")
            .add_attribute("token_id", token_id))
    }

    if config.minter_can_burn {
        // validate sender permissions
        _can_update(&deps, &info)?;

        __burn_token(&cw721_contract, deps.storage, token_id.clone())?;

        __update_burnt_list(deps.storage, &info.sender, &token_id)?;

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

    if tokens.len() > 30 {
        return Err(ContractError::RequestTooLarge{ size: tokens.len() })
    }

    if tokens.is_empty() {
        return Err(ContractError::RequestTooSmall{ size: tokens.len() })
    }

    if config.owners_can_burn {
        let mut burnt_tokens = vec![];

        for token_id in tokens {
            let token = cw721_contract.tokens.load(deps.storage, &token_id)?;

            if token.owner != info.sender {
                return Err(ContractError::Unauthorized {})
            }

            __burn_token(&cw721_contract, deps.storage, token_id.clone())?;

            __update_burnt_amount(deps.storage, &info.sender)?;

            __update_burnt_list(deps.storage, &info.sender, &token_id)?;

            burnt_tokens.push(token_id);
        }

        return Ok(Response::new()
            .add_attribute("action", "burn_batch")
            .add_attribute("type", "owner_burn")
            .add_attribute("tokens", format!("[{}]", burnt_tokens.join(",")))
        )
    }

    if config.minter_can_burn {
        // validate sender permissions
        _can_update(&deps, &info)?;

        let mut burnt_tokens = vec![];

        for token_id in tokens {
            __burn_token(&cw721_contract, deps.storage, token_id.clone())?;

            __update_burnt_list(deps.storage, &info.sender, &token_id)?;

            burnt_tokens.push(token_id);
        }

        return Ok(Response::new()
            .add_attribute("action", "burn")
            .add_attribute("type", "minter_burn")
            .add_attribute("tokens", format!("[{}]", burnt_tokens.join(",")))
        )
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

    // check if we can mint
    let current_token_id = _can_mint(
        &current_count,
        &env.block.time,
        &config.start_mint,
        config.token_total,
        config.token_supply,
        &minter,
        &info.sender
    )?;

    // validate funds according to set price
    let coin_found = _can_pay(&config, &info, Uint128::from(1u32))?;

    _try_mint(
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

    let mut mint_amount = msg.amount;

    if mint_amount == Uint128::from(0u32) {
        mint_amount = Uint128::from(1u32)
    }

    if mint_amount > config.max_mint_batch {
        mint_amount = config.max_mint_batch
    }

    // check if we can mint
    let mut current_token_id = _can_mint(
        &minted_total,
        &env.block.time,
        &config.start_mint,
        config.token_total,
        config.token_supply,
        &minter,
        &info.sender
    )?;

    // validate funds according to set price and total to mint
    let mut coin_found = _can_pay(&config, &info, mint_amount)?;

    let mut total_minted = 0u32;

    let mut ids: Vec<String> = vec![];

    while Uint128::from(total_minted) < mint_amount {
        let res = _try_mint(
            deps.storage,
            &info.sender,
            &minter,
            &cw721_contract,
            &current_token_id.to_string()
        );

        if res.is_ok() {
            total_minted += 1;
            current_token_id += Uint128::from(1u32);
            ids.push(current_token_id.to_string())
        }
    }

    coin_found.amount = config.cost_amount * Uint128::from(total_minted);

    // send funds to the configured funds wallet
    // send the info below
    Ok(Response::new()
        .add_attribute("action", "mint_batch")
        .add_attribute("owner", info.sender)
        .add_attribute("requested", msg.amount.to_string())
        .add_attribute("minted", total_minted.to_string())
        .add_attribute("cost", coin_found.amount.to_string())
        .add_attribute("list", format!("[{}]", ids.join(",")))
        .add_message(
            CosmosMsg::Bank(BankMsg::Send {
                to_address: config.funds_wallet,
                amount: vec![coin_found],
            })
        )
    )
}

pub fn execute_store(
    deps: DepsMut,
    info: MessageInfo,
    nft_data: MintMsg<Extension>,
) -> Result<Response, ContractError> {
    // validate sender permissions
    _can_store(&deps, &info)?;

    let cw721_contract = CW721Contract::default();
    let minter = cw721_contract.minter.load(deps.storage)?;

    _try_store(deps.storage, &nft_data, &minter, &cw721_contract)?;

    let total = CONFIG.load(deps.storage)?.token_total + Uint128::from(1u8);
    __update_total(deps.storage, total)?;

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
    _can_store(&deps, &info)?;

    let cw721_contract = CW721Contract::default();
    let minter = cw721_contract.minter.load(deps.storage)?;

    let mut total = CONFIG.load(deps.storage)?.token_total;
    for nft_data in data.batch {
        _try_store(deps.storage, &nft_data, &minter, &cw721_contract)?;
        total += Uint128::from(1u8)
    }

    __update_total(deps.storage, total)?;

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
    _can_store(&deps, &info)?;

    let cw721_contract = CW721Contract::default();
    let minter = cw721_contract.minter.load(deps.storage)?;

    let mut config = CONFIG.load(deps.storage)?.store_conf;
    if config.is_none() && msg.conf.is_none() {
        return Err(ContractError::NoConfiguration {})
    }

    if msg.conf.is_some() {
        config = msg.conf
    }

    let conf = config.unwrap();

    let mut total = CONFIG.load(deps.storage)?.token_total;

    for attr_values in msg.attributes {
        let name = format!("{} #{}", conf.name, total);

        let mut attr : Vec<Trait> = vec![];
        for (index, value) in attr_values.iter().enumerate() {
            attr.push(Trait {
                display_type: None,
                value: value.clone(),
                trait_type: conf.attributes[index].clone()
            })
        }

        let token = TokenInfo {
            owner: minter.clone(),
            approvals: vec![],
            token_uri: None,
            extension: Some(Metadata {
                name: Some(name.clone()),
                description: Some(conf.desc.to_string()),
                image: Some(format!("{}/{}.png", conf.ipfs, total)),
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

    __update_total(deps.storage, total)?;

    Ok(Response::new()
        .add_attribute("action", "store_conf")
        .add_attribute("token_total", total.to_string())
    )
}
