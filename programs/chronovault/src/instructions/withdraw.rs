use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};
use crate::error::ChronoVaultError;
use crate::state::ChronoVault;






#[derive(Accounts)]
pub struct withdrawStruct<'info> {

// signer
#[account(mut)]
pub signer:Signer<'info>,

// mint of token
#[account(mut)]
pub depositer:SystemAccount<'info>,

// chronoaccount
#[account(
    mut,
    close = depositer,
    seeds = [b"chrono_vault",depositer.key().as_ref(),chrono_account.seed.to_le_bytes().as_ref()],
    bump = chrono_account.bump,
    has_one = maker 
)]

}