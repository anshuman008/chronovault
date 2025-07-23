use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};
use crate::state::ChronoVault;





// deposit accounts


#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct DepositeStruct <'info> {

    // signer
    #[account(mut)]
    pub signer:Signer<'info>,

    // token mint
    #[account(
     mint::token_program = token_program,
    )]
    pub mint:InterfaceAccount<'info,Mint>,
    
    // chronovault
    #[account(
        init,
        payer = signer,
        space = ChronoVault::INIT_SPACE + ChronoVault::DESCRIMINATOR.len(),
        seeds =[b"chrono_vault",signer.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub chrono_account:Account<'info,ChronoVault>,
    
    // vault
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = chrono_account,
        associated_token::token_program = token_program
    )]
    pub vault:InterfaceAccount<'info,TokenAccount>,

   // user ata
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_ata:InterfaceAccount<'info,TokenAccount>,

    // recipient key
    pub recipient_key:SystemAccount<'info>,   


    //  programs
     pub associated_token_program:Program<'info,AssociatedToken>,
     pub token_program:Program<'info,TokenInterface>,
     pub system_program:Program<'info,System>, 
}


