use crate::states::Building;
use anchor_lang::prelude::*;

#[account]
pub struct Town {
    pub name: String,
    pub buildings: Vec<Building>,
}
