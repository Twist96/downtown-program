use crate::constants::*;
use crate::states::Town;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateTown<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [constants::TOWN],
        bump,
        payer = signer,
        space = 8 + std::mem::size_of::<Town>()
    )]
    town: Account<'info, Town>,

    system_program: Program<'info, System>,
}
pub fn create_town_(ctx: Context<CreateTown>, name: String) -> Result<()> {
    ctx.accounts.town.name = name;
    ctx.accounts.town.buildings = Vec::new();
    Ok(())
}
