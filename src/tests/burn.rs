// TODO: Add break things tests
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
        tests::helpers::tests_helpers::{
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
    #[should_panic(expected = "Unauthorized")]
    fn admin_cant_burn_owned() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        let mut init_msg = get_init_msg(0,0);

        init_msg.burn.owners = false;
        init_msg.burn.admin = Some(ADMIN.to_string());

        instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

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

        let exec_mint = ExecuteMsg::MintBatch(MintBatchMsg {
            amount: Uint128::from(10u32)
        });

        let mut env = mock_env();
        env.block.time = now();

        // SEND EXACT AMOUNT FOR IT TO ACCEPT THE TRANSACTION
        execute(deps.as_mut(), env, mock_info(MINTER, &[
            Coin::new(40000000u128, DENOM.to_string())
        ]), exec_mint).unwrap();

        // BURN MSG IS SENT WITH ORIGINAL CREATOR INFO ABOVE
        execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Burn{ token_id: String::from("0") }
        ).unwrap();
    }

    #[test]
    fn admin_can_burn_owned() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        let mut init_msg = get_init_msg(0,0);

        init_msg.burn.owners = false;
        init_msg.burn.can_burn_owned = true;
        init_msg.burn.admin = Some(ADMIN.to_string());

        instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

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

        let exec_mint = ExecuteMsg::MintBatch(MintBatchMsg {
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
        assert_eq!(res.attributes[1].value, "admin_burn");
    }

    #[test]
    fn owners_burn() {
        let mut deps = mock_dependencies();
        let mut info = mock_info(ADMIN, &[]);

        let mut init_msg = get_init_msg(0,0);

        // ENABLE OWNERS BURN
        init_msg.burn.owners = true;

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
            ExecuteMsg::Burn{ token_id: String::from("0") }
        ).unwrap();

        assert_eq!(res.attributes[0].value, "burn");
        assert_eq!(res.attributes[1].value, "owner_burn");
    }

    #[test]
    fn both_can_burn() {
        let mut deps = mock_dependencies();
        let mut info = mock_info(ADMIN, &[]);
        let mut msg = get_init_msg(0, 900);

        msg.burn.admin = Some(ADMIN.to_string());
        msg.burn.owners = true;
        msg.burn.both_can_burn = Some(true);
        msg.burn.can_burn_owned = true;

        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

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

        let exec_mint = ExecuteMsg::MintBatch(MintBatchMsg {
            amount: Uint128::from(10u32)
        });

        let mut env = mock_env();
        env.block.time = now();

        // SEND EXACT AMOUNT FOR IT TO ACCEPT THE TRANSACTION
        execute(deps.as_mut(), env.clone(), mock_info(MINTER, &[
            Coin::new(40000000u128, DENOM.to_string())
        ]), exec_mint.clone()).unwrap();

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

        // SEND EXACT AMOUNT FOR IT TO ACCEPT THE TRANSACTION
        // let res = execute(deps.as_mut(), env, mock_info(ADMIN, &[
        //     Coin::new(40000000u128, DENOM.to_string())
        // ]), exec_mint);

        let mut info = mock_info(ADMIN, &[]);
        info.sender = Addr::unchecked(ADMIN);

        // BURN BABY BURN
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Burn{ token_id: String::from("1") }
        ).unwrap();

        assert_eq!(res.attributes[0].value, "burn");
        assert_eq!(res.attributes[1].value, "admin_burn");

    }


    #[test]
    #[should_panic(expected = "GenericErr")]
    fn both_cant_burn() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);
        let mut msg = get_init_msg(0, 900);

        msg.burn.admin = Some(ADMIN.to_string());
        msg.burn.owners = false;

        msg.burn.both_can_burn = Some(true);

        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    }

}

// mod breaking {
//   #[test]
//   fn missing() {
//     assert!(false)
//   }
// }
