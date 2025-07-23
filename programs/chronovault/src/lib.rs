use anchor_lang::prelude::*;

mod instructions;
pub use instructions::*;

mod state;
mod error;

declare_id!("AM37vnJ3mXiMSNSfaeTV1ZVicK7zoyxWqVrrBdnikUb");

#[program]
pub mod chronovault {

    use super::*;

    pub fn deposite(ctx:Context<DepositeStruct>, seed: u64, amount: u64, lock_duration: u64) -> Result<()> {
        instructions::deposite::helper(ctx, seed, amount, lock_duration)
    }


    pub fn withdraw(ctx:Context<WithdrawStruct>) -> Result<()> {
        ctx.accounts.withdraw()
    }

 

}