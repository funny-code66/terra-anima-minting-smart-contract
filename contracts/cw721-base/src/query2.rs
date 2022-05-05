use crate::constants::*;
use crate::msg::*;
use crate::state::*;
use crate::state2::*;
use crate::threshold::ThresholdResponse;
use crate::traits::*;
// use bytes::Bytes;
use cosmwasm_std::{
    to_binary, Addr, Binary, BlockInfo, Decimal, Deps, Env, Order, StdResult, Uint128,
};
// use cw3;
use cw_storage_plus::Bound;

impl<'a> Cw721ExtendedContract<'a> {
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::RoyaltyInfo {
                token_id,
                sale_price,
            } => to_binary(&self.query_royalties_info(deps, token_id, sale_price)?),
            QueryMsg::CheckRoyalties {} => to_binary(&self.check_royalties(deps)?),
            // QueryMsg::IsOnReveal {} => to_binary(&self.query_is_on_reveal(deps)?),
            // QueryMsg::GetTokenUri { token_id } => {
            //     to_binary(&self.query_get_token_uri(deps, token_id)?)
            // }
            QueryMsg::GetBalance { owner } => to_binary(&self.query_get_balance(deps, owner)?),
            // QueryMsg::IsOnWhitelist { member } => {
            //     to_binary(&self.check_is_on_whitelist(deps, member)?)
            // }
            // QueryMsg::IsOnPresale {} => to_binary(&self.check_is_on_presale(deps, env)?),
            QueryMsg::GetExtension { token_id } => {
                to_binary(&self.query_get_extension(deps, token_id)?)
            }
            QueryMsg::Threshold {} => to_binary(&self.query_threshold(deps)?),
            QueryMsg::Proposal { proposal_id } => {
                to_binary(&self.query_proposal(deps, env, proposal_id)?)
            }
            QueryMsg::Vote { proposal_id, voter } => {
                to_binary(&self.query_vote(deps, proposal_id, voter)?)
            }
            QueryMsg::ListProposals { start_after, limit } => {
                to_binary(&self.list_proposals(deps, env, start_after, limit)?)
            }
            QueryMsg::ReverseProposals {
                start_before,
                limit,
            } => to_binary(&self.reverse_proposals(deps, env, start_before, limit)?),
            QueryMsg::ListVotes {
                proposal_id,
                start_after,
                limit,
            } => to_binary(&self.list_votes(deps, proposal_id, start_after, limit)?),
            QueryMsg::Voter { address } => to_binary(&self.query_voter(deps, address)?),
            QueryMsg::ListVoters { start_after, limit } => {
                to_binary(&self.list_voters(deps, start_after, limit)?)
            }
            _ => Cw721ExtendedContract::default()._query(deps, env, msg),
        }
    }
}

impl<'a> Cw721ExtendedQuery<Extension> for Cw721ExtendedContract<'a> {
    fn query_royalties_info(
        &self,
        _deps: Deps,
        _token_id: String,
        sale_price: Uint128,
    ) -> StdResult<RoyaltiesInfoResponse> {
        let percentage = Decimal::percent(ROYALTY_PERCENTAGE);
        let royalty_from_sale_price = sale_price * percentage;

        Ok(RoyaltiesInfoResponse {
            address: ROYALTY_ADDRESS.to_string(),
            royalty_amount: royalty_from_sale_price,
        })
    }

    fn check_royalties(&self, _deps: Deps) -> StdResult<CheckRoyaltiesResponse> {
        Ok(CheckRoyaltiesResponse {
            royalty_payments: true,
        })
    }

    // fn query_is_on_reveal(&self, deps: Deps) -> StdResult<IsOnRevealResponse> {
    //     let res: bool = self
    //         .is_on_reveal
    //         .may_load(deps.storage)?
    //         .unwrap_or_default();

    //     Ok(IsOnRevealResponse { is_on_reveal: res })
    // }

    fn query_get_token_uri(&self, _deps: Deps, token_id: String) -> StdResult<GetTokenUriResponse> {
        // let is_on_reveal = self
        //     .is_on_reveal
        //     .may_load(deps.storage)?
        //     .unwrap_or_default();

        // let res = match is_on_reveal {
        //     true => format!("{}{}.json", BASE_URI, token_id),
        //     false => String::from("NOT_YET_REVEALED"),
        // };

        Ok(GetTokenUriResponse {
            token_uri: format!("{}{}.json", BASE_URI, token_id),
        })
    }

    fn query_get_balance(&self, deps: Deps, owner: String) -> StdResult<GetBalanceResponse> {
        let res = self
            .wallet_balance
            .may_load(deps.storage, &Addr::unchecked(owner))?
            .unwrap_or(0);
        Ok(GetBalanceResponse { balance: res })
    }

    // fn check_is_on_whitelist(
    //     &self,
    //     deps: Deps,
    //     member: String,
    // ) -> StdResult<IsOnWhitelistResponse> {
    //     let res = self
    //         .whitelist
    //         .may_load(deps.storage, &Addr::unchecked(member))?
    //         .unwrap_or(false);
    //     Ok(IsOnWhitelistResponse {
    //         is_on_whitelist: res,
    //     })
    // }

    fn query_get_extension(
        &self,
        deps: Deps,
        token_id: String,
    ) -> StdResult<GetExtensionResponse<Extension>> {
        let info = self.tokens.load(deps.storage, &token_id)?;
        Ok(GetExtensionResponse {
            extension: info.extension,
        })
    }

    // fn check_is_on_presale(&self, deps: Deps, env: Env) -> StdResult<IsOnPresaleResponse> {
    //     let time_deployed = self.time_deployed.load(deps.storage)?;

    //     if env.block.time.seconds() < (time_deployed.seconds() + 600) {
    //         Ok(IsOnPresaleResponse { flag: true })
    //     } else {
    //         Ok(IsOnPresaleResponse { flag: false })
    //     }
    // }

    fn query_threshold(&self, deps: Deps) -> StdResult<ThresholdResponse> {
        let cfg = self.CONFIG.load(deps.storage)?;
        Ok(cfg.threshold.to_response(cfg.total_weight))
    }

    fn query_proposal(&self, deps: Deps, env: Env, id: u64) -> StdResult<ProposalResponse> {
        let prop = self.PROPOSALS.load(deps.storage, &id.to_string()[..])?;
        let status = prop.current_status(&env.block);
        let threshold = prop.threshold.to_response(prop.total_weight);
        Ok(ProposalResponse {
            id,
            title: prop.title,
            description: prop.description,
            msgs: prop.msgs,
            status,
            expires: prop.expires,
            // threshold,
            threshold: threshold.into(),
        })
    }

    #[allow(unused_variables)]
    fn list_proposals(
        &self,
        deps: Deps,
        env: Env,
        start_after: Option<u64>,
        limit: Option<u32>,
    ) -> StdResult<ProposalListResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = Bound::Exclusive(start_after.unwrap().to_string().into_bytes());
        let proposals = self
            .PROPOSALS
            .range(deps.storage, Some(start), None, Order::Ascending)
            .take(limit)
            .map(|p| self.map_proposals(&env.block, p))
            .collect::<StdResult<_>>()?;

        Ok(ProposalListResponse { proposals })
    }

    #[allow(unused_variables)]
    fn reverse_proposals(
        &self,
        deps: Deps,
        env: Env,
        start_before: Option<u64>,
        limit: Option<u32>,
    ) -> StdResult<ProposalListResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

        let end = start_before.map(|end_u64| Bound::exclusive(&end_u64.to_string()[..]));
        let props: StdResult<Vec<_>> = self
            .PROPOSALS
            .range(deps.storage, None, None, Order::Descending)
            .map(|p| self.map_proposal(&env.block, Ok((1, p?.1))))
            .collect();

        Ok(ProposalListResponse { proposals: props? })
    }

    fn map_proposals(
        &self,
        block: &BlockInfo,
        item: StdResult<(Vec<u8>, Proposal)>,
    ) -> StdResult<ProposalResponse> {
        item.map(|(id_vec, prop)| {
            let status = prop.current_status(block);
            let threshold = prop.threshold.to_response(prop.total_weight);
            let mut id: u64 = 0;
            for i in 0..8 {
                id *= 256;
                id += u64::from(id_vec[i]);
            }
            ProposalResponse {
                id,
                title: prop.title,
                description: prop.description,
                msgs: prop.msgs,
                status,
                expires: prop.expires,
                threshold,
            }
        })
    }

    fn map_proposal(
        &self,
        block: &BlockInfo,
        item: StdResult<(u64, Proposal)>,
    ) -> StdResult<ProposalResponse> {
        item.map(|(id, prop)| {
            let status = prop.current_status(block);
            let threshold = prop.threshold.to_response(prop.total_weight);
            ProposalResponse {
                id,
                title: prop.title,
                description: prop.description,
                msgs: prop.msgs,
                status,
                expires: prop.expires,
                threshold: threshold.into(),
            }
        })
    }

    fn query_vote(&self, deps: Deps, proposal_id: u64, voter: String) -> StdResult<VoteResponse> {
        let voter = deps.api.addr_validate(&voter)?;
        let ballot = self
            .BALLOTS
            .may_load(deps.storage, (&proposal_id.to_string()[..], &voter))?;
        let vote = ballot.map(|b| VoteInfo {
            proposal_id,
            voter: voter.into(),
            vote: b.vote,
            weight: b.weight,
        });
        Ok(VoteResponse { vote })
    }

    fn list_votes(
        &self,
        deps: Deps,
        proposal_id: u64,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<VoteListResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start =
            start_after.map(|start_after_string| Bound::Exclusive(start_after_string.into()));
        let votes = self
            .BALLOTS
            .prefix(&proposal_id.to_string()[..])
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|item| {
                item.map(|(addr, ballot)| VoteInfo {
                    proposal_id,
                    voter: String::from_utf8_lossy(&addr).to_string(),
                    vote: ballot.vote,
                    weight: ballot.weight,
                })
            })
            .collect::<StdResult<_>>()?;

        Ok(VoteListResponse { votes })
    }

    fn query_voter(&self, deps: Deps, voter: String) -> StdResult<VoterResponse> {
        let voter = deps.api.addr_validate(&voter)?;
        let weight = self.VOTERS.may_load(deps.storage, &voter)?;
        Ok(VoterResponse { weight })
    }

    fn list_voters(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<VoterListResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(|s| Bound::Exclusive(s.into()));

        let voters = self
            .VOTERS
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|item| {
                item.map(|(addr, weight)| VoterDetail {
                    addr: String::from_utf8_lossy(&addr).to_string(),
                    weight,
                })
            })
            .collect::<StdResult<_>>()?;

        Ok(VoterListResponse { voters })
    }
}
