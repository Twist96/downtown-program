use crate::states::vector3d::Vector3D;
use anchor_lang::prelude::*;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Building {
    pub id: Pubkey,
    pub owner: Pubkey,
    pub house_variant: u8,
    pub position: Vector3D,
    pub scale: Vector3D,
}

impl Building {
    pub const SPACE: usize = std::mem::size_of::<Building>();
    pub fn _new(
        id: Pubkey,
        owner: Pubkey,
        house_variant: u8,
        position: Vector3D,
        scale: Vector3D,
    ) -> Self {
        Self {
            id,
            owner,
            house_variant,
            position,
            scale,
        }
    }
}
