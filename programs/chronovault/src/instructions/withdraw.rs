use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, CloseAccount}, token_2022, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};
use crate::error::ChronoVaultError;
use crate::state::ChronoVault;






#[derive(Accounts)]
pub struct WithdrawStruct<'info> {

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

pub recipient_ata:InterfaceAccount<'info,TokenAccount>,

  //  programs
     pub associated_token_program:Program<'info,AssociatedToken>,
     pub token_program:Program<'info,TokenInterface>,
     pub system_program:Program<'info,System>, 

}



impl <'info> WithdrawStruct <'info>{
    
    pub fn withdraw_and_close_vault(&self) -> Result<()>{


        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp as u64;

        require!(
            current_time >= self.chrono_account.unlock_time,
            ChronoVaultError::TokensStillLocked
        );
        
     let signer_seeds:&[&[&[u8]]]= &[&[
            b"chrono_vault",
            self.chrono_account.depositer.as_ref(),
            &self.chrono_account.seed.to_le_bytes()[..],
            &[self.chrono_account.bump]
        ]];
        
        
        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked { 
                    from: self.vault.to_account_info(), 
                    mint: self.mint.to_account_info(), 
                    to: self.recipient.to_account_info(),
                    authority: self.chrono_account.to_account_info()
                },
                signer_seeds
                ), 
                    self.vault.amount, 
                    self.mint.decimals
        )?;


         close_account(CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
             CloseAccount { 
                account: self.vault.to_account_info(),
                destination: self.depositer.to_account_info(), 
                authority: self.chrono_account.to_account_info() 
            }
            , signer_seeds))?;

            Ok(())

    }
}