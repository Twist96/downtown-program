use anchor_lang::prelude::*;

// #[account]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Vector3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
