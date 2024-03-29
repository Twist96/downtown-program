use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
use std::ops::Mul;

#[derive(Accounts)]
pub struct FundRentVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token_mint,
        associated_token::authority = user_token_account
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
    init_if_needed,
    seeds = [constants::seed_constants::VAULT, constants::seed_constants::RENT],
    bump,
    payer = signer,
    token::mint = token_mint,
    token::authority = rent_vault
    )]
    pub rent_vault: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn fund_rent_vault(ctx: Context<FundRentVault>, amount: u64) -> Result<()> {
    let total: u64 = amount.mul(10.pow(ctx.accounts.token_mint.decimals));
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.rent_vault.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        ),
        total,
    )
}
