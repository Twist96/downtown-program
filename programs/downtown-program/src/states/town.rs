use crate::constants::*;
use crate::states::custom_error::CustomError;
use crate::states::Building;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

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
        withdraw_lamport: (
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
            &Signer<'info>,
        ),
        withdraw_token: (
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
        withdraw_lamport: (
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
            &Signer<'info>,
        ),
        withdraw_token: (
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
            u8,
        ),
        nft: &Account<'info, Mint>,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        let (from, to, signer) = withdraw_lamport;
        let building = self._get_building(nft.key())?;

        if building.owner.eq(signer.key) {
            return Err(CustomError::UnauthorizedSigner.into());
        }

        match self.check_key(nft.key()) {
            true => {
                //remove building from list
                self.buildings.retain(|building| building.id != nft.key());

                //transfer lamports out
                transfer_lamports_from_vault(from, to, token_program, 1)?;
                self.dealloc(signer, Building::SPACE)?;

                let (from, to, bump) = withdraw_token;
                //transfer asset
                let signer: &[&[&[u8]]] = &[&[constants::VAULT, &[bump]]];
                transfer_token_from_program(from, to, signer, token_program)?;
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
        transfer_lamport(&self.to_account_info(), sol_receiver, rent_to_withdraw)?;
        Ok(())
    }
}

fn transfer_lamport(
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

fn transfer_token<'info>(
    from: &Account<'info, TokenAccount>,
    to: &Account<'info, TokenAccount>,
    authority: &Signer<'info>,
    amount: u64,
    token_program: &Program<'info, Token>,
) -> Result<()> {
    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
                authority: authority.to_account_info(),
            },
        ),
        amount,
    )
}

fn transfer_token_from_program<'info>(
    from: &Account<'info, TokenAccount>,
    to: &Account<'info, TokenAccount>,
    signer: &[&[&[u8]]],
    token_program: &Program<'info, Token>,
) -> Result<()> {
    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            {
                Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: from.to_account_info(),
                }
            },
            signer,
        ),
        1,
    )
}

fn transfer_lamports_from_vault<'info>(
    from: &Account<'info, TokenAccount>,
    to: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    let signer: &[&[&[u8]]] = &[&[]];

    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
                authority: from.to_account_info(),
            },
            signer,
        ),
        amount,
    )
}

fn transfer_lamports<'info>(
    from: &Signer<'info>,
    to: AccountInfo<'info>,
    amount: u64,
    system_program: &Program<'info, System>,
) -> Result<()> {
    system_program::transfer(
        CpiContext::new(
            system_program.to_account_info(),
            system_program::Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
            },
        ),
        amount,
    )
}
