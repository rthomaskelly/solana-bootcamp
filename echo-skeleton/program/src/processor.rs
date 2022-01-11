use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::error::EchoError;
use crate::instruction::EchoInstruction;
use crate::state::EchoBuffer;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EchoInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            EchoInstruction::Echo { message_to_echo } => {
                msg!("Instruction: Echo");
                let accounts_iter = &mut _accounts.iter();
                let account_info = next_account_info(accounts_iter)?;
                msg!("Trying to echo message '{:?}' onto account '{}'", 
                     message_to_echo, *account_info.key);

                msg!("Setting buffer from account info.");
                let mut ai_buffer = &mut account_info.try_borrow_mut_data()?;
                msg!("Setting echo_buffer from buffer. Buffer has '{:?}'", ai_buffer);
                let mut echo_buffer = EchoBuffer::try_from_slice(&ai_buffer)?;
                msg!("Echo buffer data {:?}.", echo_buffer.data);

                // ai_buffer.get_mut()? = &mut message_to_echo;
                // message_to_echo.serialize(&mut &mut *ai_buffer)?;

                message_to_echo.serialize(&mut &mut account_info.data.borrow_mut()[..])?;

                msg!("Successful message echo!");

                Ok(())
            }
            EchoInstruction::InitializeAuthorizedEcho {
                buffer_seed: _,
                buffer_size: _,
            } => {
                msg!("Instruction: InitializeAuthorizedEcho");
                // Err(EchoError::NotImplemented.into())
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
        _accounts: &[AccountInfo],
        data: Vec<u8>,
    ) -> ProgramResult {
            let accounts_iter = &mut _accounts.iter();
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
}
