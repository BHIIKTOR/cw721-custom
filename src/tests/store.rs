#[cfg(test)]
mod general {
    use cw_multi_test::App;
    use roboto::{Roboto, RobotoContractData};
    use crate::error::ContractError;
    use crate::msg::{StoreConfMsg, StoreConf, InstantiateMsg};
    use crate::tests::test_helpers::tests_helpers::{nft_custom_contract, get_mint_msg};
    use crate::{
        msg::ExecuteMsg,
        tests::test_helpers::tests_helpers::{
            get_store_batch_msg,
            get_init_msg,
        }
    };

    const ADMIN: &str = "admin";
    const NFT_CUSTOM: &str = &"nft_custom";

    #[test]
    fn store() {
        let init_msg = get_init_msg(0, 900);

        let mut roboto = Roboto::new(App::default(), ADMIN.to_string());

        let init = RobotoContractData::<InstantiateMsg>::new(
            nft_custom_contract,
            init_msg
        );

        let store_one = ExecuteMsg::Store(get_mint_msg("0".to_string()));

        let store_batch = ExecuteMsg::StoreBatch(get_store_batch_msg(20));

        let store_conf = ExecuteMsg::StoreConf(StoreConfMsg {
            conf: Some(StoreConf {
                name: String::from("nft"),
                desc: String::from("nft"),
                ipfs: String::from("nft"),
                attributes: vec![String::from("value"), String::from("something")],
            }),
            attributes: vec![
                vec![String::from("value"), String::from("something")],
                vec![String::from("value"), String::from("something")]
            ],
        });

        roboto
            .init(NFT_CUSTOM, init)
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, store_one, Some(|res| {
                assert_eq!(res.unwrap().events[1].attributes[1].value, "store");
            }))
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, store_batch, Some(|res| {
                assert_eq!(res.unwrap().events[1].attributes[1].value, "store_batch");
            }))
            .exec::<ExecuteMsg, ContractError>(NFT_CUSTOM, store_conf, Some(|res| {
                assert_eq!(res.unwrap().events[1].attributes[1].value, "store_conf");
            }));
    }
}
