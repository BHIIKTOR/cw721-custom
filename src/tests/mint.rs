#[cfg(test)]
mod general {
    use cosmwasm_std::{
        Uint128,
        Coin,
        Timestamp,
    };

    use cosmwasm_std::testing::{
        mock_dependencies,
        mock_env,
        mock_info,
    };
    use cw_multi_test::{App};
    use roboto::{Roboto, RobotoContractData};

    use crate::error::ContractError;
    use crate::msg::InstantiateMsg;
    use crate::tests::test_helpers::tests_helpers::nft_custom_contract;

    use crate::{
        contract::{
            execute,
            instantiate,
        },
        msg::{
            ExecuteMsg,
            MintBatchMsg,
        },
        tests::test_helpers::tests_helpers::{
            get_init_msg,
            get_store_batch_msg,
            get_mint_msg,
        }
    };

    const ADMIN: &str       = "admin";
    const MINTER: &str      = "minter";
    const DENOM: &str       = "ujuno";

    const NFT_CUSTOM: &str  = &"nft_custom";

    #[test]
    fn roboto_mint() {
        let mut init_msg = get_init_msg(0, 1);
        init_msg.max_mint_batch = Some(Uint128::from(10u128));

        let mut roboto = Roboto::new(App::default(), ADMIN.to_string());

        let mut init_custom = RobotoContractData::<InstantiateMsg>::new(
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

        let balance = vec![
            Roboto::create_balance(MINTER, vec![Coin::new(1000000000u128, DENOM.to_string())])
        ];

        roboto
            // set sender to admin
            .set_sender(ADMIN.to_string())
            // add balances
            .add_balance(balance)
            // init contract
            .init(NFT_CUSTOM, init_custom.clone())
            // exec store_batch
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, store_batch.clone(), Some(|res| {
                assert_eq!(res.unwrap().events[1].attributes[1].value, "store_batch");
            }))
            // set funds for the cost of 1 mint
            .set_funds(Some(&vec![Coin::new(4000000u128, DENOM.to_string())]))
            // set sender to minter
            .set_sender(MINTER.to_string())
            // change block time so we can mint
            .set_block(|block| {
                block.time = Timestamp::from_nanos(0);
                block
            })
            // exec mint
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint.clone(), Some(|res| {
                assert_eq!(res.unwrap().events[1].attributes[1].value, "mint");
            }))
            // set funds for the cost of 10 mints
            .set_funds(Some(&vec![Coin::new(40000000u128, DENOM.to_string())]))
            // exec mint_batch
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint_batch.clone(), Some(|res| {
                assert_eq!(res.unwrap().events[1].attributes[1].value, "mint_batch");
            }))
            // validate error when attempting to mint more than allowed per batch
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint_too_large, Some(|res| {
                assert!(res.unwrap_err().eq(&ContractError::MintAmountLargerThanAllowed{}))
            }))
            // set funds to more than needed for the mint amount
            .set_funds(Some(&vec![Coin::new(80000000u128, DENOM.to_string())]))
            // exec mint_batch with incorrenct funds
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint_incorrect_funds, Some(|res| {
                assert!(res.unwrap_err().eq(&ContractError::IncorrectFunds{}))
            }))
            // set funds to empty
            .set_funds(Some(&vec![]))
            // validate error when funds are empty
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint.clone(), Some(|res| {
                assert!(res.unwrap_err().eq(&ContractError::NoFundsSent{}))
            }))
            // set funds to less than required for the mint amount
            .set_funds(Some(&vec![Coin::new(20000000u128, DENOM.to_string())]))
            // exec mint_batch
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint_batch, Some(|res| {
                assert!(res.unwrap_err().eq(&ContractError::NotEnoughFunds{}))
            }))
            // change block time to after mint end date
            .set_block(|block| {
                block.time = Timestamp::from_nanos(2000000000);
                block
            })
            // set correct funds amount
            .set_funds(Some(&vec![Coin::new(4000000u128, DENOM.to_string())]))
            // validate error mint ended
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint.clone(), Some(|res| {
                assert!(res.unwrap_err().eq(&ContractError::MintEnded{}))
            }))
            // change mint start date
            .step(&mut |roboto| {
                init_custom.msg.dates.start = Some(Timestamp::from_nanos(100));
                roboto
            })
            // restart contract
            .init(NFT_CUSTOM, init_custom)
            // test mint, should error since there is nothing in the storage
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint.clone(), Some(|res| {
                assert!(res.unwrap_err().eq(&ContractError::CantMintNothing{}))
            }))
            // change block
            .set_block(|block| {
                block.time = Timestamp::from_nanos(0);
                block
            })
            // set sender to admin
            .set_sender(ADMIN.to_string())
            // send store_batch exec
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, store_batch, Some(|res| {
                assert_eq!(res.unwrap().events[1].attributes[1].value, "store_batch");
            }))
            // set sender to minter
            .set_sender(MINTER.to_string())
            // set funds
            .set_funds(Some(&vec![Coin::new(4000000u128, DENOM.to_string())]))
            // test cant mint yet error
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, exec_mint, Some(|res| {
                assert!(res.unwrap_err().eq(&ContractError::CantMintYet{}))
            }));
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