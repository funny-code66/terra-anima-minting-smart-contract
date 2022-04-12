use crate::state::Extension;
pub use cw721_base::{ContractError, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};

pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension>;
