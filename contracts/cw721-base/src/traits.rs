use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::msg::*;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Uint128};
use cw0::Expiration;

// TODO: move this somewhere else... ideally cosmwasm-std
pub trait CustomMsg: Clone + std::fmt::Debug + PartialEq + JsonSchema {}

impl CustomMsg for Empty {}

pub trait Cw721Extended<T, C>: Cw721ExtendedExecute<T, C> + Cw721ExtendedQuery<T>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
}

pub trait Cw721ExtendedExecute<T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    type Err: ToString;
}

pub trait Cw721ExtendedQuery<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn query_royalties_info(
        &self,
        deps: Deps,
        token_id: String,
        sale_price: Uint128,
    ) -> StdResult<RoyaltiesInfoResponse>;

    fn check_royalties(&self, _deps: Deps) -> StdResult<CheckRoyaltiesResponse>;

    fn query_is_on_reveal(&self, _deps: Deps) -> StdResult<IsOnRevealResponse>;

    fn query_get_token_uri(&self, _deps: Deps, token_id: String) -> StdResult<GetTokenUriResponse>;
}
