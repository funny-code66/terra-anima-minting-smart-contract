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
}
