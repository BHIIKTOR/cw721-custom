#[cfg(test)]
mod general_tests {
    use cosmwasm_std::{
        from_binary,
        Uint128,
        Coin,
        Addr,
        Timestamp,
    };

    use cosmwasm_std::testing::{
        mock_dependencies,
        mock_env,
        mock_info,
    };

    use cw721_base::MintMsg;

    use crate::contract::{
        execute,
        instantiate,
        query
    };

    use crate::msg::{
        ExecuteMsg,
        QueryMsg,
        BatchMintMsg,
    };

    use cw721::{
        OwnerOfResponse
    };

    use crate::tests::helpers::tests_helpers::{
        now,
        get_init_msg,
        get_store_batch_msg,
        get_mint_msg,
    };

    const ADMIN: &str = "admin";
    const MINTER: &str = "minter";

    const DENOM: &str = "uluna";

    #[test]
    fn store() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(0, 900)).unwrap();

        let msg_1 = MintMsg {
            token_id: String::from("0"),
            owner: ADMIN.to_string(),
            token_uri: None,
            extension: None,
        };

        let exec_store = ExecuteMsg::Store(msg_1);

        let res = execute(deps.as_mut(), mock_env(), info, exec_store).unwrap();

        assert_eq!(res.attributes[0].value, "store");

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("0"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();

        assert_eq!(res.owner, ADMIN);

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("0"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();

        assert_eq!(res.owner, ADMIN);
    }

    #[test]
    fn store_batch() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(0, 900)).unwrap();

        // store batch
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
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

        assert_eq!(res.owner, ADMIN);

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("1"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();

        assert_eq!(res.owner, ADMIN);
    }

    #[test]
    fn mint(){
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info, get_init_msg(0, 900)).unwrap();

        // store batch
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::StoreBatch(get_store_batch_msg())
        ).unwrap();

        let exec_mint = ExecuteMsg::Mint();

        let info = mock_info(MINTER, &[
            Coin::new(4000000u128, DENOM.to_string())
        ]);

        let mut env = mock_env();
        env.block.time = now().plus_seconds(120);

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
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info, get_init_msg(0, 900)).unwrap();

        // store batch
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::StoreBatch(get_store_batch_msg())
        ).unwrap();

        // MINTING 10 TOKENS
        let exec_mint = ExecuteMsg::MintBatch(BatchMintMsg {
            amount: Uint128::from(10u32)
        });

        let mut env = mock_env();
        env.block.time = now();

        // SEND EXACT AMOUNT FOR IT TO ACCEPT THE TRANSACTION
        execute(deps.as_mut(), env, mock_info(MINTER, &[
            Coin::new(40000000u128, DENOM.to_string())
        ]), exec_mint).unwrap();

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("1"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(res.owner, MINTER);
    }

    #[test]
    #[should_panic(expected = "MintAmountLargerThanAllowed")]
    fn mint_batch_amount_too_large(){
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        let mut msg = get_init_msg(0, 900);
        // make max_mint batch 5
        msg.max_mint_batch = Some(Uint128::from(5u128));

        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // store batch
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::StoreBatch(get_store_batch_msg())
        ).unwrap();

        // TRY TO MINT 10 TOKENS
        let exec_mint = ExecuteMsg::MintBatch(BatchMintMsg {
            amount: Uint128::from(10u32)
        });

        let mut env = mock_env();
        env.block.time = now();

        // SEND EXACT AMOUNT FOR IT TO ACCEPT THE TRANSACTION
        execute(deps.as_mut(), env, mock_info(MINTER, &[
            Coin::new(40000000u128, DENOM.to_string())
        ]), exec_mint).unwrap();

        let query_msg: QueryMsg = QueryMsg::OwnerOf {
            token_id: String::from("1"),
            include_expired: Some(true)
        };

        let res: OwnerOfResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(res.owner, MINTER);
    }

    #[test]
    #[should_panic(expected = "NoFundsSent")]
    fn mint_no_funds() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(0, 900)).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Store(get_mint_msg(String::from("0")))
        ).unwrap();

        let exec_mint = ExecuteMsg::Mint();

        let info = mock_info(MINTER, &[]);

        let mut env = mock_env();
        env.block.time = now().plus_seconds(120);

        execute(deps.as_mut(), env, info, exec_mint).unwrap();
    }

    #[test]
    #[should_panic(expected = "NotEnoughFunds")]
    fn mint_wrong_funds() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(0, 900)).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Store(get_mint_msg(String::from("0")))
        ).unwrap();

        let info = mock_info(MINTER, &[
            Coin::new(2000000u128, DENOM.to_string())
        ]);

        let mut env = mock_env();
        env.block.time = now().plus_seconds(120);

        execute(deps.as_mut(), env, info, ExecuteMsg::Mint()).unwrap();
    }

    #[test]
    #[should_panic(expected = "CantMintYet")]
    fn mint_scheduled_start_time() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(300, 900)).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Store(get_mint_msg(String::from("0")))
        ).unwrap();

        let info = mock_info(MINTER, &[
            Coin::new(4000000u128, DENOM.to_string())
        ]);

        let mut env = mock_env();
        // NOTE: this makes it fail ;)
        env.block.time = Timestamp::from_seconds(0);

        execute(deps.as_mut(), env, info, ExecuteMsg::Mint()).unwrap();
    }

    #[test]
    #[should_panic(expected = "MintEnded")]
    fn mint_scheduled_end_time() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(0, 200)).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Store(get_mint_msg(String::from("0")))
        ).unwrap();

        let info = mock_info(MINTER, &[
            Coin::new(4000000u128, DENOM.to_string())
        ]);

        let mut env = mock_env();
        // NOTE: this makes it fail ;)
        env.block.time = Timestamp::from_seconds(0).plus_seconds(300);

        execute(deps.as_mut(), env, info, ExecuteMsg::Mint()).unwrap();
    }

    #[test]
    fn mint_no_end_time() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);
        let mut msg = get_init_msg(0, 0);
        msg.end_mint = None;
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Store(get_mint_msg(String::from("0")))
        ).unwrap();

        let info = mock_info(MINTER, &[
            Coin::new(4000000u128, DENOM.to_string())
        ]);

        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(0).plus_seconds(3000);

        execute(deps.as_mut(), env, info, ExecuteMsg::Mint()).unwrap();
    }

    #[test]
    fn mint_no_start_time() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);
        let mut msg = get_init_msg(0, 300);
        msg.start_mint = None;
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Store(get_mint_msg(String::from("0")))
        ).unwrap();

        let info = mock_info(MINTER, &[
            Coin::new(4000000u128, DENOM.to_string())
        ]);

        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(0);

        execute(deps.as_mut(), env, info, ExecuteMsg::Mint()).unwrap();
    }

    #[test]
    fn burn() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(0,0)).unwrap();

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

        assert_eq!(res.owner, ADMIN);

        let exec_mint = ExecuteMsg::MintBatch(BatchMintMsg {
            amount: Uint128::from(10u32)
        });

        let mut env = mock_env();
        env.block.time = now();

        // SEND EXACT AMOUNT FOR IT TO ACCEPT THE TRANSACTION
        execute(deps.as_mut(), env, mock_info(MINTER, &[
            Coin::new(40000000u128, DENOM.to_string())
        ]), exec_mint).unwrap();

        // BURN MSG IS SENT WITH ORIGINAL CREATOR INFO ABOVE
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Burn{ token_id: String::from("0") }
        ).unwrap();

        assert_eq!(res.attributes[0].value, "burn");
        assert_eq!(res.attributes[1].value, "minter_burn");
    }

    #[test]
    fn owner_burn() {
        let mut deps = mock_dependencies();
        let mut info = mock_info(ADMIN, &[]);

        let mut init_msg = get_init_msg(0,0);

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

        assert_eq!(res.owner, ADMIN);

        let exec_mint = ExecuteMsg::MintBatch(BatchMintMsg {
            amount: Uint128::from(10u32)
        });

        let mut env = mock_env();
        env.block.time = now();

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
            info,
            ExecuteMsg::Burn{ token_id: String::from("0") }
        ).unwrap();

        assert_eq!(res.attributes[0].value, "burn");
        assert_eq!(res.attributes[1].value, "owner_burn");
    }

}

// TODO: Add store conf tests
// TODO: Add freeze test
// TODO: Add update conf test

