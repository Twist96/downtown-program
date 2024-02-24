use crate::constants::constants;
use crate::states::custom_error::CustomError;
use crate::states::Building;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

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
        payer: &Signer<'info>,
        user_nft_token_account: &Account<'info, TokenAccount>,
        nft_token_account: &Account<'info, TokenAccount>,
        building: Building,
        bump: u8,
        system_program: &Program<'info, System>,
        token_program: &Program<'info, Token>,
    ) -> Result<()>;

    fn withdraw_building(
        &mut self,
        payer: &Signer<'info>,
        user_nft_token_account: &Account<'info, TokenAccount>,
        nft_token_account: &Account<'info, TokenAccount>,
        building_id: Pubkey,
        bump: u8,
        system_program: &Program<'info, System>,
        token_program: &Program<'info, Token>,
    ) -> Result<()>;

    fn realloc(
        &mut self,
        action: ReallocAction,
        space_to_add: usize,
        payer: &Signer<'info>,
        bump: u8,
        system_program: &Program<'info, System>,
    ) -> Result<()>;
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
        payer: &Signer<'info>,
        user_nft_token_account: &Account<'info, TokenAccount>,
        nft_token_account: &Account<'info, TokenAccount>,
        building: Building,
        bump: u8,
        system_program: &Program<'info, System>,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        match self.check_key(building.id) {
            true => {}
            false => {
                self.realloc(
                    ReallocAction::Increase,
                    Building::SPACE,
                    payer,
                    bump,
                    system_program,
                )?;
                self.buildings.push(building);

                // Deposit NFT
                transfer_token(
                    user_nft_token_account,
                    nft_token_account,
                    payer,
                    1,
                    token_program,
                )?;
                ()
            }
        }
        Ok(())
    }

    fn withdraw_building(
        &mut self,
        _payer: &Signer<'info>,
        user_nft_token_account: &Account<'info, TokenAccount>,
        nft_token_account: &Account<'info, TokenAccount>,
        building_id: Pubkey,
        _bump: u8,
        _system_program: &Program<'info, System>,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        match self.check_key(building_id) {
            true => {
                // self.realloc(
                //     ReallocAction::Decrease,
                //     Building::SPACE,
                //     payer,
                //     bump,
                //     system_program,
                // )?;
                self.buildings.retain(|building| building.id != building_id);
                transfer_lamports_from_vault(
                    nft_token_account,
                    user_nft_token_account,
                    token_program,
                    1,
                )?;
                ()
            }
            false => {}
        }
        Ok(())
    }

    fn realloc(
        &mut self,
        action: ReallocAction,
        space_to_add: usize,
        payer: &Signer<'info>,
        bump: u8,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        let account_info = self.to_account_info();

        match action {
            ReallocAction::Increase => {
                // transfer into pool account
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
            }
            ReallocAction::Decrease => {
                // transfer out of pool account
                let new_account_size = account_info.data_len() - space_to_add;
                let rent_to_withdraw = (Rent::get())?.minimum_balance(Building::SPACE);
                transfer_out_lamports(
                    account_info.clone(),
                    payer.to_account_info(),
                    rent_to_withdraw,
                    bump,
                    system_program,
                )?;
                account_info.realloc(new_account_size, false)?;
            }
        }

        Ok(())
    }
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

fn transfer_out_lamports<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    amount: u64,
    bump: u8,
    system_program: &Program<'info, System>,
) -> Result<()> {
    let signer: &[&[&[u8]]] = &[&[constants::TOWN, &[bump]]];

    system_program::transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            system_program::Transfer { from, to },
            signer,
        ),
        amount,
    )
}

pub enum ReallocAction {
    Increase,
    Decrease,
}
