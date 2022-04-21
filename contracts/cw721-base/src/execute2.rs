use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{
    BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
    Uint128,
};

use cw2::set_contract_version;
use cw721::{
    ContractInfoResponse, CustomMsg, Cw721Execute, Cw721ReceiveMsg, Expiration, NumTokensResponse,
};

use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;
use crate::traits::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw721-base";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> Cw721ExtendedContract<'a> {
    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Withdraw {} => self.execute_withdraw(deps, env, info),
            _ => Cw721ExtendedContract::default()._execute(deps, env, info, msg),
        }
    }
}

impl<'a> Cw721ExtendedExecute<Extension> for Cw721ExtendedContract<'a> {
    fn execute_withdraw(
        &self,
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let team = self.team.load(deps.storage)?;
        let pro = self.pro.load(deps.storage)?;
        let treas = self.treas.load(deps.storage)?;

        let current_uluna_amount = deps
            .querier
            .query_balance(env.contract.address.to_string(), "uluna")?
            .amount;

        let team_portion = current_uluna_amount * Uint128::from(30u128) / Uint128::from(100u128);
        let pro_portion = current_uluna_amount * Uint128::from(14u128) / Uint128::from(100u128);
        let treas_portion = current_uluna_amount * Uint128::from(56u128) / Uint128::from(100u128);

        let mut messages: Vec<CosmosMsg> = vec![];
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: team.to_string(),
            amount: vec![Coin {
                denom: String::from("uluna"),
                amount: team_portion,
            }],
        }));
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: pro.to_string(),
            amount: vec![Coin {
                denom: String::from("uluna"),
                amount: pro_portion,
            }],
        }));
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: treas.to_string(),
            amount: vec![Coin {
                denom: String::from("uluna"),
                amount: treas_portion,
            }],
        }));
        Ok(Response::new().add_messages(messages))
    }

    fn execute_set_base_uri(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _uri: String,
    ) -> Result<Response, ContractError> {
        self.base_uri.save(deps.storage, &_uri)?;

        Ok(Response::new()
            .add_attribute("action", "set_base_uri")
            .add_attribute("base_uri", &_uri))
    }

    fn execute_set_art_reveal(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        art_reveal: bool,
    ) -> Result<Response, ContractError> {
        self.is_on_reveal.save(deps.storage, &art_reveal)?;

        Ok(Response::new()
            .add_attribute("action", "set_art_reveal")
            .add_attribute("base_uri", &art_reveal.to_string()))
    }
}

// helpers
// impl<'a, T, C> Cw721Contract<'a, T, C>
// where
//     T: Serialize + DeserializeOwned + Clone,
//     C: CustomMsg,
// {
//     pub fn _transfer_nft(
//         &self,
//         deps: DepsMut,
//         env: &Env,
//         info: &MessageInfo,
//         recipient: &str,
//         token_id: &str,
//     ) -> Result<TokenInfo<T>, ContractError> {
//         let mut token = self.tokens.load(deps.storage, &token_id)?;
//         // ensure we have permissions
//         self.check_can_send(deps.as_ref(), env, info, &token)?;
//         // set owner and remove existing approvals
//         token.owner = deps.api.addr_validate(recipient)?;
//         token.approvals = vec![];
//         self.tokens.save(deps.storage, &token_id, &token)?;
//         Ok(token)
//     }

//     #[allow(clippy::too_many_arguments)]
//     pub fn _update_approvals(
//         &self,
//         deps: DepsMut,
//         env: &Env,
//         info: &MessageInfo,
//         spender: &str,
//         token_id: &str,
//         // if add == false, remove. if add == true, remove then set with this expiration
//         add: bool,
//         expires: Option<Expiration>,
//     ) -> Result<TokenInfo<T>, ContractError> {
//         let mut token = self.tokens.load(deps.storage, &token_id)?;
//         // ensure we have permissions
//         self.check_can_approve(deps.as_ref(), env, info, &token)?;

//         // update the approval list (remove any for the same spender before adding)
//         let spender_addr = deps.api.addr_validate(spender)?;
//         token.approvals = token
//             .approvals
//             .into_iter()
//             .filter(|apr| apr.spender != spender_addr)
//             .collect();

//         // only difference between approve and revoke
//         if add {
//             // reject expired data as invalid
//             let expires = expires.unwrap_or_default();
//             if expires.is_expired(&env.block) {
//                 return Err(ContractError::Expired {});
//             }
//             let approval = Approval {
//                 spender: spender_addr,
//                 expires,
//             };
//             token.approvals.push(approval);
//         }

//         self.tokens.save(deps.storage, &token_id, &token)?;

//         Ok(token)
//     }

//     /// returns true iff the sender can execute approve or reject on the contract
//     pub fn check_can_approve(
//         &self,
//         deps: Deps,
//         env: &Env,
//         info: &MessageInfo,
//         token: &TokenInfo<T>,
//     ) -> Result<(), ContractError> {
//         // owner can approve
//         if token.owner == info.sender {
//             return Ok(());
//         }
//         // operator can approve
//         let op = self
//             .operators
//             .may_load(deps.storage, (&token.owner, &info.sender))?;
//         match op {
//             Some(ex) => {
//                 if ex.is_expired(&env.block) {
//                     Err(ContractError::Unauthorized {})
//                 } else {
//                     Ok(())
//                 }
//             }
//             None => Err(ContractError::Unauthorized {}),
//         }
//     }

//     /// returns true iff the sender can transfer ownership of the token
//     fn check_can_send(
//         &self,
//         deps: Deps,
//         env: &Env,
//         info: &MessageInfo,
//         token: &TokenInfo<T>,
//     ) -> Result<(), ContractError> {
//         // owner can send
//         if token.owner == info.sender {
//             return Ok(());
//         }

//         // any non-expired token approval can send
//         if token
//             .approvals
//             .iter()
//             .any(|apr| apr.spender == info.sender && !apr.is_expired(&env.block))
//         {
//             return Ok(());
//         }

//         // operator can send
//         let op = self
//             .operators
//             .may_load(deps.storage, (&token.owner, &info.sender))?;
//         match op {
//             Some(ex) => {
//                 if ex.is_expired(&env.block) {
//                     Err(ContractError::Unauthorized {})
//                 } else {
//                     Ok(())
//                 }
//             }
//             None => Err(ContractError::Unauthorized {}),
//         }
//     }
// }
