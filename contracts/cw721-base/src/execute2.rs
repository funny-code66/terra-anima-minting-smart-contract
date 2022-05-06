use crate::constants::*;
use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;
use crate::state2::*;
use crate::traits::*;

use cosmwasm_std::{
    Addr, BankMsg, Coin, CosmosMsg, DepsMut, Empty, Env, MessageInfo, Response, Uint128,
};
use cw0::Expiration;
use cw3::Vote;
use std::cmp::Ordering;

// const BASE_URI: &str = "ipfs://QmRiLKmhizpnwqpHGeiJnL4G6fsPAxdEdCiDkuJpt7xHPH/";

impl<'a> Cw721ExtendedContract<'a> {
    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        match msg {
            // ExecuteMsg::FreeMint(msg) => self.execute_free_mint(deps, env, info, msg),
            ExecuteMsg::Withdraw {} => self.execute_withdraw(deps, env, info),
            // ExecuteMsg::SetArtReveal { art_reveal } => {
            //     self.execute_set_art_reveal(deps, env, info, art_reveal)
            // }
            // ExecuteMsg::Sign {} => self.execute_sign(deps, env, info),
            // ExecuteMsg::AddWhitelist { member } => {
            //     self.execute_add_whitelist(deps, env, info, member)
            // }
            // ExecuteMsg::RemoveWhitelist { member } => {
            //     self.execute_remove_whitelist(deps, env, info, member)
            // }
            ExecuteMsg::AddExtension(msg) => {
                self.execute_add_extension(deps, env, info, msg.token_id, msg.extension)
            }
            // CW3
            ExecuteMsg::Propose {
                title,
                description,
                msgs,
                latest,
            } => self.execute_propose(deps, env, info, title, description, msgs, latest),
            ExecuteMsg::Vote { proposal_id, vote } => {
                self.execute_vote(deps, env, info, proposal_id, vote)
            }
            ExecuteMsg::Execute { proposal_id } => {
                self.execute_execute(deps, env, info, proposal_id)
            }
            ExecuteMsg::Close { proposal_id } => self.execute_close(deps, env, info, proposal_id),

            _ => Cw721ExtendedContract::default()._execute(deps, env, info, msg),
        }
    }
}

impl<'a> Cw721ExtendedExecute<Extension> for Cw721ExtendedContract<'a> {
    fn execute_withdraw(
        &self,
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
    ) -> Result<Response, ContractError> {
        //     let team_signed = self
        //         .cw3_signature
        //         .may_load(deps.storage, &Addr::unchecked(ADDR_TEAM))?
        //         .unwrap_or(false);
        //     let pro_signed = self
        //         .cw3_signature
        //         .may_load(deps.storage, &Addr::unchecked(ADDR_PRO))?
        //         .unwrap_or(false);
        //     let treas_signed = self
        //         .cw3_signature
        //         .may_load(deps.storage, &Addr::unchecked(ADDR_TREAS))?
        //         .unwrap_or(false);

        //     if !team_signed || !pro_signed || !treas_signed {
        //         return Err(ContractError::NotAllSigned {});
        //     }

        let current_uluna_amount = deps
            .querier
            .query_balance(env.contract.address.to_string(), "uluna")?
            .amount;

        let team_portion = current_uluna_amount * Uint128::from(30u128) / Uint128::from(100u128);
        let pro_portion = current_uluna_amount * Uint128::from(14u128) / Uint128::from(100u128);
        let treas_portion = current_uluna_amount * Uint128::from(56u128) / Uint128::from(100u128);

        // self.cw3_signature
        //     .save(deps.storage, &Addr::unchecked(ADDR_TEAM), &(false))?;
        // self.cw3_signature
        //     .save(deps.storage, &Addr::unchecked(ADDR_PRO), &(false))?;
        // self.cw3_signature
        //     .save(deps.storage, &Addr::unchecked(ADDR_TREAS), &(false))?;

        let mut messages: Vec<CosmosMsg> = vec![];
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: ADDR_TEAM.to_string(),
            amount: vec![Coin {
                denom: String::from("uluna"),
                amount: team_portion,
            }],
        }));
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: ADDR_PRO.to_string(),
            amount: vec![Coin {
                denom: String::from("uluna"),
                amount: pro_portion,
            }],
        }));
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: ADDR_TREAS.to_string(),
            amount: vec![Coin {
                denom: String::from("uluna"),
                amount: treas_portion,
            }],
        }));
        Ok(Response::new().add_messages(messages))
    }

    // fn execute_set_art_reveal(
    //     &self,
    //     deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     art_reveal: bool,
    // ) -> Result<Response, ContractError> {
    //     if info.sender != self.minter.load(deps.storage)? {
    //         return Err(ContractError::NotMinter {});
    //     }
    //     self.is_on_reveal.save(deps.storage, &art_reveal)?;

    //     Ok(Response::new()
    //         .add_attribute("action", "set_art_reveal")
    //         .add_attribute("base_uri", &art_reveal.to_string()))
    // }

    // fn execute_free_mint(
    //     &self,
    //     deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     msg: FreeMintMsg<Extension>,
    // ) -> Result<Response, ContractError> {
    //     let freemint_count: u64 = self
    //         .freemint_count
    //         .may_load(deps.storage)?
    //         .unwrap_or_default();

    //     if info.sender != self.minter.load(deps.storage)? {
    //         return Err(ContractError::NotMinter {});
    //     }

    //     if freemint_count >= 1 {
    //         return Err(ContractError::FreeLimitExceeded {});
    //     };

    //     let token_id: &str = &(freemint_count + 3001).to_string()[..];
    //     // create the token
    //     let token = TokenInfo {
    //         owner: deps.api.addr_validate(&msg.owner)?,
    //         approvals: vec![],
    //         token_uri: Some(format!("{}{}.json", BASE_URI, token_id)),
    //         extension: msg.extension,
    //     };

    //     self.tokens
    //         .update(deps.storage, token_id, |old| match old {
    //             Some(_) => Err(ContractError::Claimed {}),
    //             None => Ok(token),
    //         })?;

    //     let old_balance = self.wallet_balance.may_load(deps.storage, &info.sender)?;
    //     let new_balance = match old_balance {
    //         None => 1,
    //         Some(val) => val + 1,
    //     };
    //     self.wallet_balance
    //         .save(deps.storage, &info.sender, &new_balance)?;

    //     self.freemint_count
    //         .save(deps.storage, &(freemint_count + 1))?;

    //     Ok(Response::new()
    //         .add_attribute("action", "free_mint")
    //         .add_attribute("minter", info.sender)
    //         .add_attribute("owner", msg.owner)
    //         .add_attribute("token_id", token_id))
    // }

    // fn execute_sign(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    // ) -> Result<Response, ContractError> {
    //     if info.sender == ADDR_TEAM || info.sender == ADDR_PRO || info.sender == ADDR_TREAS {
    //         self.cw3_signature
    //             .save(deps.storage, &info.sender, &(true))?;
    //         return Ok(Response::new()
    //             .add_attribute("action", "sign_for_withdraw")
    //             .add_attribute("signer", info.sender)
    //             .add_attribute("time", &env.block.time.seconds().to_string()));
    //     } else {
    //         return Err(ContractError::NotSigner {});
    //     }
    // }

    // fn execute_add_whitelist(
    //     &self,
    //     deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     member: String,
    // ) -> Result<Response, ContractError> {
    //     if info.sender != self.minter.load(deps.storage)? {
    //         return Err(ContractError::NotMinter {});
    //     }
    //     self.whitelist
    //         .save(deps.storage, &Addr::unchecked(member.clone()), &(true))?;
    //     Ok(Response::new()
    //         .add_attribute("action", "add_to_whitelist")
    //         .add_attribute("member", &member))
    // }

    // fn execute_remove_whitelist(
    //     &self,
    //     deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     member: String,
    // ) -> Result<Response, ContractError> {
    //     if info.sender != self.minter.load(deps.storage)? {
    //         return Err(ContractError::NotMinter {});
    //     }
    //     self.whitelist
    //         .save(deps.storage, &Addr::unchecked(member.clone()), &(false))?;
    //     Ok(Response::new()
    //         .add_attribute("action", "remove_from_whitelist")
    //         .add_attribute("member", &member))
    // }

    fn execute_add_extension(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_id: String,
        ext: Extension,
    ) -> Result<Response, ContractError> {
        if info.sender != self.minter.load(deps.storage)? {
            return Err(ContractError::NotMinter {});
        }
        let token = TokenInfo {
            owner: Addr::unchecked("not_yet_set"),
            approvals: vec![],
            token_uri: Some(String::from("not_yet_set")),
            extension: ext.clone(),
        };

        self.tokens
            .update(deps.storage, &&token_id[..], |old| match old {
                Some(pre_token) => match pre_token.owner == "not_yet_set" {
                    false => Err(ContractError::Claimed {}),
                    true => Ok(token),
                },
                None => Ok(token),
            })?;
        Ok(Response::new()
            .add_attribute("action", &format!("add extension for TOKEN #{}", token_id))
            .add_attribute("extension.image", &ext.unwrap().image.unwrap()))
    }

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
    ) -> Result<Response<Empty>, ContractError> {
        // only members of the multisig can create a proposal
        let vote_power = self
            .VOTERS
            .may_load(deps.storage, &info.sender)?
            .ok_or(ContractError::Unauthorized {})?;

        let cfg = self.CONFIG.load(deps.storage)?;

        // max expires also used as default
        let max_expires = cfg.max_voting_period.after(&env.block);
        let mut expires = latest.unwrap_or(max_expires);
        let comp = expires.partial_cmp(&max_expires);
        if let Some(Ordering::Greater) = comp {
            expires = max_expires;
        } else if comp.is_none() {
            return Err(ContractError::WrongExpiration {});
        }

        // create a proposal
        let mut prop = Proposal {
            title,
            description,
            start_height: env.block.height,
            expires,
            msgs,
            status: Status::Open,
            votes: Votes::yes(vote_power),
            threshold: cfg.threshold,
            total_weight: cfg.total_weight,
        };
        prop.update_status(&env.block);
        let id = self.next_id(deps.storage)?;
        self.PROPOSALS
            .save(deps.storage, &id.to_string()[..], &prop)?;

        // add the first yes vote from voter
        let ballot = Ballot {
            weight: vote_power,
            vote: Vote::Yes,
        };
        self.BALLOTS
            .save(deps.storage, (&id.to_string()[..], &info.sender), &ballot)?;

        Ok(Response::new()
            .add_attribute("action", "propose")
            .add_attribute("sender", info.sender)
            .add_attribute("proposal_id", id.to_string())
            .add_attribute("status", format!("{:?}", prop.status)))
    }

    fn execute_vote(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        proposal_id: u64,
        vote: Vote,
    ) -> Result<Response<Empty>, ContractError> {
        // only members of the multisig with weight >= 1 can vote
        let voter_power = self.VOTERS.may_load(deps.storage, &info.sender)?;
        let vote_power = match voter_power {
            Some(power) if power >= 1 => power,
            _ => return Err(ContractError::Unauthorized {}),
        };

        // ensure proposal exists and can be voted on
        let mut prop = self
            .PROPOSALS
            .load(deps.storage, &proposal_id.to_string()[..])?;
        if prop.status != Status::Open {
            return Err(ContractError::NotOpen {});
        }
        if prop.expires.is_expired(&env.block) {
            return Err(ContractError::Expired {});
        }

        // cast vote if no vote previously cast
        self.BALLOTS.update(
            deps.storage,
            (&proposal_id.to_string()[..], &info.sender),
            |bal| match bal {
                Some(_) => Err(ContractError::AlreadyVoted {}),
                None => Ok(Ballot {
                    weight: vote_power,
                    vote,
                }),
            },
        )?;

        // update vote tally
        prop.votes.add_vote(vote, vote_power);
        prop.update_status(&env.block);
        self.PROPOSALS
            .save(deps.storage, &proposal_id.to_string()[..], &prop)?;

        Ok(Response::new()
            .add_attribute("action", "vote")
            .add_attribute("sender", info.sender)
            .add_attribute("proposal_id", proposal_id.to_string())
            .add_attribute("status", format!("{:?}", prop.status)))
    }

    fn execute_execute(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        proposal_id: u64,
    ) -> Result<Response, ContractError> {
        // anyone can trigger this if the vote passed

        let mut prop = self
            .PROPOSALS
            .load(deps.storage, &proposal_id.to_string()[..])?;
        // we allow execution even after the proposal "expiration" as long as all vote come in before
        // that point. If it was approved on time, it can be executed any time.
        if prop.status != Status::Passed {
            return Err(ContractError::WrongExecuteStatus {});
        }

        // set it to executed
        prop.status = Status::Executed;
        self.PROPOSALS
            .save(deps.storage, &proposal_id.to_string()[..], &prop)?;

        // dispatch all proposed messages
        Ok(Response::new()
            .add_messages(prop.msgs)
            .add_attribute("action", "execute")
            .add_attribute("sender", info.sender)
            .add_attribute("proposal_id", proposal_id.to_string()))
    }

    fn execute_close(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        proposal_id: u64,
    ) -> Result<Response<Empty>, ContractError> {
        // anyone can trigger this if the vote passed

        let mut prop = self
            .PROPOSALS
            .load(deps.storage, &proposal_id.to_string()[..])?;
        if [Status::Executed, Status::Rejected, Status::Passed]
            .iter()
            .any(|x| *x == prop.status)
        {
            return Err(ContractError::WrongCloseStatus {});
        }
        if !prop.expires.is_expired(&env.block) {
            return Err(ContractError::NotExpired {});
        }

        // set it to failed
        prop.status = Status::Rejected;
        self.PROPOSALS
            .save(deps.storage, &proposal_id.to_string()[..], &prop)?;

        Ok(Response::new()
            .add_attribute("action", "close")
            .add_attribute("sender", info.sender)
            .add_attribute("proposal_id", proposal_id.to_string()))
    }
}
