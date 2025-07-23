use anchor_lang::prelude::*;
mod instructions;

mod state;
declare_id!("GUXotqYYMotNh12quGgD6ovqcM3YRUC1t7axk1iDYWth");

#[program]
pub mod chronovault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
