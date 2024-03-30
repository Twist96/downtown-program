use crate::states::{Building, Town, TownAccount, Vector3D};
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct InsertHouse<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::seed_constants::TOWN],
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
        init_if_needed,
        payer = signer,
        seeds = [constants::seed_constants::VAULT, nft_mint.key().as_ref()],
        bump,
        token::mint = nft_mint,
        token::authority = nft_vault
    )]
    nft_vault: Account<'info, TokenAccount>,
    nft_mint: Account<'info, Mint>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn insert_house(
    ctx: Context<InsertHouse>,
    house_variant: u8,
    position: Vector3D,
    scale: Vector3D,
) -> Result<()> {
    let building = Building {
        id: ctx.accounts.nft_mint.key(),
        owner: ctx.accounts.signer.key(),
        house_variant,
        position,
        scale,
    };
    let town = &mut ctx.accounts.town;
    town.insert_building(
        (
            &ctx.accounts.user_nft_ata,
            &ctx.accounts.nft_vault,
            &ctx.accounts.signer,
        ),
        building,
        &ctx.accounts.system_program,
        &ctx.accounts.token_program,
    )?;

    msg!("building numbers: {}", ctx.accounts.town.buildings.len());
    return Ok(());
}
