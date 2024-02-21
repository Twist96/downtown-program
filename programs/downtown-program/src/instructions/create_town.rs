// use crate::constants::*;
use crate::states::Town;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateTown<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [Town::SEED_PREFIX.as_bytes()],
        bump,
        payer = signer,
        space = 8 + std::mem::size_of::<Town>()
    )]
    town: Account<'info, Town>,

    system_program: Program<'info, System>,
}
pub fn create_town_(ctx: Context<CreateTown>, name: String) -> Result<()> {
    ctx.accounts.town.set_inner(Town::new(name));
    Ok(())
}
