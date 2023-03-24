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

    // use cw721_base::MintMsg;

    use crate::msg::MigrateMsg;
    // use crate::msg::{StoreConfMsg, StoreConf, MigrateMsg, InstantiateMsg};
    use crate::state::Config;
    use crate::{
        types_mint,
        contract::{
            // execute,
            instantiate,
            // query,
            migrate
        },
        // msg::{
        //     ExecuteMsg,
        //     QueryMsg,
        // },
        tests::test_helpers::tests_helpers::{
            // get_store_batch_msg,
            get_init_msg,
        }
    };

    const ADMIN: &str = "admin";

    #[test]
    fn migrate_with_conf_and_clear_state() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            get_init_msg(0,0)
        ).unwrap();

        let config = Config {
            creator: String::from(ADMIN),
            name: String::from("nft2"),
            dates: types_mint::Dates::default(),
            cost: types_mint::Costs::default(),
            burn: types_mint::Burn::default(),
            token_supply: Default::default(),
            wallet: types_mint::Wallet::default(),
            max_mint_batch: Some(Uint128::from(8u32)),
            store_conf: Default::default(),
            token_total: Uint128::from(10000u32),
            frozen: false,
            paused: false,
        };

        let msg : MigrateMsg<Config> = MigrateMsg::WithConfigClearState {
          version: String::from("2.0.0"),
          config
        };

        let res: Response = migrate(deps.as_mut(), mock_env(), msg).unwrap();

        assert_eq!(res.attributes[0].value, String::from("migration"));
        assert_eq!(res.attributes[1].value, String::from("2.0.0"))
    }

    #[test]
    fn migrate_with_conf() {
        let mut deps = mock_dependencies();
        let info = mock_info(ADMIN, &[]);

        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            get_init_msg(0,0)
        ).unwrap();

        let config = Config {
            creator: String::from(ADMIN),
            name: String::from("nft2"),
            dates: types_mint::Dates::default(),
            cost: types_mint::Costs::default(),
            burn: types_mint::Burn::default(),
            token_supply: Default::default(),
            wallet: types_mint::Wallet::default(),
            max_mint_batch: Some(Uint128::from(8u32)),
            store_conf: Default::default(),
            token_total: Uint128::from(10000u32),
            frozen: false,
            paused: false,
        };

        let msg : MigrateMsg<Config> = MigrateMsg::WithConfig {
          version: String::from("2.0.0"),
          config
        };

        let res: Response = migrate(deps.as_mut(), mock_env(), msg).unwrap();

        assert_eq!(res.attributes[0].value, String::from("migration"));
        assert_eq!(res.attributes[1].value, String::from("2.0.0"))
    }
}
