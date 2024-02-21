use crate::states::Building;
use anchor_lang::prelude::*;
use anchor_lang::system_program;

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

    fn insert_building(
        &mut self,
        payer: &Signer<'info>,
        building: Building,
        system_program: &Program<'info, System>,
    ) -> Result<()>;
    fn realloc(
        &mut self,
        space_to_add: usize,
        payer: &Signer<'info>,
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

    fn insert_building(
        &mut self,
        payer: &Signer<'info>,
        building: Building,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        match self.check_key(building.id) {
            true => {}
            false => {
                self.realloc(Building::SPACE, payer, system_program)?;
                self.buildings.push(building)
            }
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

        //determine additional rent
        let lamport_required = (Rent::get())?.minimum_balance(new_account_size);
        let additional_rent_to_pay = lamport_required - account_info.lamports();

        //make payment
        system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: payer.to_account_info(),
                    to: account_info.clone(),
                },
            ),
            additional_rent_to_pay,
        )?;

        account_info.realloc(new_account_size, false)?;
        Ok(())
    }
}
