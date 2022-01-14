use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use std::mem::size_of;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ExchangeBooth {
    pub initialized: bool,
    pub admin: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub vault_a: Pubkey, // token account controlled by PDA
    pub vault_b: Pubkey, // token account controlled by PDA
    pub oracle: Pubkey,
    // pub spread_bps: u64,
}
