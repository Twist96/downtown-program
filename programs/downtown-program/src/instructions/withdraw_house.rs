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
    user_nft_token_account: Account<'info, TokenAccount>,
    #[account(
    mut,
    associated_token::mint = nft_mint,
    associated_token::authority = signer
    )]
    nft_token_account: Account<'info, TokenAccount>,
    nft_mint: Account<'info, Mint>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn withdraw_house_<'info>(ctx: Context<WithdrawHouse>, house_id: Pubkey) -> Result<()> {
    let town = &mut ctx.accounts.town;
    town.withdraw_building(
        &ctx.accounts.signer,
        &ctx.accounts.user_nft_token_account,
        &ctx.accounts.nft_token_account,
        house_id,
        ctx.bumps.town,
        &ctx.accounts.system_program,
        &ctx.accounts.token_program,
    )?;

    return Ok(());
}
