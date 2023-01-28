// use cosmwasm_schema::cw_serde;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Binary, Uint128, Timestamp};
use cw721_base::{
    msg::{
        ExecuteMsg as CW721ExecuteMsg,
        InstantiateMsg as CW721InstantiateMsg,
        QueryMsg as CW721QueryMsg,
    },
    MintMsg as CW721MintMsg,
};

use crate::state::{Extension};

pub type MintMsg = CW721MintMsg<Extension>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BatchStoreMsg {
    // pub batch: [CW721MintMsg<Extension>; 50],
    pub batch: Vec<CW721MintMsg<Extension>>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransferOperation {
    pub recipient: String,
    pub tokens: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MintBatchMsg {
    // pub batch: [CW721MintMsg<Extension>; 50],
    pub amount: Uint128
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RemoteMintBatchMsg {
    pub owner: String,
    pub amount: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RemoteBurnBatchMsg {
    pub owner: String,
    pub tokens: Vec<String>,
}

use cw721::Expiration;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct StoreConf {
    pub name: String,
    pub desc: String,
    pub ipfs: String,
    pub attributes: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct StoreConfMsg {
    pub attributes: Vec<Vec<String>>,
    pub conf: Option<StoreConf>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    // Name of the NFT contract
    pub name: String,

    // Symbol of the NFT contract
    pub symbol: String,

    // minter
    pub minter: String, // who can mint

    // if some start mint is used to start at a date
    pub start_mint: Option<Timestamp>,

    // if some end mint is the limit of minting phase
    pub end_mint: Option<Timestamp>,

    // name of the token
    pub cost_denom: String,
    // amount
    pub cost_amount: Uint128,

    // maximum token supply
    pub token_supply: Uint128,

    // wallet that recieves the funds
    pub funds_wallet: String,

    // defaults to 10
    pub max_mint_batch: Option<Uint128>,

    // turn this ON to allow holders of the nft to burn their tokens
    pub owners_can_burn: bool,

    // turn this off to do not allow the contract owner to burn tokens
    pub minter_can_burn: bool,

    // Used for StoreConf call but can be provided during the call
    pub store_conf: Option<StoreConf>,
}

impl From<InstantiateMsg> for CW721InstantiateMsg {
    fn from(msg: InstantiateMsg) -> CW721InstantiateMsg {
        CW721InstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            minter: msg.minter,
        }
    }
}

// Extended CW721 ExecuteMsg, added the ability to update, burn, and finalize nft
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
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

impl From<ExecuteMsg> for CW721ExecuteMsg<Extension> {
    fn from(msg: ExecuteMsg) -> CW721ExecuteMsg<Extension> {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

impl From<QueryMsg> for CW721QueryMsg {
    fn from(msg: QueryMsg) -> CW721QueryMsg {
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg<T> {
    pub version: String,
    pub config: Option<T>,
}
