use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};
use crate::error::ChronoVaultError;
use crate::state::ChronoVault;






#[derive(Accounts)]
pub struct withdrawStruct<'info> {

// signer
#[account(mut)]
pub recipient:Signer<'info>,

// depositer 
#[account(mut)]
pub depositer:SystemAccount<'info>,

// mint 
pub mint:InterfaceAccount<'info,Mint>,

// chronoaccount
#[account(
    mut,
    close = depositer,
    seeds = [b"chrono_vault",depositer.key().as_ref(),chrono_account.seed.to_le_bytes().as_ref()],
    bump = chrono_account.bump,
    has_one =  depositer @ ChronoVaultError::InvalidDepsiter,
    has_one =  mint @ ChronoVaultError::InvalidMint,
    has_one = recipient @ ChronoVaultError::InvalidRecipient
)]
pub chrono_account:Account<'info,ChronoVault>,


// vault
#[
    account(
        mut,
       associated_token::mint = mint,
       associated_token::authority = chrono_account,
       associated_token::token_program = token_program
    )
]
pub vault:InterfaceAccount<'info,TokenAccount>,


// recipient ata
#[
    account(
        init_if_needed,
        payer = recipient,
        associated_token::mint = mint,
        associated_token::authority = recipient,
        associated_token::token_program = token_program
    )
]

pub recipient_ata:InterfaceAccount<'info,TokenAccount>

}