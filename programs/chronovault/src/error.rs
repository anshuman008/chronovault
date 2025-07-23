use anchor_lang::prelude::*;


#[error_code]
pub enum ChronoVaultError {
    #[msg("Lock time is not completed yet")]
    InvalidTime,
    #[msg("Invalid depositer")]
    InvalidMaker,
    #[msg("Invalid mint")]
    InvalidMint,
}