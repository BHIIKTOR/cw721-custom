use cosmwasm_std::{
    Env,
    DepsMut,
    MessageInfo,
    Response,
    BankMsg,
    CosmosMsg,
    Uint128, Storage,
};

use cw721_base::{ MintMsg };
use cw721_base::state::{ TokenInfo };

use crate::state::{
    CW721Contract,
    Extension,
    CONFIG,
    Metadata,
    Trait,
    Config, PLEDGED_TOKENS_BY_ADDR, PLEDGED_TOKENS,
};

use crate::helpers::{
    transfer_nft,
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
    MintBatchMsg,
    StoreConfMsg,
    InstantiateMsg, TransferOperation
};

pub fn execute_freeze(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    can_update(&deps, &info)?;

    let mut config = CONFIG.load(deps.storage)?;
    config.frozen = true;

    CONFIG.save(deps.storage, &config)?;

    Ok(
        Response::new()
            .add_attribute("action", "freeze")
    )
}

pub fn execute_unfreeze(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    can_update(&deps, &info)?;

    let mut config = CONFIG.load(deps.storage)?;
    config.frozen = false;

    CONFIG.save(deps.storage, &config)?;

    Ok(
        Response::new()
            .add_attribute("action", "unfreeze")
    )
}

pub fn execute_pause(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    can_update(&deps, &info)?;

    let mut config = CONFIG.load(deps.storage)?;
    config.paused = true;
    CONFIG.save(deps.storage, &config)?;

    Ok(
        Response::new()
            .add_attribute("action", "pause")
    )
}

pub fn execute_unpause(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    can_update(&deps, &info)?;

    let mut config = CONFIG.load(deps.storage)?;
    config.paused = false;

    CONFIG.save(deps.storage, &config)?;

    Ok(
        Response::new()
            .add_attribute("action", "unpause")
    )
}

pub fn execute_update_conf(
    deps: DepsMut,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    can_update(&deps, &info)?;

    let config = CONFIG.load(deps.storage)?;

    if config.frozen {
        return Err(ContractError::ContractFrozen {  })
    }

    let config = Config {
        creator: msg.creator,
        name: msg.name,
        token_supply: msg.token_supply,
        token_total: Uint128::zero(),
        cost: msg.cost,
        dates: Some(msg.dates).unwrap_or_default(),
        max_mint_batch: Some(msg.max_mint_batch).unwrap_or_else(|| Some(Uint128::from(10u128))),
        burn: msg.burn,
        wallet: msg.wallet,
        store_conf: msg.store_conf,
        frozen: false,
        paused: false,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(
        Response::new()
            .add_attribute("action", "config")
            .add_attribute("sub", "update")
            .add_attribute("result", "success")
    )
}

pub fn execute_transfer_batch(
    env: Env,
    deps: DepsMut,
    info: MessageInfo,
    transfer: TransferOperation,
) -> Result<Response, ContractError> {
    let cw721_contract = CW721Contract::default();

    let mut results = vec![];

    let recipient_address = &deps.api.addr_validate(&transfer.recipient)?;

    for token_id in transfer.tokens {
        let token = cw721_contract.tokens.load(deps.storage, &token_id)?;

        match transfer_nft(
            deps.storage,
            &env,
            &cw721_contract,
            &info,
            recipient_address,
            &token,
            &token_id
        ) {
            Ok(_) => results.push(token_id.to_string()),
            Err(e) => {
                let v = vec![token_id, e.to_string()];
                results.push(v.join(", "))
            },
        }
    }

    Ok(
        Response::new()
            .add_attribute("action", "transfer_batch")
            .add_attribute("sender", info.sender)
            .add_attribute("recipient", transfer.recipient)
            .add_attribute("result", format!("{:?}", results))
    )
}

pub fn execute_pledge(
    _env: Env,
    deps: DepsMut,
    info: MessageInfo,
    tokens: Vec<String>,
) -> Result<Response, ContractError> {
    let cw721_contract = CW721Contract::default();

    let config = CONFIG.load(deps.storage)?;
    let mut pledged_list: Vec<String> = Vec::with_capacity(tokens.len());
    let mut errors: Vec<String> = Vec::with_capacity(tokens.len());

    let mut response: Response = Response::default().add_attribute("action", "pledge");

    let mut handle_error = |
        e: ContractError
    | {
        errors.push(e.to_string())
    };

    tokens
        .into_iter()
        .for_each(|token_id| {
            if !cw721_contract.tokens.has(deps.storage, &token_id) {
                handle_error(ContractError::DontExists { });
                return
            }

            let res = cw721_contract.tokens.load(deps.storage, &token_id);

            if res.is_err() {
                handle_error(ContractError::Std( res.err().unwrap() ));
                return
            }

            let token = res.unwrap();

            if token.owner == info.sender || info.sender == config.creator {
                if PLEDGED_TOKENS.has(deps.storage, token_id.clone()) {
                    let res = PLEDGED_TOKENS.load(deps.storage, token_id.clone());

                    if res.is_err() {
                        handle_error(ContractError::Std( res.err().unwrap() ));
                        return
                    }

                    // if the value is true finish the operation
                    // token has been burned already
                    if res.unwrap() {
                        handle_error(ContractError::TokenPledged { token_id });
                        return
                    }
                }

                let res = PLEDGED_TOKENS.save(deps.storage, token_id.clone(), &false);

                if res.is_err() {
                    handle_error(ContractError::Std( res.err().unwrap() ));
                    return
                }

                pledged_list.push(token_id)
            } else {
                handle_error(ContractError::Unauthorized { });
            }
        });

    if !pledged_list.is_empty() {
        // add pledged tokens to address
        if PLEDGED_TOKENS_BY_ADDR.has(deps.storage, &info.sender) {
            let mut old = PLEDGED_TOKENS_BY_ADDR.load(deps.storage, &info.sender)?;
            pledged_list.append(&mut old);
        }

        let res = PLEDGED_TOKENS_BY_ADDR.save(deps.storage, &info.sender, &pledged_list);

        if res.is_err() {
            pledged_list
                .into_iter()
                .for_each(|token_id| {
                    PLEDGED_TOKENS.remove(deps.storage, token_id);
                });

            handle_error(ContractError::Std( res.err().unwrap() ));
        } else {
            response = response.add_attribute("list", format!("{:?}", pledged_list));
        }
    }

    if !errors.is_empty() {
        response = response.add_attribute("errors", format!("{:?}", errors));
    }

    Ok(response)
}

pub fn execute_burn(
    env: Env,
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    if !PLEDGED_TOKENS.has(deps.storage, token_id.clone()) {
        return Err(ContractError::TokenNotPledged { token_id })
    }

    let cw721_contract = CW721Contract::default();
    let config = CONFIG.load(deps.storage)?;

    let token: TokenInfo<Option<Metadata>> = cw721_contract.tokens.load(deps.storage, &token_id)?;

    if config.burn.owner_can_burn && token.owner == info.sender {
        burn_and_update(
            &cw721_contract,
            deps.storage,
            &token,
            &token_id,
            &info.sender,
            &env.block,
            true
        )?;

        return Ok(Response::new()
            .add_attribute("action", "burn")
            .add_attribute("sub", "owner_burn")
            .add_attribute("token_id", token_id))
    }

    if config.burn.can_burn_owned && config.creator == info.sender {
        // validate sender permissions
        // can_update(&deps, &info)?;
        burn_and_update(
            &cw721_contract,
            deps.storage,
            &token,
            &token_id,
            &info.sender,
            &env.block,
            false
        )?;

        return Ok(Response::new()
            .add_attribute("action", "burn")
            .add_attribute("sub", "admin_burn")
            .add_attribute("token_id", token_id))
    } else if token.owner != info.sender {
        return Err(ContractError::UnauthorizedWithMsg {
            msg: "sender is not owner".to_string()
        })
    }

    Ok(Response::new()
        .add_attribute("action", "burn_nothing")
        .add_attribute("why", "configuration")
        .add_attribute("owners_can_burn", config.burn.owner_can_burn.to_string())
        .add_attribute("admin_can_burn", "true")
    )
}

pub fn execute_burn_batch(
    env: Env,
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

    let mut owner_burn = false;
    let mut admin_burn = false;
    let mut errors: Vec<String> = Vec::with_capacity(tokens.len());
    let mut response;

    let mut handle_error = |
        is_err: bool,
        e: Option<ContractError>
    | {
        if is_err {
            errors.push(e.unwrap().to_string())
        }
    };

    let call_burn_and_update = |
        storage: &mut dyn Storage,
        token: &TokenInfo<Option<Metadata>>,
        token_id: &String,
        check_owner: bool
    | -> Result<(), ContractError> {
        burn_and_update(
            &cw721_contract,
            storage,
            token,
            token_id,
            &info.sender,
            &env.block,
            check_owner
        )
    };

    let token_list: Vec<(bool, String)> = tokens
        .into_iter()
        .map(|token_id| {
            if !cw721_contract.tokens.has(deps.storage, &token_id) {
                handle_error(true, Some(ContractError::DontExists { }));
                return (false, token_id)
            }

            if !PLEDGED_TOKENS.has(deps.storage, token_id.clone()) {
                handle_error(true, Some(ContractError::TokenNotPledged { token_id: token_id.clone() }));
                return (false, token_id)
            }

            let token = cw721_contract.tokens.load(deps.storage, token_id.as_str()).unwrap();

            let no_auth = ContractError::UnauthorizedWithMsg {
                msg: "sender is not owner".to_string()
            };

            if config.burn.owner_can_burn && token.owner == info.sender {
                // NOTE: IS THIS EFFICIENT?
                owner_burn = true;

                if token.owner == config.creator {
                    handle_error(true, Some(no_auth))
                } else {
                    let op_res = call_burn_and_update(deps.storage, &token, &token_id, true);

                    handle_error(op_res.is_err(), op_res.err());

                    return (true, token_id)
                }
            } else if config.creator == info.sender {
                // NOTE: IS THIS EFFICIENT?
                admin_burn = true;

                let op_res = call_burn_and_update(deps.storage, &token, &token_id, !config.burn.can_burn_owned);

                handle_error(op_res.is_err(), op_res.err());

                return (true, token_id)
            }

            (false, token_id)
        })
        .collect();

    if owner_burn {
        response = Response::new()
            .add_attribute("action", "burn_batch")
            .add_attribute("sub", "owner_burn")
    } else if admin_burn {
        response = Response::new()
            .add_attribute("action", "burn_batch")
            .add_attribute("sub", "admin_burn")
    } else {
        response = Response::new()
            .add_attribute("action", "burn_nothing")
            .add_attribute("why", "configuration")
            .add_attribute("owners_can_burn", config.burn.owner_can_burn.to_string())
            .add_attribute("admin_can_burn", "true");
    }

    if response.attributes[0].value != "burn_nothing" {
        response = response.add_attribute("results", format!("{:?}", token_list));

        if !errors.is_empty() {
            response = response.add_attribute("errors", format!("{:?}", errors));
        }
    }

    Ok(response)
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
    let mint_amount = Uint128::one();

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
                to_address: config.wallet.wallet.to_string(),
                amount: vec![coin_found],
            })
        )
    )
}

pub fn execute_mint_batch(
    env: Env,
    deps: DepsMut,
    info: MessageInfo,
    msg: MintBatchMsg,
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
        //atempt to mint
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
            current_token_id += Uint128::one();
            ids.push(current_token_id.to_string())
        }

        // push error msg to response msg
        if res.is_err() {
            errors.push(format!("token: {}, error: {}", current_token_id, res.unwrap_err()))
        }
    }

    coin_found.amount = config.cost.amount * Uint128::from(total_minted);

    response_msg = response_msg.add_attribute("minted", total_minted.to_string())
        .add_attribute("cost", coin_found.amount.to_string())
        .add_attribute("list", format!("{:?}", ids));

    if !errors.is_empty() {
        response_msg = response_msg.add_attribute("errors", format!("{:?}", errors));
    }

    // send funds to the configured funds wallet
    response_msg = response_msg.add_message(
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.wallet.wallet.into_string(),
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

    let total = CONFIG.load(deps.storage)?.token_total + Uint128::one();
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

    let mut store_conf = conf.store_conf;

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

        total += Uint128::one()
    }

    total = update_total(deps.storage, &total)?;

    Ok(Response::new()
        .add_attribute("action", "store_conf")
        .add_attribute("token_total", total.to_string())
    )
}
