use crate::msg::*;
use crate::state::Cw721ExtendedContract;
use cosmwasm_std::{Decimal, Deps, StdResult, Uint128};

pub fn query_royalties_info(
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

pub fn check_royalties(_deps: Deps) -> StdResult<CheckRoyaltiesResponse> {
    Ok(CheckRoyaltiesResponse {
        royalty_payments: true,
    })
}

pub fn query_is_on_reveal(_deps: Deps) -> StdResult<IsOnRevealResponse> {
    Ok(IsOnRevealResponse { is_on_reveal: true })
}

pub fn query_get_token_uri(_deps: Deps, token_id: String) -> StdResult<GetTokenUriResponse> {
    Ok(GetTokenUriResponse {
        token_uri: String::from("NOT_YET_REVEALED"),
    })
}
