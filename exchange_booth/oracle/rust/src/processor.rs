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

        }
    }
}
