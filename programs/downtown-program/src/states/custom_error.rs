use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("House not found in town")]
    BuildingNotFound,
}
