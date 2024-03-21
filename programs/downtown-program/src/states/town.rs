use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[account]
pub struct Town {
    pub name: String,
    pub buildings: Vec<Building>,
}

impl Town {
    pub const SEED_PREFIX: &'static str = "town";
    pub const _SPACE: usize = 8 + 4 + Building::SPACE;

    pub fn new(name: String) -> Self {
        Self {
            name,
            buildings: vec![],
        }
    }
}

pub trait TownAccount<'info> {
    fn check_key(&self, id: Pubkey) -> bool;
    fn _get_building(&self, id: Pubkey) -> Result<Building>;

    fn insert_building(
        &mut self,
        deposit: (
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
            &Signer<'info>,
        ),
        building: Building,
        system_program: &Program<'info, System>,
        token_program: &Program<'info, Token>,
    ) -> Result<()>;

    fn withdraw_building(
        &mut self,
        withdraw_token_: (
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
            u8,
        ),
        nft: &Account<'info, Mint>,
        token_program: &Program<'info, Token>,
    ) -> Result<()>;

    fn realloc(
        &mut self,
        space_to_add: usize,
        payer: &Signer<'info>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn dealloc(&mut self, sol_receiver: &Signer<'info>, space_to_deduct: usize) -> Result<()>;
}

impl<'info> TownAccount<'info> for Account<'info, Town> {
    fn check_key(&self, id: Pubkey) -> bool {
        for building in &self.buildings {
            if building.id == id {
                return true;
            }
        }
        return false;
    }

    fn _get_building(&self, id: Pubkey) -> Result<Building> {
        for building in &self.buildings {
            if building.id == id {
                return Ok(building.clone());
            }
        }
        return Err(CustomError::BuildingNotFound.into());
    }

    fn insert_building(
        &mut self,
        deposit: (
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
            &Signer<'info>,
        ),
        building: Building,
        system_program: &Program<'info, System>,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        let (user_nft_ata, nft_vault, signer) = deposit;

        match self.check_key(building.id) {
            true => {}
            false => {
                self.realloc(Building::SPACE, signer, system_program)?;
                self.buildings.push(building);

                // Deposit NFT
                transfer_token(user_nft_ata, nft_vault, signer, 1, token_program)?;
                ()
            }
        }
        Ok(())
    }

    fn withdraw_building(
        &mut self,
        withdraw_token_: (
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
            u8,
        ),
        nft_mint: &Account<'info, Mint>,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        let (from, to, bump) = withdraw_token_;

        if from.amount != 1 {
            return Err(CustomError::InsufficientVaultAsset.into());
        }

        match self.check_key(nft_mint.key()) {
            true => {
                //remove building from list
                self.buildings
                    .retain(|building| building.id != nft_mint.key());

                //transfer lamports out
                // self.dealloc(signer, Building::SPACE)?;

                //transfer asset
                let nft_key = nft_mint.key();
                let seed: &[&[&[u8]]] = &[&[VAULT, nft_key.as_ref(), &[bump]]];
                withdraw_token(from, to, seed, token_program, 1)?;
                ()
            }
            false => {}
        }
        Ok(())
    }

    fn realloc(
        &mut self,
        space_to_add: usize,
        payer: &Signer<'info>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        let account_info = self.to_account_info();

        let new_account_size = account_info.data_len() + space_to_add;
        let lamport_required = (Rent::get())?.minimum_balance(new_account_size);
        let additional_rent_to_pay: u64 = lamport_required - account_info.lamports();
        transfer_lamports(
            payer,
            account_info.clone(),
            additional_rent_to_pay,
            system_program,
        )?;
        account_info.realloc(new_account_size, false)?;

        Ok(())
    }

    fn dealloc(&mut self, sol_receiver: &Signer<'info>, space_to_deduct: usize) -> Result<()> {
        //reduce size
        let account_info = self.to_account_info();
        let new_account_size = account_info.data_len() - space_to_deduct;
        account_info.realloc(new_account_size, false)?;

        //make withdrawal
        let rent_to_withdraw = (Rent::get())?.minimum_balance(Building::SPACE);
        withdraw_lamport(&self.to_account_info(), sol_receiver, rent_to_withdraw)?;
        Ok(())
    }
}

fn withdraw_lamport(
    from_account: &AccountInfo,
    to_account: &AccountInfo,
    amount_of_lamports: u64,
) -> Result<()> {
    if **from_account.try_borrow_mut_lamports()? < amount_of_lamports {
        return Err(CustomError::InsufficientVaultSol.into());
    }
    **from_account.try_borrow_mut_lamports()? -= amount_of_lamports;
    **to_account.try_borrow_mut_lamports()? += amount_of_lamports;
    Ok(())
}
