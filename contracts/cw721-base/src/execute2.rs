use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;
use crate::traits::*;

const BASE_URI: &str = "ipfs://QmRiLKmhizpnwqpHGeiJnL4G6fsPAxdEdCiDkuJpt7xHPH/";

impl<'a> Cw721ExtendedContract<'a> {
    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::FreeMint(msg) => self.execute_free_mint(deps, env, info, msg),
            ExecuteMsg::Withdraw {} => self.execute_withdraw(deps, env, info),
            ExecuteMsg::SetBaseUri { base_uri } => {
                self.execute_set_base_uri(deps, env, info, base_uri)
            }
            ExecuteMsg::SetArtReveal { art_reveal } => {
                self.execute_set_art_reveal(deps, env, info, art_reveal)
            }
            ExecuteMsg::Sign {} => self.execute_sign(deps, env, info),
            ExecuteMsg::AddWhitelist { member } => {
                self.execute_add_whitelist(deps, env, info, member)
            }
            ExecuteMsg::RemoveWhitelist { member } => {
                self.execute_remove_whitelist(deps, env, info, member)
            }
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
        let team = self.team.load(deps.storage)?;
        let pro = self.pro.load(deps.storage)?;
        let treas = self.treas.load(deps.storage)?;

        let team_signed = self
            .cw3_signature
            .may_load(deps.storage, &team)?
            .unwrap_or(false);
        let pro_signed = self
            .cw3_signature
            .may_load(deps.storage, &pro)?
            .unwrap_or(false);
        let treas_signed = self
            .cw3_signature
            .may_load(deps.storage, &treas)?
            .unwrap_or(false);

        if !team_signed || !pro_signed || !treas_signed {
            return Err(ContractError::NotAllSigned {});
        }

        let current_uluna_amount = deps
            .querier
            .query_balance(env.contract.address.to_string(), "uluna")?
            .amount;

        let team_portion = current_uluna_amount * Uint128::from(30u128) / Uint128::from(100u128);
        let pro_portion = current_uluna_amount * Uint128::from(14u128) / Uint128::from(100u128);
        let treas_portion = current_uluna_amount * Uint128::from(56u128) / Uint128::from(100u128);

        self.cw3_signature.save(deps.storage, &team, &(false))?;
        self.cw3_signature.save(deps.storage, &pro, &(false))?;
        self.cw3_signature.save(deps.storage, &treas, &(false))?;

        let mut messages: Vec<CosmosMsg> = vec![];
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: team.to_string(),
            amount: vec![Coin {
                denom: String::from("uluna"),
                amount: team_portion,
            }],
        }));
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: pro.to_string(),
            amount: vec![Coin {
                denom: String::from("uluna"),
                amount: pro_portion,
            }],
        }));
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: treas.to_string(),
            amount: vec![Coin {
                denom: String::from("uluna"),
                amount: treas_portion,
            }],
        }));
        Ok(Response::new().add_messages(messages))
    }

    fn execute_set_base_uri(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        _uri: String,
    ) -> Result<Response, ContractError> {
        if info.sender != self.minter.load(deps.storage)? {
            return Err(ContractError::NotMinter {});
        }
        self.base_uri.save(deps.storage, &_uri)?;

        Ok(Response::new()
            .add_attribute("action", "set_base_uri")
            .add_attribute("base_uri", &_uri))
    }

    fn execute_set_art_reveal(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        art_reveal: bool,
    ) -> Result<Response, ContractError> {
        if info.sender != self.minter.load(deps.storage)? {
            return Err(ContractError::NotMinter {});
        }
        self.is_on_reveal.save(deps.storage, &art_reveal)?;

        Ok(Response::new()
            .add_attribute("action", "set_art_reveal")
            .add_attribute("base_uri", &art_reveal.to_string()))
    }

    fn execute_free_mint(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: FreeMintMsg<Extension>,
    ) -> Result<Response, ContractError> {
        let freemint_count: u64 = self
            .freemint_count
            .may_load(deps.storage)?
            .unwrap_or_default();

        if info.sender != self.minter.load(deps.storage)? {
            return Err(ContractError::NotMinter {});
        }

        if freemint_count >= 1 {
            return Err(ContractError::FreeLimitExceeded {});
        };

        let token_id: &str = &(freemint_count + 3001).to_string()[..];
        // create the token
        let token = TokenInfo {
            owner: deps.api.addr_validate(&msg.owner)?,
            approvals: vec![],
            token_uri: Some(format!("{}{}.json", BASE_URI, token_id)),
            extension: msg.extension,
        };

        self.tokens
            .update(deps.storage, token_id, |old| match old {
                Some(_) => Err(ContractError::Claimed {}),
                None => Ok(token),
            })?;

        let old_balance = self.wallet_balance.may_load(deps.storage, &info.sender)?;
        let new_balance = match old_balance {
            None => 1,
            Some(val) => val + 1,
        };
        self.wallet_balance
            .save(deps.storage, &info.sender, &new_balance)?;

        self.freemint_count
            .save(deps.storage, &(freemint_count + 1))?;

        Ok(Response::new()
            .add_attribute("action", "free_mint")
            .add_attribute("minter", info.sender)
            .add_attribute("owner", msg.owner)
            .add_attribute("token_id", token_id))
    }

    fn execute_sign(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let team = self.team.load(deps.storage)?;
        let pro = self.pro.load(deps.storage)?;
        let treas = self.treas.load(deps.storage)?;

        if info.sender == team || info.sender == pro || info.sender == treas {
            self.cw3_signature
                .save(deps.storage, &info.sender, &(true))?;
            return Ok(Response::new()
                .add_attribute("action", "sign_for_withdraw")
                .add_attribute("signer", info.sender)
                .add_attribute("time", &env.block.time.seconds().to_string()));
        } else {
            return Err(ContractError::NotSigner {});
        }
    }

    fn execute_add_whitelist(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        member: String,
    ) -> Result<Response, ContractError> {
        if info.sender != self.minter.load(deps.storage)? {
            return Err(ContractError::NotMinter {});
        }
        self.whitelist
            .save(deps.storage, &Addr::unchecked(member.clone()), &(true))?;
        Ok(Response::new()
            .add_attribute("action", "add_to_whitelist")
            .add_attribute("member", &member))
    }

    fn execute_remove_whitelist(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        member: String,
    ) -> Result<Response, ContractError> {
        if info.sender != self.minter.load(deps.storage)? {
            return Err(ContractError::NotMinter {});
        }
        self.whitelist
            .save(deps.storage, &Addr::unchecked(member.clone()), &(false))?;
        Ok(Response::new()
            .add_attribute("action", "remove_from_whitelist")
            .add_attribute("member", &member))
    }

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
        self.extensions
            .save(deps.storage, &Addr::unchecked(token_id.clone()), &ext)?;

        Ok(Response::new()
            .add_attribute("action", &format!("add extension for TOKEN #{}", token_id)))
    }
}
