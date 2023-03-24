use cosmwasm_schema::cw_serde;

use cw721::Expiration;

use cosmwasm_std::{Addr, Binary, Uint128, Empty};

use cw721_base::{
    msg::{
        ExecuteMsg as CW721ExecuteMsg,
        InstantiateMsg as CW721InstantiateMsg,
        QueryMsg as CW721QueryMsg,
    },
    MintMsg as CW721MintMsg,
};

use crate::{
    state::Extension,
    types_mint,
};

pub type MintMsg = CW721MintMsg<Extension>;

#[cw_serde]
pub struct BatchStoreMsg {
    // pub batch: [CW721MintMsg<Extension>; 50],
    pub batch: Vec<CW721MintMsg<Extension>>
}

#[cw_serde]
pub struct TransferOperation {
    pub recipient: String,
    pub tokens: Vec<String>,
}

#[cw_serde]
pub struct MintBatchMsg {
    pub amount: Uint128
}

#[cw_serde]
pub struct StoreConf {
    pub name: String,
    pub desc: String,
    pub ipfs: String,
    pub attributes: Vec<String>,
}

impl Default for StoreConf {
    fn default() -> Self {
        Self {
            name: String::from("nft"),
            desc: String::from("desc"),
            ipfs: String::from("ipfs://"),
            attributes: vec![],
        }
    }
}

#[cw_serde]
pub struct StoreConfMsg {
    pub attributes: Vec<Vec<String>>,
    pub conf: Option<StoreConf>
}

#[cw_serde]
#[derive(Default)]
pub struct InstantiateMsg {
    // This is the contract operator
    pub creator: String,

    // Name of the NFT contract
    pub name: String,

    // Symbol of the NFT
    pub symbol: String,

    pub dates: types_mint::Dates,

    pub cost: types_mint::Costs,

    pub burn: types_mint::Burn,

    // Maximum token supply
    pub token_supply: Uint128,

    // Wallet that recieves the funds
    pub wallet: types_mint::Wallet,

    // Defaults to 10
    pub max_mint_batch: Option<Uint128>,

    // Used for StoreConf call but can be provided during the call
    pub store_conf: StoreConf,
}

impl InstantiateMsg {
    pub fn new(creator: String) -> Self {
        Self {
            creator,
            name: Default::default(),
            symbol: Default::default(),
            dates: Default::default(),
            cost: Default::default(),
            burn: Default::default(),
            token_supply: Default::default(),
            wallet: Default::default(),
            max_mint_batch: Default::default(),
            store_conf: Default::default(),
        }
    }
}

impl From<InstantiateMsg> for CW721InstantiateMsg {
    fn from(msg: InstantiateMsg) -> CW721InstantiateMsg {
        CW721InstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            minter: msg.creator,
        }
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    // freeze contract
    Freeze(),

    // unfreeze contract
    Unfreeze(),

    // pause contract
    Pause(),

    // unpause contract
    Unpause(),

    // update the initial config
    UpdateConf (InstantiateMsg),

    // add token to pledge list
    Pledge {
        tokens: Vec<String>,
    },


    // burn given token
    Burn {
        token_id: String,
    },

    // burn tokens in batch
    BurnBatch { tokens: Vec<String> },

    RemoteBurnBatch{ tokens: Vec<String>, owner: String },

    // Mint a new token, can only be called by the contract minter
    Mint(),

    // mint using a max configurable amount per batch
    MintBatch(MintBatchMsg),

    RemoteMintBatch{ amount: Uint128, owner: String },

    // Store token metadata for later minting
    Store(MintMsg),

    // Store token metadata in batch for later minting
    StoreBatch(BatchStoreMsg),

    // Optimized batch token metadata storage
    StoreConf(StoreConfMsg),

    // Standard CW721 ExecuteMsg
    // Transfer is a base message to move a token to another account without triggering actions
    TransferNft {
        recipient: String,
        token_id: String,
    },

    // Transfer a batch of nfts to a single recipient
    TransferBatch(TransferOperation),

    // Transfer a multiple nfts to multiple recipients
    // TransferOperations {
    //     tx: Vec<TransferOperation>,
    // },

    // Send is a base message to transfer a token to a contract and trigger an action
    // on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },

    // Allows operator to transfer / send the token from the owner's account.
    // If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },

    // Remove previously granted Approval
    Revoke {
        spender: String,
        token_id: String,
    },

    // Allows operator to transfer / send any token from the owner's account.
    // If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },

    // Remove previously granted ApproveAll permission
    RevokeAll {
        operator: String,
    },
}

impl From<ExecuteMsg> for CW721ExecuteMsg<Extension, Empty> {
    fn from(msg: ExecuteMsg) -> CW721ExecuteMsg<Extension, Empty> {
        match msg {
            ExecuteMsg::TransferNft { recipient, token_id, } => CW721ExecuteMsg::TransferNft { recipient, token_id, },
            ExecuteMsg::SendNft { contract, token_id, msg, } => CW721ExecuteMsg::SendNft { contract, token_id, msg, },
            ExecuteMsg::Approve { spender, token_id, expires, } => CW721ExecuteMsg::Approve { spender, token_id, expires, },
            ExecuteMsg::Revoke { spender, token_id } => CW721ExecuteMsg::Revoke { spender, token_id },
            ExecuteMsg::ApproveAll { operator, expires } => CW721ExecuteMsg::ApproveAll { operator, expires },
            ExecuteMsg::RevokeAll { operator } => CW721ExecuteMsg::RevokeAll { operator },
            _ => panic!("cannot covert {:?} to CW721ExecuteMsg", msg),
        }
    }
}

#[cw_serde]
// #[derive(QueryResponses)]
pub enum QueryMsg {
    // Returns the current contract config
    Config {},

    // Standard cw721 queries
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    ApprovedForAll {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    NumTokens {},
    ContractInfo {},
    NftInfo {
        token_id: String,
    },
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    NftInfoBatch {
        tokens: Vec<String>,
    },
    BurntAmount {
        address: Addr
    },
    BurntList {
        address: Addr
    },
    Burned {
        tokens: Vec<String>,
    },
}

impl From<QueryMsg> for CW721QueryMsg<Empty> {
    fn from(msg: QueryMsg) -> CW721QueryMsg<Empty> {
        match msg {
            QueryMsg::OwnerOf { token_id, include_expired, } => CW721QueryMsg::OwnerOf { token_id, include_expired, },
            QueryMsg::NumTokens {} => CW721QueryMsg::NumTokens {},
            QueryMsg::ContractInfo {} => CW721QueryMsg::ContractInfo {},
            QueryMsg::NftInfo { token_id } => CW721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo { token_id, include_expired, } => CW721QueryMsg::AllNftInfo { token_id, include_expired, },
            QueryMsg::Tokens { owner, start_after, limit, } => CW721QueryMsg::Tokens { owner, start_after, limit, },
            QueryMsg::AllTokens { start_after, limit } => CW721QueryMsg::AllTokens { start_after, limit },
            _ => panic!("cannot covert {:?} to CW721QueryMsg", msg),
        }
    }
}

// #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
// #[serde(rename_all = "snake_case")]
#[cw_serde]
pub enum MigrateMsg<T> {
    WithConfigClearState {
        version: String,
        config: T,
    },
    WithConfig {
        version: String,
        config: T,
    }
}
