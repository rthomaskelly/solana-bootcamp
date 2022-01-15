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

from utils import create_test_mint, send_transaction

from our_utils import (
    pack_str,
    airdrop_sol_to_fee_payer,
    send_and_confirm_tx,
    create_token_account, 
    send_create_token_account_ix, 
    get_token_account_pubkey,
    get_token_account_balance,
    get_vault_pda
)

class InitParams(NamedTuple):
    program_id: PublicKey
    admin: Keypair
    exchange_booth_acct: PublicKey
    mint_a: PublicKey
    mint_b: PublicKey
    vault_a: PublicKey
    vault_b: PublicKey
    oracle: PublicKey

def init(client, params: InitParams) -> None:
    init_ix = get_init_ix(params)

    tx = Transaction().add(init_ix)
    send_and_confirm_tx(client, tx, [params.admin])


def get_init_ix(params: InitParams) -> TransactionInstruction:
    data = struct.pack("<B", 0)

    return TransactionInstruction(
        keys=[
            AccountMeta(
                pubkey=params.admin.public_key,
                is_signer=True,
                is_writable=False
            ),
            AccountMeta(
                pubkey=params.exchange_booth_acct,
                is_signer=False,
                is_writable=True,
            ),
            AccountMeta(
                pubkey=params.mint_a,
                is_signer=False,
                is_writable=False,
            ),
            AccountMeta(
                pubkey=params.mint_b,
                is_signer=False,
                is_writable=False,
            ),
            AccountMeta(
                pubkey=params.vault_a,
                is_signer=False,
                is_writable=True,
            ),
            AccountMeta(
                pubkey=params.vault_b,
                is_signer=False,
                is_writable=True,
            ),
            AccountMeta(
                pubkey=params.oracle,
                is_signer=False,
                is_writable=False,
            ),
            AccountMeta(
                pubkey=SYS_PROGRAM_ID,
                is_signer=False,
                is_writable=False,
            ),
            AccountMeta(
                pubkey=TOKEN_PROGRAM_ID,
                is_signer=False,
                is_writable=False,
            ),
            AccountMeta(
                pubkey=SYSVAR_RENT_PUBKEY,
                is_signer=False,
                is_writable=False,
            ),
        ],
        program_id=params.program_id,
        data=data,
    )

def create_exchange_booth(client, exchange_booth: Keypair, program_id, admin: Keypair, space):
    print(f"eb pk: {exchange_booth.public_key}, prog_id: {program_id}, admin pk: {admin.public_key}, space: {space}")
    create_account_ix = create_account(
            CreateAccountParams(
                from_pubkey=admin.public_key,
                new_account_pubkey=exchange_booth.public_key,
                lamports=client.get_minimum_balance_for_rent_exemption(space)[
                    "result"
                ],
                space=space,
                program_id=program_id,
            )
        )
    tx = Transaction().add(create_account_ix)
    send_and_confirm_tx(client, tx, [admin, exchange_booth])

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("program_id", help="Devnet program ID (base58 encoded string) of the deployed Echo Program")
    args = parser.parse_args()

    program_id = PublicKey(args.program_id)

    admin = Keypair()
    print('admins seed ', admin.seed)
    # admin = Keypair.from_seed(seed)

    client = Client("https://api.devnet.solana.com")
    airdrop_sol_to_fee_payer(client, admin.public_key)

    mint_a_key = create_test_mint(admin)
    mint_b_key = create_test_mint(admin)
    
    exchange_booth_keypair = Keypair()
    exchange_booth_space = 193
    # airdrop_sol_to_fee_payer(client, exchange_booth_keypair.public_key)
    create_exchange_booth(client, exchange_booth_keypair,
        program_id, admin, exchange_booth_space)
    oracle_keypair = Keypair()
    
    vault_a_pda, bump_seed_a = get_vault_pda(exchange_booth_keypair.public_key, program_id, b"vault_a")
    vault_b_pda, bump_seed_b = get_vault_pda(exchange_booth_keypair.public_key, program_id, b"vault_b")

    init(
        client,
        InitParams
        (
            program_id=program_id,
            admin=admin,
            exchange_booth_acct=exchange_booth_keypair.public_key,
            mint_a=mint_a_key,
            mint_b=mint_b_key,
            vault_a=vault_a_pda,
            vault_b=vault_b_pda,
            oracle=oracle_keypair.public_key
        ))



