use crate::msg::{ExecuteMsg, MintMsg};
use cosmwasm_std::{Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw721_base::{ContractError, Cw721Contract};
// use cw721_base::msg::Mint;
use crate::state::Cw721ExtendedContract;
use crate::state::Extension;

pub type Cw721MintMsg = MintMsg<Extension>;
pub fn execute_mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MintMsg<Extension>,
) -> Result<Response, ContractError> {
    let mut fund = Coin {
        amount: Uint128::from(0u128),
        denom: String::from("luna"),
    };

    for coin in info.clone().funds {
        if coin.denom == "uluna" {
            fund = Coin {
                amount: fund.amount + coin.amount,
                denom: coin.denom,
            };
        }
    }

    let token_count = Cw721ExtendedContract::default().token_count(deps.storage)?;

    let token_num = match token_count < 1000 {
        true => match fund.amount.u128() {
            130000 => 1,
            125000 => match msg.token_num == String::from("b") {
                true => 1,
                false => 0,
            },
            _ => 0,
        },
        false => match fund.amount.u128() {
            150000 => 1,
            145000 => match msg.token_num == String::from("b") {
                true => 1,
                false => 0,
            },
            140000 => match msg.token_num == String::from("c") {
                true => 1,
                false => 0,
            },
            135000 => match msg.token_num == String::from("d") {
                true => 1,
                false => 0,
            },
            13000 => match msg.token_num == String::from("e") {
                true => 1,
                false => 0,
            },
            _ => 0,
        },
    };

    match token_num {
        0 => Err(ContractError::Unauthorized {}),
        _ => Cw721ExtendedContract::default().execute(
            deps,
            env,
            info.clone(),
            ExecuteMsg::Mint(msg).into(),
        ),
    }
}
