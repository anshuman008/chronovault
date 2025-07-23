use anchor_lang::prelude::*;




#[derive(InitSpace)]
#[account(discriminator = 1)]
pub struct ChronoVault{
   pub seed: u64,
   pub depositer: Pubkey,
   pub recipient: Pubkey,
   pub mint: Pubkey,
   pub amount: u64,
   pub time: u64,
   pub bump: u8
}



