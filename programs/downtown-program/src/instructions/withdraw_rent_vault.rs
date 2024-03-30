use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct WithdrawVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
    associated_token::mint = token_mint,
    associated_token::authority = user_token_account
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
    mut,
    seeds = [constants::seed_constants::VAULT, constants::seed_constants::RENT],
    bump,
    token::mint = token_mint,
    token::authority = rent_vault
    )]
    pub rent_vault: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn withdraw_rent_vault(ctx: Context<WithdrawVault>, amount: u64) -> Result<()> {
    let signer: &[&[&[u8]]] = &[&[VAULT, RENT, &[ctx.bumps.rent_vault]]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.rent_vault.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.rent_vault.to_account_info(),
            },
            signer,
        ),
        amount,
    )
}
