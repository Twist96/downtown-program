use crate::states::vector3d::Vector3D;
use anchor_lang::prelude::*;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Building {
    pub id: String,
    pub house_variant: u8,
    pub position: Vector3D,
    pub scale: Vector3D,
}
