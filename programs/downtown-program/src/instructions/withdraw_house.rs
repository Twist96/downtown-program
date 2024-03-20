use crate::constants::*;
use crate::states::{Town, TownAccount};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct WithdrawHouse<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
    mut,
    seeds = [constants::TOWN],
    bump,
    )]
    town: Account<'info, Town>,

    #[account(
    init_if_needed,
    payer = signer,
    associated_token::mint = nft_mint,
    associated_token::authority = signer
    )]
    user_nft_ata: Account<'info, TokenAccount>,

    #[account(
    mut,
    token::mint = nft_mint,
    token::authority = signer
    )]
    nft_vault: Account<'info, TokenAccount>,
    nft_mint: Account<'info, Mint>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn withdraw_house_<'info>(ctx: Context<WithdrawHouse>) -> Result<()> {
    let town = &mut ctx.accounts.town;
    let withdrawal = (
        &ctx.accounts.nft_vault,
        &ctx.accounts.user_nft_ata,
        &ctx.accounts.signer,
    );
    let withdraw_token = (
        &ctx.accounts.nft_vault,
        &ctx.accounts.user_nft_ata,
        ctx.bumps.town,
    );
    town.withdraw_building(
        withdrawal,
        withdraw_token,
        &ctx.accounts.nft_mint,
        &ctx.accounts.token_program,
    )?;

    return Ok(());
}
