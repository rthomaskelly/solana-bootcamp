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
            EchoInstruction::Echo { data } => {
                msg!("Instruction: Echo");
                let accounts_iter = &mut _accounts.iter();
                let account_info = next_account_info(accounts_iter)?;
                msg!("Trying to echo message '{:?}' onto account '{}'", 
                     data, *account_info.key);
                let mut echo_buffer = EchoBuffer::try_from_slice(&account_info.data.borrow_mut())?;
                echo_buffer.data = data;
                msg!("Copied the data. echo_buffer.data '{:?}'", echo_buffer.data);
                // let wrapped_data = EchoBuffer::try_from_slice(&data);
                msg!("Copied the data. About to serialize...");
                echo_buffer.serialize(&mut *account_info.data.borrow_mut())?;
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
}
