use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

declare_id!("HhaFfLGyd1SsyiCsuZQxHbgn18DU3jtHM9DrjZvuMPMN");

pub const MASTER_WALLET: Pubkey = pubkey!("71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK");
#[cfg(feature = "devnet")]
pub const USDC_MINT: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

#[program]
pub mod insight_faucet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, cap: u64) -> Result<()> {
        ctx.accounts.state_info.cap = cap;
        ctx.accounts.state_info.bump = ctx.bumps.state_info;
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        #[cfg(any(feature = "devnet"))]
        {
            if ctx.accounts.token.key() != USDC_MINT {
                return Err(ErrorCode::InvalidToken.into());
            }
        }

        let current_timestamp = Clock::get()?.unix_timestamp;

        if current_timestamp < ctx.accounts.recipient_info.next_claim {
            return Err(ErrorCode::AlreadyClaimed.into());
        }

        let cap = ctx.accounts.state_info.cap;

        let token = ctx.accounts.token.key();

        let seeds = &[
            b"state_info",
            token.as_ref(),
            &[ctx.accounts.state_info.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let transfer_accounts = TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.token.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.state_info.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );

        transfer_checked(
            cpi_context,
            cap,
            ctx.accounts.token.decimals,
        )?;

        ctx.accounts.recipient_info.next_claim = current_timestamp + (60 * 60 * 24);
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
        seeds = [b"state_info", token.key().as_ref()],
        bump
    )]
    pub state_info: Box<Account<'info, State>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token,
        associated_token::authority = state_info,
        associated_token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut, address = MASTER_WALLET)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub to: SystemAccount<'info>,

    #[account(mint::token_program = token_program)]
    pub token: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token,
        associated_token::authority = to,
        associated_token::token_program = token_program,
    )]
    pub recipient_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"state_info", token.key().as_ref()],
        bump = state_info.bump,
    )]
    pub state_info: Box<Account<'info, State>>,

    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = state_info,
        associated_token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,        
        payer = signer, 
        space = 8 + Recipient::INIT_SPACE,
        seeds = [b"recipient_info", to.key().as_ref()],
        bump
    )]
    pub recipient_info: Box<Account<'info, Recipient>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct State {
    pub cap: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Recipient {
    pub next_claim: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid token")]
    InvalidToken,
    #[msg("Already claimed")]
    AlreadyClaimed,
}
