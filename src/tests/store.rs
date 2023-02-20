#[cfg(test)]
mod general {
    use cosmwasm_std::{
        from_binary,
    };

    use cosmwasm_std::testing::{
        mock_dependencies,
        mock_env,
        mock_info,
    };

    use cw721_base::MintMsg;

    use crate::msg::{StoreConfMsg, StoreConf};
    use crate::{
        contract::{
            execute,
            instantiate,
            query
        },
        msg::{
            ExecuteMsg,
            QueryMsg,
        },
        tests::test_helpers::tests_helpers::{
            get_store_batch_msg,
            get_init_msg,
        }
    };

    use cw721::{
        OwnerOfResponse
    };

    const ADMIN: &str = "admin";

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
    fn store_conf() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(0, 900)).unwrap();

        let conf = Some(StoreConf {
            name: String::from("nft"),
            desc: String::from("nft"),
            ipfs: String::from("nft"),
            attributes: vec![String::from("value"), String::from("something")],
        });

        let msg = ExecuteMsg::StoreConf(StoreConfMsg {
            attributes: vec![vec![String::from("value"), String::from("something")]],
            conf,
        });

        // store conf
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            msg
        ).unwrap();

        assert_eq!(res.attributes[0].value, "store_conf");
        assert_eq!(res.attributes[1].key, "token_total");
        assert_eq!(res.attributes[1].value, "1");
    }
}
