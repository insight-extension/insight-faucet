use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

declare_id!("HhaFfLGyd1SsyiCsuZQxHbgn18DU3jtHM9DrjZvuMPMN");

pub const MASTER_WALLET: Pubkey = pubkey!("71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK");

#[program]
pub mod insight_faucet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.bump = ctx.bumps.state;
        Ok(())
    }

    pub fn claim(_ctx: Context<Claim>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, address = MASTER_WALLET)]
    pub signer: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + State::INIT_SPACE,
        seeds = [b"state", token.key().as_ref()],
        bump
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token,
        associated_token::authority = state,
        associated_token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct State {
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct User {
    pub last_claimed_at: i64,
}
