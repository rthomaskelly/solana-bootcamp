use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, msg, 
    pubkey::Pubkey,
    program_error::ProgramError,
    program::{invoke_signed, },
};

use crate::{
    state::ExchangeBooth,
};

use spl_token::{
    id, instruction,
};

use borsh::{BorshDeserialize, };

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
    amount_to_withdraw: u64,
) -> ProgramResult {
    msg!("Processing withrdraw for '{}' tokens.", amount_to_withdraw);

    let accounts_iter = &mut accounts.iter();
    let admin = next_account_info(accounts_iter)?;
    let admins_token_account = next_account_info(accounts_iter)?;
    let vault = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let exchange_booth_acct = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    msg!("Got all six accounts.");

    assert_with_msg(
        admin.is_signer,
        ProgramError::MissingRequiredSignature,
        "First account passed for 'Admin' is not a signer as is required.",
    )?;

    msg!("Admin is a signer. Getting the Exchange Booth and validating the Admin's Public Key.");

    let exchange_booth = ExchangeBooth::try_from_slice(&exchange_booth_acct.data.borrow())?;

    assert_with_msg(
        exchange_booth.admin == *admin.key,
        ProgramError::InvalidArgument,
        "First account passed for 'Admin' does not have a Public Key \"
        matching the Admin known in the fifth argument 'Exchange Booth'.",
    )?;

    msg!("Basic checks passed. Creating transfer instruction.");

    let transfer_ix = instruction::transfer(
                token_program.key, // &id(),
                vault.key,
                admins_token_account.key,
                &vault.key,
                &[],
                amount_to_withdraw)?;

    msg!("Transfer instruction created. Calc'ing Vault bump seed.");

    let (vault_key, bump) = Pubkey::find_program_address(
        &[b"vault_b", exchange_booth_acct.key.as_ref()], program_id);

    assert_with_msg(
        vault_key == *vault.key,
        ProgramError::InvalidArgument,
        "Vault key from Find PDA doesn't match passed in Vault addr.",
    )?;

    msg!("Got right Vault PDA. Invoking the CPI.");

    invoke_signed(&transfer_ix,
        &[vault.clone(), admins_token_account.clone(), token_program.clone()], 
        &[&[b"vault_b", exchange_booth_acct.key.as_ref(), &[bump]]])?;

    msg!("CPI invoked succesffully! Withdraw complete!!");

    Ok(())
}
