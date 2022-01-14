use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use spl_token::{
    solana_program::program_pack::Pack,
    state::{Account as TokenAccount, Mint},
};

use crate::instruction::ExchangeBoothInstruction;

pub mod close_exchange_booth;
pub mod deposit;
pub mod exchange;
pub mod initialize_exchange_booth;
pub mod withdraw;

pub struct Processor {}

impl Processor {

    /// Unpacks a spl_token `Account`.
    pub fn unpack_token_account(
        account_info: &AccountInfo,
        token_program_id: &Pubkey,
    ) -> Result<TokenAccount, ProgramError> {
        if account_info.owner != token_program_id {
            return Err(ProgramError::IncorrectProgramId)
        } else {
            return TokenAccount::unpack(&account_info.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)
        }
    }

    /// Unpacks a spl_token `Mint`.
    pub fn unpack_mint(
        account_info: &AccountInfo,
        token_program_id: &Pubkey,
    ) -> Result<Mint, ProgramError> {
        if account_info.owner != token_program_id {
            Err(ProgramError::IncorrectProgramId)
        } else {
            Mint::unpack(&account_info.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)
        }
    }


    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = ExchangeBoothInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            ExchangeBoothInstruction::InititializeExchangeBooth { } => {
                msg!("Instruction: InitializeExchangeBooth");
                initialize_exchange_booth::process(program_id, accounts)?;
            }
            ExchangeBoothInstruction::Deposit { } => {
                msg!("Instruction: Deposit");
                deposit::process(program_id, accounts)?;
            }
            ExchangeBoothInstruction::Withdraw { amount_to_withdraw } => {
                msg!("Instruction: Withdraw");
                withdraw::process(program_id, accounts, amount_to_withdraw)?;
            }
            ExchangeBoothInstruction::Exchange {tokens_to_transfer} => {
                msg!("Instruction: Withdraw");
                exchange::process(program_id, accounts, tokens_to_transfer)?;
            }
            ExchangeBoothInstruction::CloseExchangeBooth { } => {
                msg!("Instruction: CloseExchangeBooth");
                close_exchange_booth::process(program_id, accounts)?;
            }
        }

        Ok(())
    }
}
