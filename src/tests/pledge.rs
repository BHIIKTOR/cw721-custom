#[cfg(test)]
mod general {
    use cosmwasm_std::{
        from_binary,
        Uint128,
        Coin,
        Addr,
    };

    use cosmwasm_std::testing::{
        mock_dependencies,
        mock_env,
        mock_info,
    };

    use crate::{
        contract::{
            execute,
            instantiate,
            query
        },
        msg::{
            ExecuteMsg,
            QueryMsg,
            MintBatchMsg,
        },
        tests::test_helpers::tests_helpers::{
            now,
            get_init_msg,
            get_store_batch_msg
        }
    };

    use cw721::{
        OwnerOfResponse
    };

    const ADMIN: &str = "admin";
    const MINTER: &str = "minter";
    const DENOM: &str = "ujuno";

    #[test]
    fn pledge() {
        let mut deps = mock_dependencies();
        let mut info = mock_info(ADMIN, &[]);

        let mut init_msg = get_init_msg(0,0);

        // ENABLE OWNERS BURN
        init_msg.burn.owner_can_burn = true;

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
            ExecuteMsg::StoreBatch(get_store_batch_msg(20))
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

        let exec_mint = ExecuteMsg::MintBatch(MintBatchMsg {
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

        // PLEDGE
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Pledge { tokens: vec![String::from("0")] }
        ).unwrap();

        assert_eq!(res.attributes[0].value, "pledge");
        assert_eq!(res.attributes[1].key, "list");
        assert_eq!(res.attributes[1].value, "[\"0\"]");
    }

    #[test]
    fn already_pledged() {
        let mut deps = mock_dependencies();
        let mut info = mock_info(ADMIN, &[]);

        let mut init_msg = get_init_msg(0,0);

        // ENABLE OWNERS BURN
        init_msg.burn.owner_can_burn = true;

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
            ExecuteMsg::StoreBatch(get_store_batch_msg(20))
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

        let exec_mint = ExecuteMsg::MintBatch(MintBatchMsg {
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

        // PLEDGE
        execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Pledge { tokens: vec![String::from("0")] }
        ).unwrap();

        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Pledge { tokens: vec![String::from("0")] }
        );

        assert!(res.is_err())
    }

    #[test]
    fn pledge_add_more() {
        let mut deps = mock_dependencies();
        let mut info = mock_info(ADMIN, &[]);

        let mut init_msg = get_init_msg(0,0);

        // ENABLE OWNERS BURN
        init_msg.burn.owner_can_burn = true;

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
            ExecuteMsg::StoreBatch(get_store_batch_msg(20))
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

        let exec_mint = ExecuteMsg::MintBatch(MintBatchMsg {
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

        // PLEDGE
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Pledge { tokens: vec![String::from("0")] }
        ).unwrap();

        assert_eq!(res.attributes[0].value, "pledge");
        assert_eq!(res.attributes[1].key, "list");
        assert_eq!(res.attributes[1].value, "[\"0\"]");

        // PLEDGE
        let res = execute(
          deps.as_mut(),
          mock_env(),
          info.clone(),
          ExecuteMsg::Pledge { tokens: vec![String::from("1")] }
      ).unwrap();

      assert_eq!(res.attributes[0].value, "pledge");
      assert_eq!(res.attributes[1].key, "list");
      assert_eq!(res.attributes[1].value, "[\"1\", \"0\"]");
    }

    #[test]
    fn pledge_dont_exists() {
        let mut deps = mock_dependencies();
        let mut info = mock_info(ADMIN, &[]);

        let mut init_msg = get_init_msg(0,0);

        // ENABLE OWNERS BURN
        init_msg.burn.owner_can_burn = true;

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
            ExecuteMsg::StoreBatch(get_store_batch_msg(20))
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

        let exec_mint = ExecuteMsg::MintBatch(MintBatchMsg {
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

        // PLEDGE
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Pledge { tokens: vec![String::from("22")] }
        );

        assert!(res.is_err())
    }


    #[test]
    fn pledge_no_auth() {
        let mut deps = mock_dependencies();
        let mut info = mock_info(ADMIN, &[]);

        let mut init_msg = get_init_msg(0,0);

        // ENABLE OWNERS BURN
        init_msg.burn.owner_can_burn = true;

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
            ExecuteMsg::StoreBatch(get_store_batch_msg(20))
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

        let exec_mint = ExecuteMsg::MintBatch(MintBatchMsg {
            amount: Uint128::from(5u32)
        });

        let mut env = mock_env();
        env.block.time = now();

        // SEND EXACT AMOUNT FOR IT TO ACCEPT THE TRANSACTION
        execute(deps.as_mut(), env, mock_info(MINTER, &[
            Coin::new(20000000u128, DENOM.to_string())
        ]), exec_mint).unwrap();

        // CHANGE ORIGINAL SENDER TO MINTER
        info.sender = Addr::unchecked(MINTER);

        // PLEDGE
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Pledge { tokens: vec![String::from("18")] }
        );

        assert!(res.is_err())
    }
}