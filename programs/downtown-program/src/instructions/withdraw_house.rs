use crate::constants::*;
use crate::states::{Town, TownAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

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
    nft: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_house_<'info>(ctx: Context<WithdrawHouse>, house_id: Pubkey) -> Result<()> {
    let town = &mut ctx.accounts.town;
    town.withdraw_building(
        &ctx.accounts.signer,
        house_id,
        ctx.bumps.town,
        &ctx.accounts.system_program,
    )?;

    return Ok(());
}
