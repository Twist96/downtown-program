use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct WithdrawHouse<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
    mut,
    seeds = [constants::seed_constants::TOWN],
    bump,
    )]
    town: Account<'info, Town>,

    #[account(
    // init_if_needed,
    // payer = signer,
    mut,
    associated_token::mint = nft_mint,
    associated_token::authority = signer
    )]
    user_nft_ata: Account<'info, TokenAccount>,

    #[account(
    mut,
    seeds = [constants::seed_constants::VAULT, nft_mint.key().as_ref()],
    bump
    )]
    nft_vault: Account<'info, TokenAccount>,
    nft_mint: Account<'info, Mint>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn withdraw_house_<'info>(ctx: Context<WithdrawHouse>) -> Result<()> {
    let town = &mut ctx.accounts.town;
    let withdraw_token = (
        &ctx.accounts.nft_vault,
        &ctx.accounts.user_nft_ata,
        ctx.bumps.nft_vault,
    );
    town.withdraw_building(
        withdraw_token,
        &ctx.accounts.nft_mint,
        &ctx.accounts.token_program,
    )?;

    return Ok(());
}
