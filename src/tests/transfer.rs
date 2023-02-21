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
            get_store_batch_msg,
        }
    };

    use cw721::{
        OwnerOfResponse
    };

    const ADMIN: &str = "admin";
    const MINTER: &str = "minter";
    const DENOM: &str = "ujuno";

    #[test]
    fn transfer_nft() {
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

        // BURN BABY BURN
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::TransferNft { recipient: ADMIN.to_string(), token_id: String::from("0") }
        ).unwrap();

        assert_eq!(res.attributes[0].value, "transfer_nft");
        assert_eq!(res.attributes[1].value, "minter");
        assert_eq!(res.attributes[2].value, "admin");
        assert_eq!(res.attributes[3].value, "0");
    }

    #[test]
    fn transfer_nft_batch() {
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

        // BURN BABY BURN
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::TransferBatch(crate::msg::TransferOperation {
                recipient: ADMIN.to_string(),
                tokens: vec!["0".to_string(), "1".to_string(), "8".to_string()],
            })
        ).unwrap();

        assert_eq!(res.attributes[0].value, "transfer_batch");
        assert_eq!(res.attributes[1].value, "minter");
        assert_eq!(res.attributes[2].value, "admin");
        assert_eq!(res.attributes[3].value, "[\"0\", \"1\", \"8\"]");
    }


    #[test]
    #[should_panic(expected = "Unauthorized")]
    fn transfer_nft_batch_error_no_auth() {
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

        // BURN BABY BURN
        execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::TransferBatch(crate::msg::TransferOperation {
                recipient: ADMIN.to_string(),
                tokens: vec!["8".to_string(), "9".to_string()],
            })
        ).unwrap();
    }

    #[test]
    fn transfer_nft_batch_error_dont_exists() {
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

        // BURN BABY BURN
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::TransferBatch(crate::msg::TransferOperation {
                recipient: ADMIN.to_string(),
                tokens: vec!["22".to_string()],
            })
        );

        assert!(res.is_err())

        // assert_eq!(res.attributes[3].value, "[\"22, cw721_base::state::TokenInfo<core::option::Option<cw721_custom::state::Metadata>> not found\"]")
    }
}