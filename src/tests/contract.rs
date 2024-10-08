#[cfg(test)]
mod general {
    use cosmwasm_std::{
        Uint128,
        Response,
    };

    use cosmwasm_std::testing::{
        mock_dependencies,
        mock_env,
        mock_info,
    };

    use crate::types_mint;
    use crate::{
        contract::{
            execute,
            instantiate
        },
        msg::{
            ExecuteMsg
        },
        tests::test_helpers::tests_helpers::{
            get_init_msg,
        }
    };

    const ADMIN: &str = "admin";

    #[test]
    fn update_conf() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(0, 900)).unwrap();

        // store batch
        let res: Response = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateConf(crate::msg::InstantiateMsg {
                creator: String::from(ADMIN),
                name: String::from("nft2"),
                symbol: String::from("NFT2"),
                dates: types_mint::Dates::default(),
                cost: types_mint::Costs::default(),
                burn: types_mint::Burn::default(),
                token_supply: Default::default(),
                wallet: types_mint::Wallet::default(),
                max_mint_batch: Some(Uint128::from(8u32)),
                store_conf: Default::default(),
            })
        ).unwrap();

       assert_eq!(res.attributes[0].value, "config");
       assert_eq!(res.attributes[1].value, "update");
       assert_eq!(res.attributes[2].value, "success");
    }

    #[test]
    #[should_panic(expected = "ContractFrozen")]
    fn update_conf_frozen() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);
        let init_msg = get_init_msg(0, 900);

        instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        let msg = ExecuteMsg::Freeze();

        let res: Response = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        assert_eq!(res.attributes[0].value, "freeze");

        // store batch
        let res: Response = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateConf(crate::msg::InstantiateMsg {
                creator: String::from(ADMIN),
                name: String::from("nft2"),
                symbol: String::from("NFT2"),
                dates: types_mint::Dates::default(),
                cost: types_mint::Costs::default(),
                burn: types_mint::Burn::default(),
                token_supply: Default::default(),
                wallet: types_mint::Wallet::default(),
                max_mint_batch: Some(Uint128::from(8u32)),
                store_conf: Default::default(),
            })
        ).unwrap();

       assert_eq!(res.attributes[0].value, "config");
       assert_eq!(res.attributes[1].value, "update");
       assert_eq!(res.attributes[2].value, "success");
    }

    #[test]
    fn pause() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(0, 900)).unwrap();

        let msg = ExecuteMsg::Pause();

        let res: Response = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert_eq!(res.attributes[0].value, "pause");
    }

    #[test]
    fn unpause() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(0, 900)).unwrap();

        let msg = ExecuteMsg::Unpause();

        let res: Response = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert_eq!(res.attributes[0].value, "unpause");
    }

    #[test]
    fn freeze() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(0, 900)).unwrap();

        let msg = ExecuteMsg::Freeze();

        let res: Response = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert_eq!(res.attributes[0].value, "freeze");
    }

    #[test]
    fn unfreeze() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), get_init_msg(0, 900)).unwrap();

        let msg = ExecuteMsg::Unfreeze();

        let res: Response = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert_eq!(res.attributes[0].value, "unfreeze");
    }
}

// #[cfg(test)]
// mod breaking {
//     use cosmwasm_std::{
//         Uint128,
//         Response,
//     };

//     use cosmwasm_std::testing::{
//         mock_dependencies,
//         mock_env,
//         mock_info,
//     };

//     use crate::mint;
//     use crate::{
//         contract::{
//             execute,
//             instantiate
//         },
//         msg::{
//             ExecuteMsg
//         },
//         tests::helpers::tests_helpers::{
//             get_init_msg,
//         }
//     };

//     const ADMIN: &str = "admin";

//     #[test]
//     fn init_contract() {
//         let mut deps = mock_dependencies();
//         let info = mock_info(ADMIN, &[]);
//         let mut msg = get_init_msg(0, 900);

//         msg.burn.admin = Some(ADMIN.to_string());
//         msg.burn.owners = true;

//         msg.burn.both_can_burn = Some(true);

//         instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
//     }
// }