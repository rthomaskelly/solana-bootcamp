import argparse
import struct
from typing import NamedTuple

from solana.publickey import PublicKey
from solana.keypair import Keypair
from solana.rpc.api import Client
from solana.rpc.types import TxOpts
from solana.rpc.commitment import Confirmed
from solana.system_program import CreateAccountParams, create_account, SYS_PROGRAM_ID
from solana.transaction import AccountMeta, TransactionInstruction, Transaction
from solana.sysvar import SYSVAR_RENT_PUBKEY
from spl.token.constants import TOKEN_PROGRAM_ID, MINT_LEN
from spl.token.instructions import (
    initialize_mint,
    InitializeMintParams,
    TOKEN_PROGRAM_ID,
    create_associated_token_account,
    get_associated_token_address,
    mint_to_checked,
    MintToCheckedParams,
)
from spl.token.client import Token

from utils import create_test_mint, send_transaction, mint_tokens_to

from our_utils import (
    pack_str,
    airdrop_sol_to_fee_payer,
    send_and_confirm_tx,
    create_token_account, 
    send_create_token_account_ix, 
    get_token_account_pubkey,
    get_token_account_balance,
)

from withdraw_client import (
    WithdrawParams,
    withdraw,
)

from init_client import (
   InitParams,
   init, 
)

def get_vault_a_pda(
    exchange_booth: PublicKey, program_id: PublicKey):
    seeds = [b'vault_a', bytes(exchange_booth)]
    return PublicKey.find_program_address(seeds, program_id)

def get_vault_b_pda(
    exchange_booth: PublicKey, program_id: PublicKey):
    seeds = [b'vault_b', bytes(exchange_booth)]
    return PublicKey.find_program_address(seeds, program_id)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("program_id", help="Devnet program ID (base58 encoded string) of the deployed Echo Program")
    parser.add_argument("amount_to_withdraw", help="The amount to withdraw out of the vault to the Admin's token account.")
    args = parser.parse_args()

    program_id = PublicKey(args.program_id)
    amount_to_withdraw = int(args.amount_to_withdraw)

    admin = Keypair()

    client = Client("https://api.devnet.solana.com")
    airdrop_sol_to_fee_payer(client, admin.public_key)

    mint_a_key = create_test_mint(admin)
    mint_b_key = create_test_mint(admin)
    
    exchange_booth_keypair = Keypair()

    vault_a_key, _ = get_vault_a_pda(
        exchange_booth_keypair.public_key, program_id)
    vault_b_key, _ = get_vault_b_pda(
        exchange_booth_keypair.public_key, program_id)

    oracle_keypair = Keypair()

    init(
        client,
        InitParams(
            program_id=program_id,
            admin=admin,
            exchange_booth_acct=exchange_booth_keypair.public_key,
            mint_a=mint_a_key,
            mint_b=mint_b_key,
            vault_a=vault_a_key,
            vault_b=vault_b_key,
            oracle=oracle_keypair.public_key))

    token_acct_key = mint_tokens_to(
        mint=mint_b_key,
        authority=admin,
        to_user=admin.public_key,
        amount=10)

    withdraw(
        client,
        WithdrawParams(
            program_id=program_id,
            admin=admin,
            admins_token_acct=token_acct_key,
            vault=vault_b_key,
            mint=mint_b_key,
            exchange_booth_acct=exchange_booth_keypair.public_key,
            token_program=TOKEN_PROGRAM_ID,
            amount_to_withdraw=amount_to_withdraw))

