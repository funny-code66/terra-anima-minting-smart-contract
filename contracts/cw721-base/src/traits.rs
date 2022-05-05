use cw0::Expiration;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{
    BlockInfo, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Uint128,
};

use crate::error::*;
use crate::msg::*;
use crate::state2::*;
use crate::threshold::ThresholdResponse;
use cw3::Vote;

// TODO: move this somewhere else... ideally cosmwasm-std
pub trait CustomMsg: Clone + std::fmt::Debug + PartialEq + JsonSchema {}

impl CustomMsg for Empty {}

pub trait Cw721Extended<T, C>: Cw721ExtendedExecute<T> + Cw721ExtendedQuery<T>
where
    T: Serialize + DeserializeOwned + Clone + Default,
    C: CustomMsg,
{
}

pub trait Cw721ExtendedExecute<T>
where
    T: Serialize + DeserializeOwned + Clone + Default,
{
    // fn execute_withdraw(
    //     &self,
    //     _deps: DepsMut,
    //     _env: Env,
    //     _info: MessageInfo,
    // ) -> Result<Response, ContractError>;

    // fn execute_set_art_reveal(
    //     &self,
    //     _deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     art_reveal: bool,
    // ) -> Result<Response, ContractError>;

    // fn execute_free_mint(
    //     &self,
    //     deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     msg: FreeMintMsg<T>,
    // ) -> Result<Response, ContractError>;

    // fn execute_sign(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    // ) -> Result<Response, ContractError>;

    // fn execute_add_whitelist(
    //     &self,
    //     deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     member: String,
    // ) -> Result<Response, ContractError>;

    // fn execute_remove_whitelist(
    //     &self,
    //     deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     member: String,
    // ) -> Result<Response, ContractError>;

    fn execute_add_extension(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_id: String,
        ext: T,
    ) -> Result<Response, ContractError>;

    fn execute_propose(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        title: String,
        description: String,
        msgs: Vec<CosmosMsg>,
        // we ignore earliest
        latest: Option<Expiration>,
    ) -> Result<Response<Empty>, ContractError>;

    fn execute_vote(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        proposal_id: u64,
        vote: Vote,
    ) -> Result<Response<Empty>, ContractError>;

    fn execute_execute(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        proposal_id: u64,
    ) -> Result<Response, ContractError>;

    fn execute_close(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        proposal_id: u64,
    ) -> Result<Response<Empty>, ContractError>;
}

pub trait Cw721ExtendedQuery<T>
where
    T: Serialize + DeserializeOwned + Clone + Default,
{
    fn query_royalties_info(
        &self,
        _deps: Deps,
        _token_id: String,
        sale_price: Uint128,
    ) -> StdResult<RoyaltiesInfoResponse>;

    fn check_royalties(&self, _deps: Deps) -> StdResult<CheckRoyaltiesResponse>;

    // fn query_is_on_reveal(&self, _deps: Deps) -> StdResult<IsOnRevealResponse>;

    fn query_get_token_uri(&self, _deps: Deps, token_id: String) -> StdResult<GetTokenUriResponse>;

    fn query_get_extension(
        &self,
        _deps: Deps,
        token_id: String,
    ) -> StdResult<GetExtensionResponse<T>>;

    fn query_get_balance(&self, deps: Deps, owner: String) -> StdResult<GetBalanceResponse>;

    // fn check_is_on_whitelist(&self, deps: Deps, member: String)
    //     -> StdResult<IsOnWhitelistResponse>;
    // fn check_is_on_presale(&self, deps: Deps, env: Env) -> StdResult<IsOnPresaleResponse>;

    fn query_threshold(&self, deps: Deps) -> StdResult<ThresholdResponse>;

    fn query_proposal(&self, deps: Deps, env: Env, id: u64) -> StdResult<ProposalResponse>;

    fn list_proposals(
        &self,
        deps: Deps,
        env: Env,
        start_after: Option<u64>,
        limit: Option<u32>,
    ) -> StdResult<ProposalListResponse>;

    fn reverse_proposals(
        &self,
        deps: Deps,
        env: Env,
        start_before: Option<u64>,
        limit: Option<u32>,
    ) -> StdResult<ProposalListResponse>;

    fn map_proposals(
        &self,
        block: &BlockInfo,
        item: StdResult<(Vec<u8>, Proposal)>,
    ) -> StdResult<ProposalResponse>;

    fn map_proposal(
        &self,
        block: &BlockInfo,
        item: StdResult<(u64, Proposal)>,
    ) -> StdResult<ProposalResponse>;

    fn query_vote(&self, deps: Deps, proposal_id: u64, voter: String) -> StdResult<VoteResponse>;

    fn list_votes(
        &self,
        deps: Deps,
        proposal_id: u64,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<VoteListResponse>;

    fn query_voter(&self, deps: Deps, voter: String) -> StdResult<VoterResponse>;

    fn list_voters(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<VoterListResponse>;
}
