use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    system_instruction::create_account,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
    program::{invoke_signed, invoke},
};

use crate::error::EchoError;
use crate::instruction::EchoInstruction;
use crate::state::EchoBuffer;

// use ::byteorder::{LittleEndian, ReadBytesExt};
use bytemuck::cast;

use std::cmp;

pub fn assert_with_msg(statement: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !statement {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EchoInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            EchoInstruction::Echo { data } => {
                msg!("Instruction: Echo");
                let accounts_iter = &mut accounts.iter();
                let account_info = next_account_info(accounts_iter)?;
                msg!("Trying to echo message '{:?}' of length {} onto account '{}'", 
                     data, data.len(), *account_info.key);

                msg!("Account data len '{}'", 
                     account_info.data_len());
                msg!("Account data '{:?}'", 
                     account_info.data);

                if account_info.data_len() == 0 {
                    return Err(ProgramError::InvalidAccountData.into());
                }
                let mut ai_buffer = account_info.data.borrow_mut();
                for i in 0..cmp::min(ai_buffer.len(), data.len()) {
                    if ai_buffer[i] != 0 {
                        return Err(ProgramError::InvalidAccountData.into());
                    }

                    ai_buffer[i] = data[i];
                }

                msg!("Successful message echo!");

                Ok(())
            }
            EchoInstruction::InitializeAuthorizedEcho {
                buffer_seed,
                buffer_size,
            } => {
                msg!("Instruction: InitializeAuthorizedEcho");
                let accounts_iter = &mut accounts.iter();
                let authorized_buffer = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;
                let system_program = next_account_info(accounts_iter)?;

                msg!("Got all three accounts.");

                assert_with_msg(
                    authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Second account passed 'Authority' is not a signer as is required.",
                )?;

                assert_with_msg(
                    buffer_size >=9,
                    ProgramError::InvalidArgument,
                    "buffer_size must be greater than or equal to 9"
                )?;

                msg!("Authority is a signer.");

                let (authorized_buffer_key, bump_seed) = Pubkey::find_program_address(
                    &[
                        b"authority",
                        authority.key.as_ref(),
                        &buffer_seed.to_le_bytes() // distinguishes different authorized_echo accounts for a single authority
                    ],
                    program_id,
                );
                // bump_seed is used to poke [ bump :) ] PublicKey of PDA off the SecretKey->PublicKey curve

                // authority = owner of buffer we are creating
                assert_with_msg(
                    authorized_buffer_key == *authorized_buffer.key,
                    ProgramError::InvalidArgument,
                    "Key returned from find_program_address (while creating PDA) was not equal to the key passed as the 'authority_buffer' Account.",
                )?;

                // allocate buffer_size bytes to authorized_buffer using system_program
                // let sz: u64 = to_u64(buffer_size);
                // let ix = solana_program::system_instruction::allocate(
                //             authorized_buffer.key, buffer_size as u64);

                // TODO left off here - let's look at the arguments
                // TODO pull up lecture 3 github project too

                // instruction to create auth buffer account with authority as owner
                let ix = system_instruction::create_account(
                    authority.key,
                    authorized_buffer.key,
                    Rent::get()?.minimum_balance(buffer_size),
                    buffer_size as u64,
                    program_id,
                );

                // need all accts passed into instruction
                invoke_signed(&ix,
                    &[authority.clone(), authorized_buffer.clone(), system_program.clone()], 
                    &[&[b"authority", authority.key.as_ref(), &buffer_seed.to_le_bytes(),
                        &[bump_seed]]])?;

                let mut ab_buffer = authorized_buffer.data.borrow_mut();
                ab_buffer[0] = bump_seed;
                let seed_as_array = buffer_seed.to_le_bytes();
                ab_buffer[1..9].copy_from_slice(&seed_as_array);

                Ok(())
            }
            EchoInstruction::AuthorizedEcho { data } => {
                msg!("Instruction: AuthorizedEcho");
                // TODO (similar to init echo. ensure proper auth / pda accounts
                // first 9 bytes = bump seed and buffer seed, remainder to write data

                let accounts_iter = &mut accounts.iter();
                let authorized_buffer = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;

                msg!("Got both accounts.");

                assert_with_msg(
                    authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Second account passed 'Authority' is not a signer as is required.",
                )?;

                let reserved_bytes_for_seeds_on_buffer = 9;

                assert_with_msg(
                    authorized_buffer.data_len() >= reserved_bytes_for_seeds_on_buffer,
                    ProgramError::InvalidAccountData,
                    "First Account 'Authorized Buffer' should have at least 9 bytes already allocated."
                )?;

                msg!("Basic checks passed. Verifying auth signs the buffer.");

                let mut buffer_data = authorized_buffer.data.borrow_mut();
                let bump_seed = buffer_data[0];
                let mut buffer_seed : u64 = 0;
                buffer_seed.to_le_bytes().copy_from_slice(&buffer_data[1..9]);

                /*
                let authority_seeds = 
                    &[authority.key.as_ref(), &buffer_seed.to_le_bytes(), 
                        &[bump_seed]];

                msg!("Got auth seeds. Verifying sign.");
                let auth_key = Pubkey::create_program_address(authority_seeds, program_id)?;
                assert_with_msg(
                    auth_key == *authority.key,
                    ProgramError::InvalidArgument,
                    "Invalid PDA seeds for authority",
                )?;
                */

                msg!("All checks passed. Setting up some constants.");

                // let mut echo_buffer = buffer_data[reserved_bytes_for_seeds_on_buffer..];
                let buffer_data_available_len = 
                    buffer_data.len() - reserved_bytes_for_seeds_on_buffer;
                let max_read_len = cmp::min(buffer_data_available_len, data.len());

                msg!("Fill buffer with data.");
                let mut i = 0;
                while i < max_read_len {
                    buffer_data[i + reserved_bytes_for_seeds_on_buffer] = data[i];
                    i += 1;
                }
                while i < buffer_data_available_len {
                    buffer_data[i + reserved_bytes_for_seeds_on_buffer] = 0;
                    i += 1;
                }
                
                msg!("Successful authorized echo!!");

                Ok(())
            }
            EchoInstruction::InitializeVendingMachineEcho {
                price: _,
                buffer_size: _,
            } => {
                msg!("Instruction: InitializeVendingMachineEcho");
                // Err(EchoError::NotImplemented.into())
                Ok(())
            }
            EchoInstruction::VendingMachineEcho { data: _ } => {
                msg!("Instruction: VendingMachineEcho");
                // Err(EchoError::NotImplemented.into())
                Ok(())
            }

        }
    }

    pub fn echo_impl1(
        accounts: &[AccountInfo],
        data: Vec<u8>,
    ) -> ProgramResult {
            let accounts_iter = &mut accounts.iter();
            let account_info = next_account_info(accounts_iter)?;
            msg!("Trying to echo message '{:?}' onto account '{}'", 
                     data, *account_info.key);
            msg!("Extra line to check.");
            let mut echo_buffer = EchoBuffer::try_from_slice(&account_info.data.borrow())?;
            msg!("Trying to copy data onto custom buffer.");
            // echo_buffer.data = data;
            msg!("Copied the data. About to serialize...");
            msg!("Copied the data. echo_buffer.data '{:?}'", echo_buffer.data);
            echo_buffer.serialize(&mut *account_info.data.borrow_mut())?;
            msg!("Successful message echo!");
            Ok(())
    }

    pub fn echo_impl2(
        accounts: &[AccountInfo],
        message_to_echo: Vec<u8>,
    ) -> ProgramResult {
            msg!("Instruction: Echo");
            let accounts_iter = &mut accounts.iter();
            let account_info = next_account_info(accounts_iter)?;
            msg!("Trying to echo message '{:?}' onto account '{}'", 
                 message_to_echo, *account_info.key);

            msg!("Setting buffer from account info.");
            let mut ai_buffer = &mut account_info.try_borrow_mut_data()?;
            msg!("Setting echo_buffer from buffer. Buffer has '{:?}'", ai_buffer);
            // message_to_echo.serialize(&mut &mut *ai_buffer)?;
            Ok(())
    }
}
