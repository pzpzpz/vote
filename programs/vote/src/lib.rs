use anchor_lang::prelude::*;

declare_id!("HpAaAcDB98tU7TGtRycGW4dGNHsqLgKwhimbawSdR8s8");

#[program]
pub mod vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
