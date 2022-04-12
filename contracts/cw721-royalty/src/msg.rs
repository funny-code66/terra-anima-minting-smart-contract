use cosmwasm_std::{ Binary, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw721::Expiration;

use crate::state::Extension;
pub use cw721_base::{ContractError, InstantiateMsg, MintMsg, MinterResponse, QueryMsg as Cw721QueryMsg};

pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Extension>;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Transfer is a base message to move a token to another account without triggering actions
    TransferNft { recipient: String, token_id: String },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke { spender: String, token_id: String },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { operator: String },

    /// Mint a new NFT, can only be called by the contract minter
    Mint(MintMsg<Extension>)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    RoyaltyInfo {
        token_id: String,
        sale_price: Uint128,
    },
    CheckRoyalties {},
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    AllOperators {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    NumTokens {},
    ContractInfo {},
    NftInfo { token_id: String },
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
}

impl From<ExecuteMsg> for Cw721ExecuteMsg {
    fn from (msg: ExecuteMsg) -> Cw721ExecuteMsg {
        match msg {
            // Transfer is a base message to move a token to another account without triggering actions
            ExecuteMsg::Mint(msg) => Cw721ExecuteMsg::Mint(msg),
            ExecuteMsg::TransferNft { recipient, token_id } =>
            Cw721ExecuteMsg::TransferNft { recipient, token_id },
            
            // Send is a base message to transfer a token to a contract and trigger an action
            // on the receiving contract.
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => Cw721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            },
            // Allows operator to transfer / send the token from the owner's account.
            // If expiration is set, then this allowance has a time/height limit
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => Cw721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            },
            // Remove previously granted Approval
            ExecuteMsg::Revoke { spender, token_id } =>
            Cw721ExecuteMsg::Revoke { spender, token_id },
            // Allows operator to transfer / send any token from the owner's account.
            // If expiration is set, then this allowance has a time/height limit
            ExecuteMsg::ApproveAll {
                operator,
                expires,
            } => Cw721ExecuteMsg::ApproveAll {
                operator,
                expires,
            },
            // Remove previously granted ApproveAll permission
            ExecuteMsg::RevokeAll { operator } =>
            Cw721ExecuteMsg::RevokeAll { operator },
        }
    }
}

impl From<QueryMsg> for Cw721QueryMsg {
    fn from (msg: QueryMsg) -> Cw721QueryMsg {
        match msg {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Cw721QueryMsg:: OwnerOf {
                token_id,
                include_expired,
            },
            // QueryMsg::AllOperators {
            //     owner,
            //     include_expired,
            //     start_after,
            //     limit,
            // } => Cw721QueryMsg::AllOperators {
            //     owner,
            //     include_expired,
            //     start_after,
            //     limit,
            // },
            QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
            QueryMsg::ContractInfo {} => Cw721QueryMsg::ContractInfo {},
            QueryMsg::NftInfo { token_id } => Cw721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
            QueryMsg::Tokens{
                owner,
                start_after,
                limit,
            } => Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllTokens { start_after, limit } => {
                Cw721QueryMsg::AllTokens { start_after, limit }
            },
            _ => panic!("cannot convert {:?} to Cw721QueryMsg", msg),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct RoyaltiesInfoResponse {
    pub address: String,
    pub royalty_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CheckRoyaltiesResponse {
    pub royalty_payments: bool,
}