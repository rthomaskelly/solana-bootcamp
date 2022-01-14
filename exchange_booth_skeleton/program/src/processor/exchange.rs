use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, msg, 
    pubkey::Pubkey,
    program_error::ProgramError
};

use crate::{
    error::ExchangeBoothError,
    state::ExchangeBooth,
};

use spl_token::{
    solana_program::program_pack::Pack,
    state::{Account as TokenAccount, Mint},
};

use borsh::{BorshDeserialize, BorshSerialize};

pub fn assert_with_msg(statement: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !statement {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    tokens_to_transfer: u64,
) -> ProgramResult {

    // Validate tokens to transfer
    assert_with_msg(std::assert_eq!(tokens_to_transfer, 0), ProgramError::InvalidArgument, "The number of tokens to transfer must be > 0");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get all the accounts that we need

    // User account that is initiating the exchange
    let user_account = next_account_info(accounts_iter)?;
    assert_with_msg(user_account.is_signer, ProgramError::MissingRequiredSignature, "The user account must be a signer");

    // User token accounts
    let user_token_a_account = next_account_info(accounts_iter)?;
    assert_with_msg(user_token_a_account.is_writable, ProgramError::MissingRequiredSignature, "The user token A account must be writable");

    let user_token_b_account = next_account_info(accounts_iter)?;
    assert_with_msg(user_token_b_account.is_writable, ProgramError::MissingRequiredSignature, "The user token B account must be writable");

    // Vault accounts that are PDAs
    let vault_a_pda = next_account_info(accounts_iter)?;
    let vault_a_pda_acc_info = TokenAccount::unpack(&vault_a_pda.data.borrow()).map_err(|_| ProgramError::InvalidAccountData)?;

    let vault_b_pda = next_account_info(accounts_iter)?;
    let vault_b_pda_acc_info = TokenAccount::unpack(&vault_b_pda.data.borrow()).map_err(|_| ProgramError::InvalidAccountData)?;

    // Mint accounts
    let mint_a_acc = next_account_info(accounts_iter)?;
    let vault_a_pda_acc_info = Mint::unpack(&mint_a_acc.data.borrow()).map_err(|_| ProgramError::InvalidAccountData)?;
    let mint_b_acc = next_account_info(accounts_iter)?;
    let vault_b_pda_acc_info = Mint::unpack(&mint_b_acc.data.borrow()).map_err(|_| ProgramError::InvalidAccountData)?;

    // Get the exchange booth account
    let exchange_booth_acc = next_account_info(accounts_iter)?;
    let exchange_booth = ExchangeBooth::try_from_slice(&exchange_booth_acc.data.borrow())?;

    // Validate the mint accounts with the corresponding mint accounts in the exchange booth
    Ok(())
}
