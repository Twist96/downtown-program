mod instructions;
mod states;
mod utils;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("CgGCmVn7W9zjKjAqw3ypEQfEEiJGSM1u87AzyEC81m5b");

#[program]
pub mod downtown_program {
    use super::*;
    use crate::states::Vector3D;

    pub fn create_town(ctx: Context<CreateTown>, name: String) -> Result<()> {
        instructions::create_town(ctx, name)
    }

    pub fn insert_house(
        ctx: Context<InsertHouse>,
        house_variant: u8,
        position_x: i64,
        position_y: i64,
        position_z: i64,
    ) -> Result<()> {
        let position = Vector3D::new(position_x, position_y, Some(position_z));
        let scale = Vector3D::new(0, 0, Some(0));

        instructions::insert_house(ctx, house_variant, position, scale)
    }

    pub fn withdraw_house(ctx: Context<WithdrawHouse>) -> Result<()> {
        withdraw_house_(ctx)
    }

    pub fn fund_rent_vault(ctx: Context<FundRentVault>, amount: u64) -> Result<()> {
        instructions::fund_rent_vault(ctx, amount)
    }

    pub fn withdraw_rent_vault(ctx: Context<WithdrawRentVault>, amount: u64) -> Result<()> {
        instructions::withdraw_rent_vault(ctx, amount)
    }

    pub fn claim_rent(ctx: Context<ClaimRent>) -> Result<()> {
        instructions::claim_rent(ctx)
    }
}
