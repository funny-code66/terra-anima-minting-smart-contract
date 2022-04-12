use cosmwasm_std::{DepsMut, Env, MessageInfo, StdResult, Response};
use crate::msg::{ ExecuteMsg, MintMsg };
use cw721_base::ContractError;

pub fn execute_mint(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    for coin in info.funds {
        if coin.denom != "luna" {
            continue;
        }
        
    }
    return Err(ContractError::Unauthorized {});
    // Cw721ExtendedContract::default().execute(deps, env, info, ExecuteMsg::Mint(
    //     MintMsg {
    //         extension: Some(Metadata {
    //             royalty_payment_address: Some("".to_string()),
    //             ..msg.extension.unwrap()
    //         }),
    //         ..msg
    //     }
    // ))}
}