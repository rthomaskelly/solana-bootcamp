use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, msg, 
    pubkey::Pubkey,
};

use crate::{
    error::ExchangeBoothError,
    state::ExchangeBooth,
};

use borsh::{BorshDeserialize, BorshSerialize};


pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {

    Ok(())
}
