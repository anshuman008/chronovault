use anchor_lang::prelude::*;


#[error_code]
pub enum ChronoVaultError {
    #[msg("Lock time is not completed yet")]
    TokensStillLocked,
    #[msg("Tokens are still locked - unlock time has not passed")]
    InvalidDepsiter,
    #[msg("Invalid recipient not authorized")]
    InvalidRecipient,
    #[msg("Invalid mint")]
    InvalidMint,
}