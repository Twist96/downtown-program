use anchor_lang::prelude::*;

// #[account]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Vector3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vector3D {
    pub fn new(x: i64, y: i64, z: Option<i64>) -> Self {
        match z {
            None => Vector3D { x, y, z: 0 },
            Some(z) => Vector3D { x, y, z },
        }
    }
}
