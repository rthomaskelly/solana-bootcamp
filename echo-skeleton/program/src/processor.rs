use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
    program::{invoke_signed, invoke},
};

use crate::error::EchoError;
use crate::instruction::EchoInstruction;
use crate::state::EchoBuffer;

// use crate::byteorder::{BigEndian, LittleEndian, ReadBytesExt};
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
            EchoInstruction::Echo { message_to_echo } => {
                msg!("Instruction: Echo");
                let accounts_iter = &mut accounts.iter();
                let account_info = next_account_info(accounts_iter)?;
                msg!("Trying to echo message '{:?}' of length {} onto account '{}'", 
                     message_to_echo, message_to_echo.len(), *account_info.key);

                msg!("Account data len '{}'", 
                     account_info.data_len());
                msg!("Account data '{:?}'", 
                     account_info.data);

                let mut ai_buffer = account_info.data.borrow_mut();
                for i in 0..cmp::min(ai_buffer.len(), message_to_echo.len()) {
                    if ai_buffer[i] != 0 {
                        return Err(ProgramError::InvalidAccountData.into());
                    }

                    ai_buffer[i] = message_to_echo[i];
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

                msg!("Authority is a signer.");

                let (authorized_buffer_key, bump_seed) = Pubkey::find_program_address(
                    &[
                        b"authority",
                        authority.key.as_ref(),
                        &buffer_seed.to_le_bytes()
                    ],
                    program_id,
                );

                assert_with_msg(
                    authorized_buffer_key == *authority.key,
                    ProgramError::InvalidArgument,
                    "Key returned from find_program_address (while creating PDA) was not equal to the key passed as the 'authority' Account.",
                )?;

                // allocate buffer_size bytes to authorized_buffer using system_program
                // let sz: u64 = to_u64(buffer_size);
                let ix = solana_program::system_instruction::allocate(
                            authorized_buffer.key, buffer_size as u64);
                invoke_signed(&ix,
                    &[authorized_buffer.clone(), system_program.clone()], 
                    &[&[authority.key.as_ref(), &[bump_seed]]])?;

                let mut ab_buffer = authorized_buffer.data.borrow_mut();
                ab_buffer[0] = bump_seed;
                let seed_as_array: [u8; 8] = bytemuck::cast(buffer_seed);
                ab_buffer[1..8].copy_from_slice(&seed_as_array);


                Ok(())
            }
            EchoInstruction::AuthorizedEcho { data: _ } => {
                msg!("Instruction: AuthorizedEcho");
                // Err(EchoError::NotImplemented.into())
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
