use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, StdResult, Response, Uint128};
use crate::msg::{ ExecuteMsg, MintMsg };
use cw721_base::{Cw721Contract, ContractError};
// use cw721_base::msg::Mint;
use crate::state::Cw721ExtendedContract;
use crate::state::Extension;

pub type Cw721MintMsg = MintMsg<Extension>;
pub fn execute_mint(deps: DepsMut, env: Env, info: MessageInfo, msg: MintMsg<Extension>) -> Result<Response, ContractError> {
    let mut fund = Coin {
        amount: Uint128::from(0u128),
        denom: String::from("luna"),
    };
    for coin in info.clone().funds {
        if coin.denom == "luna" {
            fund = Coin {
                amount: fund.amount + coin.amount,
                denom: coin.denom,
            };
        } else if coin.denom == "uluna" {
            fund = Coin {
                amount: fund.amount + coin.amount,
                denom: coin.denom,
            };
        } else if coin.denom == "Luna" {
            fund = Coin {
                amount: fund.amount + coin.amount,
                denom: coin.denom,
            };
        }
    }
    let token_num = match fund.amount.u128() {
        1000000 => 1,
        1500000 => 1,
        2500000 => 2,
        2900000 => 2,
        4200000 => 3,
        5500000 => 4,
        6500000 => 5,
        _ => 0,
    };
    let token_count = Cw721ExtendedContract::default().token_count(deps.storage)?;

    if token_num == 0 {
        return Err(ContractError::Unauthorized {});
    }
    else {
        Cw721ExtendedContract::default().execute(deps, env, info.clone(), ExecuteMsg::Mint(
            // MintMsg {
            //     extension: Some(Metadata {
            //         royalty_payment_address: Some("".to_string()),
            //         ..msg.extension.unwrap()
            //     }),
            //     ..msg::Mint
            // }
            msg,
        ).into())
        // return Ok(Response::new()
        //     .add_attribute("action", "mint")
        //     .add_attribute("minter", info.sender)
        //     .add_attribute("token_id", token_count.to_string())
        // )
    }
}