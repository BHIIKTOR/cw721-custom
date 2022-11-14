#[cfg(test)]
mod tests {
    use crate::state::{Extension};
    use crate::contract::{execute, instantiate, query};

    use crate::msg::{
        InstantiateMsg,
        ExecuteMsg,
        QueryMsg,
        BatchStoreMsg,
        BatchMintMsg,
    };

    use cw721_base::{ MintMsg };

    use cosmwasm_std::{
        from_binary,
        Uint128,
        Coin,
        Addr
    };

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use cw721::{
        OwnerOfResponse
    };

    use crate::helpers::_now;

    const CREATOR: &str = "creator";
    const MINTER: &str = "minter";

    // const MINTER2: &str = "minter2";
    const FUNDWALLET: &str = "wallet";

    const DENOM: &str = "uluna";
    const SUPPLY: u128 = 5000u128;
    const COST: u128 = 4000000u128;

    fn get_mint_msg (id: String) -> MintMsg<Extension> {
        return MintMsg {
            token_id: String::from(id),
            owner: CREATOR.to_string(),
            token_uri: None,
            extension: None,
        };
    }

    fn get_init_msg () -> InstantiateMsg {
        InstantiateMsg {
            name: "nftt".to_string(),
            symbol: "NFTT".to_string(),
            minter: CREATOR.to_string(),
            funds_wallet: FUNDWALLET.to_string(),
            token_supply: Uint128::from(SUPPLY),
            cost_denom: DENOM.to_string(),
            cost_amount: Uint128::from(COST),
            max_mint_batch: Uint128::from(10u128),
            owners_can_burn: false,
            minter_can_burn: true,
            start_mint: Some(_now()),
            store_conf: None
        }
    }

    fn get_store_batch_msg () -> BatchStoreMsg {
        let mut batch: Vec<MintMsg<Extension>> = vec![];
        let num: usize = 20;

        for elem in 0..num {
            // let mut msg = msg_1.clone();
            // msg.token_id = String::from(format!("{}", elem));
            batch.push(get_mint_msg(String::from(format!("{}", elem))));
        }

        return BatchStoreMsg {
            batch
        }
    }

    #[test]
    fn store() {
        let mut deps = mock_dependencies();
        let info = mock_info(CREATOR, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg()).unwrap();

        let msg_1 = MintMsg {
            token_id: String::from("0"),
            owner: CREATOR.to_string(),
            token_uri: None,
            extension: None,
        };

        let exec_store = ExecuteMsg::Store(msg_1);

        let res = execute(deps.as_mut(), mock_env(), info.clone(), exec_store).unwrap();

        assert_eq!(res.attributes[0].value, "store");

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("0"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();

        assert_eq!(res.owner, CREATOR);

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("0"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();

        assert_eq!(res.owner, CREATOR);
    }

    #[test]
    fn store_batch() {
        let mut deps = mock_dependencies();
        let info = mock_info(CREATOR, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg()).unwrap();

        // store batch
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::StoreBatch(get_store_batch_msg())
        ).unwrap();

        assert_eq!(res.attributes[0].value, "store_batch");
        assert_eq!(res.attributes[1].key, "token_total");
        assert_eq!(res.attributes[1].value, "20");

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("0"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();

        assert_eq!(res.owner, CREATOR);

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("1"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();

        assert_eq!(res.owner, CREATOR);
    }

    #[test]
    fn mint(){
        let mut deps = mock_dependencies();
        let info = mock_info(CREATOR, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg()).unwrap();

        // store batch
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(CREATOR, &[]),
            ExecuteMsg::StoreBatch(get_store_batch_msg())
        ).unwrap();

        let exec_mint = ExecuteMsg::Mint();

        let info = mock_info(MINTER, &[
            Coin::new(4000000u128, DENOM.to_string())
        ]);

        let mut env = mock_env();
        env.block.time = _now().plus_seconds(120);

        execute(deps.as_mut(), env, info, exec_mint).unwrap();

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("0"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();

        assert_eq!(res.owner, MINTER);
    }


    #[test]
    fn mint_batch(){
        let mut deps = mock_dependencies();
        let info = mock_info(CREATOR, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg()).unwrap();

        // store batch
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(CREATOR, &[]),
            ExecuteMsg::StoreBatch(get_store_batch_msg())
        ).unwrap();

        // MINTING 10 TOKENS
        let exec_mint = ExecuteMsg::MintBatch(BatchMintMsg {
            amount: Uint128::from(10u32)
        });

        let mut env = mock_env();
        env.block.time = _now();

        // SEND EXACT AMOUNT FOR IT TO ACCEPT THE TRANSACTION
        execute(deps.as_mut(), env, mock_info(MINTER, &[
            Coin::new(40000000u128, DENOM.to_string())
        ]), exec_mint).unwrap();

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("1"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();

        println!("prints: {} {}", res.owner, MINTER);

        assert_eq!(res.owner, MINTER);
    }

    #[test]
    #[should_panic(expected = "NoFundsSent")]
    fn mint_no_funds() {
        let mut deps = mock_dependencies();
        let info = mock_info(CREATOR, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg()).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Store(get_mint_msg(String::from("0")))
        ).unwrap();

        let exec_mint = ExecuteMsg::Mint();

        let info = mock_info(MINTER, &[]);

        let mut env = mock_env();
        env.block.time = _now().plus_seconds(120);

        execute(deps.as_mut(), env, info, exec_mint).unwrap();
    }

    #[test]
    #[should_panic(expected = "NotEnoughFunds")]
    fn mint_wrong_funds() {
        let mut deps = mock_dependencies();
        let info = mock_info(CREATOR, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg()).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Store(get_mint_msg(String::from("0")))
        ).unwrap();

        let info = mock_info(MINTER, &[
            Coin::new(2000000u128, DENOM.to_string())
        ]);

        let mut env = mock_env();
        env.block.time = _now().plus_seconds(120);

        execute(deps.as_mut(), env, info, ExecuteMsg::Mint()).unwrap();
    }

    #[test]
    #[should_panic(expected = "CantMintYet")]
    fn mint_wrong_time() {
        let mut deps = mock_dependencies();
        let info = mock_info(CREATOR, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg()).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Store(get_mint_msg(String::from("0")))
        ).unwrap();

        let info = mock_info(MINTER, &[
            Coin::new(4000000u128, DENOM.to_string())
        ]);

        let mut env = mock_env();
        // NOTE: this makes it fail ;)
        env.block.time = _now().minus_seconds(300);

        execute(deps.as_mut(), env, info, ExecuteMsg::Mint()).unwrap();
    }

    #[test]
    fn burn() {
        let mut deps = mock_dependencies();
        let info = mock_info(CREATOR, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg()).unwrap();

        // store batch
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::StoreBatch(get_store_batch_msg())
        ).unwrap();

        assert_eq!(res.attributes[0].value, "store_batch");
        assert_eq!(res.attributes[1].key, "token_total");
        assert_eq!(res.attributes[1].value, "20");

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("0"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();

        assert_eq!(res.owner, CREATOR);

        let exec_mint = ExecuteMsg::MintBatch(BatchMintMsg {
            amount: Uint128::from(10u32)
        });

        let mut env = mock_env();
        env.block.time = _now();

        // SEND EXACT AMOUNT FOR IT TO ACCEPT THE TRANSACTION
        execute(deps.as_mut(), env, mock_info(MINTER, &[
            Coin::new(40000000u128, DENOM.to_string())
        ]), exec_mint).unwrap();

        // BURN MSG IS SENT WITH ORIGINAL CREATOR INFO ABOVE
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Burn{ token_id: String::from("0") }
        ).unwrap();

        assert_eq!(res.attributes[0].value, "burn");
        assert_eq!(res.attributes[1].value, "minter_burn");
    }

    #[test]
    fn owner_burn() {
        let mut deps = mock_dependencies();
        let mut info = mock_info(CREATOR, &[]);

        let mut init_msg = get_init_msg();

        // ENABLE OWNERS BURN
        init_msg.owners_can_burn = true;

        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            init_msg
        ).unwrap();

        // store batch
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::StoreBatch(get_store_batch_msg())
        ).unwrap();

        assert_eq!(res.attributes[0].value, "store_batch");
        assert_eq!(res.attributes[1].key, "token_total");
        assert_eq!(res.attributes[1].value, "20");

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("0"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();

        assert_eq!(res.owner, CREATOR);

        let exec_mint = ExecuteMsg::MintBatch(BatchMintMsg {
            amount: Uint128::from(10u32)
        });

        let mut env = mock_env();
        env.block.time = _now();

        // SEND EXACT AMOUNT FOR IT TO ACCEPT THE TRANSACTION
        execute(deps.as_mut(), env, mock_info(MINTER, &[
            Coin::new(40000000u128, DENOM.to_string())
        ]), exec_mint).unwrap();

        // CHANGE ORIGINAL SENDER TO MINTER
        info.sender = Addr::unchecked(MINTER);

        // BURN BABY BURN
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Burn{ token_id: String::from("0") }
        ).unwrap();

        assert_eq!(res.attributes[0].value, "burn");
        assert_eq!(res.attributes[1].value, "owner_burn");
    }

    #[test]
    fn query_nft_info_batch() {
        QueryMsg::NftInfoBatch{
            tokens: vec![
                String::from("0"),
                String::from("1"),
            ]
        };
    }


    #[test]
    fn query_burned() {
        QueryMsg::BurntList{
            address: Addr::unchecked(FUNDWALLET)
        };
    }
}

// TODO: Add store conf tests
// arg
