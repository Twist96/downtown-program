use crate::constants::*;
use crate::states::{Building, Town, TownAccount, Vector3D};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct InsertHouse<'info> {
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

pub fn insert_house_(
    ctx: Context<InsertHouse>,
    house_variant: u8,
    position: Vector3D,
    scale: Vector3D,
) -> Result<()> {
    let building = Building {
        id: ctx.accounts.nft.key(),
        house_variant,
        position,
        scale,
    };
    let town = &mut ctx.accounts.town;
    town.insert_building(
        &ctx.accounts.signer,
        building,
        ctx.bumps.town,
        &ctx.accounts.system_program,
    )?;

    msg!("building numbers: {}", ctx.accounts.town.buildings.len());
    return Ok(());
}
