use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};
use crate::state::ChronoVault;






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
    
    // chronoaccount
    #[account(
        init,
        payer = signer,
        space = ChronoVault::INIT_SPACE + ChronoVault::DISCRIMINATOR.len(),
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
     pub token_program:Interface<'info,TokenInterface>,
     pub system_program:Program<'info,System>, 
}

impl <'info> DepositeStruct <'info>{
    
    pub fn initialize_account(&mut self,seed:u64,lock_duration:u64, bump:u8) -> Result<()>{

       let clock = Clock::get()?;
       let current_time = clock.unix_timestamp as u64;
       let unlock_time = current_time + lock_duration;

       self.chrono_account.set_inner(ChronoVault { 
          seed: seed,
          depositer: self.signer.key(), 
          recipient: self.recipient_key.key(), 
          mint: self.mint.key(),
          deposit_time: current_time,
          unlock_time: unlock_time,
          bump: bump 
        });

        Ok(())
    }


    pub fn deposite_tokens(&self,amount:u64) -> Result<()>{
      
      transfer_checked(
        CpiContext::new(self.token_program.to_account_info(), 
        TransferChecked { 
            from: self.user_ata.to_account_info(), 
            mint: self.mint.to_account_info(), 
            to: self.vault.to_account_info(),
            authority: self.signer.to_account_info() 
        }),
        amount,
        self.mint.decimals)?;

        Ok(())
    }

}


pub fn helper(ctx:Context<DepositeStruct>,seed:u64,amount:u64,lock_duration:u64) -> Result<()> {
     ctx.accounts.initialize_account(seed, lock_duration, ctx.bumps.chrono_account)?;
     ctx.accounts.deposite_tokens(amount)    
}


