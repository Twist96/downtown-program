mod instructions;
mod states;
mod utils;

use crate::instructions::*;
use anchor_lang::prelude::*;

declare_id!("CgGCmVn7W9zjKjAqw3ypEQfEEiJGSM1u87AzyEC81m5b");

#[program]
pub mod downtown_program {
    use super::*;
    use crate::states::Vector3D;

    pub fn create_town(ctx: Context<CreateTown>, name: String) -> Result<()> {
        create_town_(ctx, name)
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

        insert_house_(ctx, house_variant, position, scale)
    }

    pub fn withdraw_house(ctx: Context<WithdrawHouse>) -> Result<()> {
        withdraw_house_(ctx)
    }
}
