use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("House not found in town")]
    BuildingNotFound,
    #[msg("Not enough sol in vault")]
    InsufficientVaultSol,
    #[msg("Not enough tokens to pay rent")]
    InsufficientRentVault,
    #[msg("Asset not owned by signer")]
    UnauthorizedSigner,
    #[msg("Asset not present in vault")]
    InsufficientVaultAsset,
}
