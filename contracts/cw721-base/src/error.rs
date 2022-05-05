use crate::threshold::ThresholdError;
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("token_id already claimed")]
    Claimed {},

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Provided Fund Mismatch")]
    FundMismatch {},

    #[error("Wallet balance limit exceeded")]
    WalletLimitExceeded {},

    #[error("Presale Mint limit exceeded")]
    PresaleLimitExceeded {},

    #[error("All tokens sold out")]
    SoldOut {},

    #[error("Free Mint limit exceeded")]
    FreeLimitExceeded {},

    #[error("Not a minter")]
    NotMinter {},

    #[error("Not a signer")]
    NotSigner {},

    #[error("Not on whitelist")]
    NotWhitelist {},

    #[error("Not all signed")]
    NotAllSigned {},

    #[error("Cannot get extension")]
    CannotGetExtension {},

    #[error("Cannot execute your message, make sure if func exists")]
    CannotExecuteMsg {},

    #[error("{0}")]
    Threshold(#[from] ThresholdError),

    #[error("Required weight cannot be zero")]
    ZeroWeight {},

    #[error("Not possible to reach required (passing) weight")]
    UnreachableWeight {},

    #[error("No voters")]
    NoVoters {},
    #[error("Proposal is not open")]
    NotOpen {},

    #[error("Proposal voting period has expired")]
    VotingExpired {}, //////////////////////////////

    #[error("Proposal must expire before you can close it")]
    NotExpired {},

    #[error("Wrong expiration option")]
    WrongExpiration {},

    #[error("Already voted on this proposal")]
    AlreadyVoted {},

    #[error("Proposal must have passed and not yet been executed")]
    WrongExecuteStatus {},

    #[error("Cannot close completed or passed proposals")]
    WrongCloseStatus {},
}
