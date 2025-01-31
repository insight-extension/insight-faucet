use anchor_lang::prelude::*;

declare_id!("HhaFfLGyd1SsyiCsuZQxHbgn18DU3jtHM9DrjZvuMPMN");

#[program]
pub mod insight_faucet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
