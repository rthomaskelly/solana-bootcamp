use solana_program::{
    system_instruction::create_account,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    account_info::{AccountInfo, next_account_info},
    program_pack::{Pack},
    entrypoint::ProgramResult, msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
    program::{invoke_signed, invoke},
};

use spl_token::{
    id, instruction, state::Account
};

use crate::{
    error::ExchangeBoothError,
    state::ExchangeBooth,
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
    accounts: &[AccountInfo]
    // ???
) -> ProgramResult {
    msg!("Instruction: InititializeExchangeBooth");
    let accounts_iter = &mut accounts.iter();

    let vault_size: usize = Account::LEN;
    // get accounts
    let administrator_ai = next_account_info(accounts_iter)?;
    let exchange_booth_ai = next_account_info(accounts_iter)?;
    let mint_a_ai = next_account_info(accounts_iter)?;
    let mint_b_ai = next_account_info(accounts_iter)?;
    let vault_a_ai = next_account_info(accounts_iter)?;
    let vault_b_ai = next_account_info(accounts_iter)?;
    let oracle_ai = next_account_info(accounts_iter)?;
    let system_program_ai = next_account_info(accounts_iter)?;
    let token_program_ai = next_account_info(accounts_iter)?;
    let rent_account_ai = next_account_info(accounts_iter)?;
    
    assert_with_msg(
        administrator_ai.is_signer,
        ProgramError::MissingRequiredSignature,
        "first account passed 'administrator' is not a signer as is required.",
    )?;

    // get PDAs of vaults
    let(vault_a_key, vault_a_bump) = Pubkey::find_program_address(
        &[
            b"vault_a",
            exchange_booth_ai.key.as_ref()  
        ],
        program_id    
    );
    
    msg!("vault a pda: '{:?}', bump: {}", vault_a_key, vault_a_bump);

    assert_with_msg(
        vault_a_key == *vault_a_ai.key,
        ProgramError::InvalidArgument,
        "Key returned from find_program_address (while creating PDA) was not equal to the key passed as the 'vault_a' Account.",
    )?;

    let(vault_b_key, vault_b_bump) = Pubkey::find_program_address(
        &[
            b"vault_b",
            exchange_booth_ai.key.as_ref()  
        ],
        program_id    
    );

    msg!("vault b pda: '{:?}', bump: {}", vault_b_key, vault_b_bump);

    assert_with_msg(
        vault_b_key == *vault_b_ai.key,
        ProgramError::InvalidArgument,
        "Key returned from find_program_address (while creating PDA) was not equal to the key passed as the 'vault_b' Account.",
    )?;

    let ix_create_vault_a = system_instruction::create_account(
       administrator_ai.key,
       vault_a_ai.key,
       Rent::get()?.minimum_balance(vault_size),
       vault_size as u64,
       token_program_ai.key
    );

    let ix_create_vault_b = system_instruction::create_account(
        administrator_ai.key,
        vault_b_ai.key,
        Rent::get()?.minimum_balance(vault_size),
        vault_size as u64,
        token_program_ai.key
     );

     // need all accts passed into instruction
     // signers = same keys that went into pda
     invoke_signed(&ix_create_vault_a,
        &[administrator_ai.clone(), vault_a_ai.clone(), system_program_ai.clone()], 
        &[&[b"vault_a", exchange_booth_ai.key.as_ref(), &[vault_a_bump]]])?;

    invoke_signed(&ix_create_vault_b,
        &[administrator_ai.clone(), vault_b_ai.clone(), system_program_ai.clone()], 
        &[&[b"vault_b", exchange_booth_ai.key.as_ref(), &[vault_b_bump]]])?;

     let ix_init_acct_vault_a = spl_token::instruction::initialize_account(
         token_program_ai.key, 
         &vault_a_key, 
         mint_a_ai.key, 
         &vault_a_key)?;

     let ix_init_acct_vault_b = spl_token::instruction::initialize_account(
        token_program_ai.key, 
        &vault_b_key, 
        mint_b_ai.key, 
        &vault_b_key)?;

    invoke_signed(&ix_init_acct_vault_a,
            &[token_program_ai.clone(), vault_a_ai.clone(), mint_a_ai.clone(), rent_account_ai.clone()], 
            &[&[b"vault_a", exchange_booth_ai.key.as_ref(), &[vault_a_bump]]])?;   

    invoke_signed(&ix_init_acct_vault_b,
                &[token_program_ai.clone(), vault_b_ai.clone(), mint_b_ai.clone(), rent_account_ai.clone()], 
                &[&[b"vault_b", exchange_booth_ai.key.as_ref(), &[vault_b_bump]]])?;  

     let mut exchange_booth = ExchangeBooth::load_unchecked(exchange_booth_ai)?;
                exchange_booth.admin = *administrator_ai.key;
                exchange_booth.mint_a = *mint_a_ai.key;
                exchange_booth.mint_b = *mint_b_ai.key;
                exchange_booth.vault_a = *vault_a_ai.key;
                exchange_booth.vault_b = *vault_b_ai.key;
                exchange_booth.oracle = *oracle_ai.key;
                exchange_booth.save(exchange_booth_ai)?;

    Ok(())
}
