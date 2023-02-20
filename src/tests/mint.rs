#[cfg(test)]
mod general {
  use cosmwasm_std::{
      from_binary,
      Uint128,
      Coin,
      Timestamp,
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
          get_mint_msg,
      }
  };

  use cw721::{
      OwnerOfResponse
  };

  const ADMIN: &str = "admin";
  const MINTER: &str = "minter";
  const DENOM: &str = "ujuno";

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
      let exec_mint = ExecuteMsg::MintBatch(MintBatchMsg {
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
      let exec_mint = ExecuteMsg::MintBatch(MintBatchMsg {
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