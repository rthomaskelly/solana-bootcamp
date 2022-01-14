use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};
use std::mem::size_of;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ExchangeBooth {
    pub initialized: bool,
    pub admin: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub vault_a: Pubkey, // token account controlled by PDA
    pub vault_b: Pubkey, // token account controlled by PDA
    pub oracle: Pubkey,
    // pub spread_bps: u64, // not using right away
}

impl ExchangeBooth {
    pub fn load_unchecked(ai: &AccountInfo) -> Result<Self, ProgramError> {
        Ok(Self::try_from_slice(&ai.data.borrow())?)
    }

    fn validate(&self) -> ProgramResult {
        // none
        Ok(())
    }

    pub fn load(ai: &AccountInfo) -> Result<Self, ProgramError> {
        let account = Self::try_from_slice(&ai.data.borrow())?;
        account.validate()?;
        Ok(account)
    }

    pub fn save(&self, ai: &AccountInfo) -> ProgramResult {
        Ok(self.serialize(&mut *ai.data.borrow_mut())?)
    }
}
