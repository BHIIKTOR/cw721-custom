#[cfg(test)]
mod general {
    use cosmwasm_std::{
        from_binary,
        Uint128,
        Coin,
        Timestamp,
        Addr,
    };
    // use cw20::Cw20Coin;
    use cosmwasm_std::testing::{
        mock_dependencies,
        mock_env,
        mock_info,
    };
    use cw_multi_test::{App, Executor};
    use roboto::{Roboto, RobotoContractData};

    use crate::error::ContractError;
    // use crate::error::ContractError;
    use crate::msg::InstantiateMsg;
    use crate::tests::test_helpers::tests_helpers::nft_custom_contract;
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
            get_mint_msg,
        }
    };

    use cw721::{
        OwnerOfResponse
    };

    const ADMIN: &str       = "admin";
    const MINTER: &str      = "minter";
    const DENOM: &str       = "ujuno";

    const NFT_CUSTOM: &str  = &"nft_custom";

    #[test]
    fn testo() {
        let mut app = App::default();
        let code_id = app.store_code(nft_custom_contract());
        let init_msg = get_init_msg(0, 12000000000);
        let sender = Addr::unchecked(ADMIN.to_string());
        let contract_addr = app.instantiate_contract(
                code_id,
                sender.clone(),
                &init_msg,
                &vec![],
                "NFT",
                Some(ADMIN.to_string())
            ).unwrap();
        let store_batch = ExecuteMsg::StoreBatch(get_store_batch_msg(1));
        let res_batch = app.execute_contract(sender, contract_addr.clone(), &store_batch, &vec![]).unwrap();
        println!("res_batch: {:#?}", res_batch);
        let exec_mint = ExecuteMsg::Mint();
        let res_mint = app.execute_contract::<ExecuteMsg>(
                Addr::unchecked(MINTER.to_string()),
                contract_addr,
                &exec_mint,
                &[Coin::new(4000000u128, DENOM.to_string())]
            );
        println!("res_mint:{:#?}", res_mint);
        assert!(false)
    }

    #[test]
    fn roboto_mint() {
        let mut init_msg = get_init_msg(0, 12000000000);
        init_msg.max_mint_batch = Some(Uint128::from(10u128));

        let mut roboto = Roboto::new(App::default(), ADMIN.to_string());

        let init_custom = RobotoContractData::<InstantiateMsg>::new(
            nft_custom_contract,
            init_msg
        );

        let store_batch = ExecuteMsg::StoreBatch(get_store_batch_msg(40));

        let exec_mint = ExecuteMsg::Mint();

        let exec_mint_batch = ExecuteMsg::MintBatch(MintBatchMsg {
            amount: Uint128::from(10u32)
        });

        let exec_mint_incorrect_funds = ExecuteMsg::MintBatch(MintBatchMsg {
            amount: Uint128::from(10u32)
        });

        let exec_mint_too_large = ExecuteMsg::MintBatch(MintBatchMsg {
            amount: Uint128::from(11u32)
        });

        roboto
            .set_sender(ADMIN.to_string())
            .add_balance(MINTER, vec![Coin::new(1000000000u128, DENOM.to_string())])
            .init(NFT_CUSTOM, init_custom)
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, store_batch, Some(|res| {
                assert_eq!(res.unwrap().events[1].attributes[1].value, "store_batch");
            }))
            .set_block(|block| {
                block.time = Timestamp::from_nanos(0);
                block
            })
            .set_funds(Some(&vec![Coin::new(4000000u128, DENOM.to_string())]))
            .set_sender(MINTER.to_string())
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint.clone(), Some(|res| {
                assert_eq!(res.unwrap().events[1].attributes[1].value, "mint");
            }))
            .set_funds(Some(&vec![Coin::new(40000000u128, DENOM.to_string())]))
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint_batch.clone(), Some(|res| {
                assert_eq!(res.unwrap().events[1].attributes[1].value, "mint_batch");
            }))
            .set_funds(Some(&vec![Coin::new(40000000u128, DENOM.to_string())]))
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint_too_large, Some(|res| {
                assert!(res.unwrap_err().eq(&ContractError::MintAmountLargerThanAllowed{}))
            }))
            .set_funds(Some(&vec![Coin::new(80000000u128, DENOM.to_string())]))
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint_incorrect_funds, Some(|res| {
                assert!(res.unwrap_err().eq(&ContractError::IncorrectFunds{}))
            }))
            .set_funds(Some(&vec![]))
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint, Some(|res| {
                assert!(res.unwrap_err().eq(&ContractError::NoFundsSent{}))
            }))
            .set_funds(Some(&vec![Coin::new(20000000u128, DENOM.to_string())]))
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint_batch, Some(|res| {
                assert!(res.unwrap_err().eq(&ContractError::NotEnoughFunds{}))
            }));
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
        msg.dates.end = None;
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
        msg.dates.start = None;
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
}

// mod breaking {
//   #[test]
//   fn missing() {
//     assert!(false)
//   }
// }