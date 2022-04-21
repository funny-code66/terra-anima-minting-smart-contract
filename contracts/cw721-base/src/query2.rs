use crate::msg::*;
use crate::state::*;
use crate::traits::*;
use cosmwasm_std::{to_binary, Binary, Decimal, Deps, Env, StdResult, Uint128};

impl<'a> Cw721ExtendedContract<'a> {
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::RoyaltyInfo {
                token_id,
                sale_price,
            } => to_binary(&self.query_royalties_info(deps, token_id, sale_price)?),
            QueryMsg::CheckRoyalties {} => to_binary(&self.check_royalties(deps)?),
            QueryMsg::IsOnReveal {} => to_binary(&self.query_is_on_reveal(deps)?),
            QueryMsg::GetTokenUri { token_id } => {
                to_binary(&self.query_get_token_uri(deps, token_id)?)
            }
            _ => Cw721ExtendedContract::default()._query(deps, env, msg),
        }
    }
}

impl<'a> Cw721ExtendedQuery<Extension> for Cw721ExtendedContract<'a> {
    fn query_royalties_info(
        &self,
        deps: Deps,
        token_id: String,
        sale_price: Uint128,
    ) -> StdResult<RoyaltiesInfoResponse> {
        let contract = Cw721ExtendedContract::default();
        let token_info = contract.tokens.load(deps.storage, &token_id)?;

        let royalty_percentage = match token_info.extension {
            Some(ref ext) => match ext.royalty_percentage {
                Some(percentage) => Decimal::percent(percentage),
                None => Decimal::percent(0),
            },
            None => Decimal::percent(0),
        };
        let royalty_from_sale_price = sale_price * royalty_percentage;

        let royalty_address = match token_info.extension {
            Some(ext) => match ext.royalty_payment_address {
                Some(addr) => addr,
                None => String::from(""),
            },
            None => String::from(""),
        };

        Ok(RoyaltiesInfoResponse {
            address: royalty_address,
            royalty_amount: royalty_from_sale_price,
        })
    }

    fn check_royalties(&self, _deps: Deps) -> StdResult<CheckRoyaltiesResponse> {
        Ok(CheckRoyaltiesResponse {
            royalty_payments: true,
        })
    }

    fn query_is_on_reveal(&self, _deps: Deps) -> StdResult<IsOnRevealResponse> {
        Ok(IsOnRevealResponse { is_on_reveal: true })
    }

    fn query_get_token_uri(
        &self,
        _deps: Deps,
        _token_id: String,
    ) -> StdResult<GetTokenUriResponse> {
        Ok(GetTokenUriResponse {
            token_uri: String::from("NOT_YET_REVEALED"),
        })
    }
}
