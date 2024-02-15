use crate::constants::*;
use crate::states::{Building, Town, Vector3D};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct InsertHouse<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        seeds = [constants::TOWN],
        bump,
    )]
    town: Account<'info, Town>,
    nft: Account<'info, Mint>,
}

pub fn insert_house_(
    ctx: Context<InsertHouse>,
    house_variant: u8,
    position: Vector3D,
    scale: Vector3D,
) -> Result<()> {
    let nft = ctx.accounts.nft.key().to_string();
    let building = Building {
        id: nft,
        house_variant,
        position,
        scale,
    };
    let buildings = &mut ctx.accounts.town.buildings;
    buildings.push(building);
    msg!("building numbers: {}", ctx.accounts.town.buildings.len());
    return Ok(());
}
