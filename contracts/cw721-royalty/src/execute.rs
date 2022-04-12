use cosmwasm_std::{Deps, Env, MessageInfo};
use crate::msg::ExecuteMsg;
use crate::error::ContractError;

pub fn execute_mint(deps: Deps, env: Env, info: MessageInfo, msg: ExecuteMsg::Mint) -> StdResult<Response, ContractError> {
    for coin in info.funds {
        if coin.denom != "luna" {
            Err(ContractError::Unauthorized {})
        }
        
    }
}