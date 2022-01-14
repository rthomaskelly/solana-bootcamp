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

def create_exchange_booth(client, key: Keypair, program_id, fee_payer: Keypair):
    create_account_ix = create_account(
            CreateAccountParams(
                from_pubkey=fee_payer.public_key,
                new_account_pubkey=key.public_key,
                lamports=client.get_minimum_balance_for_rent_exemption(193)[
                    "result"
                ],
                space=193,
                program_id=program_id,
            )
        )
    tx = Transaction().add(create_account_ix)
    send_and_confirm_tx(client, tx, [key, fee_payer])

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
    
    vault_a_keypair = Keypair()
    vault_b_keypair = Keypair()
    exchange_booth_keypair = Keypair()
    # airdrop_sol_to_fee_payer(client, exchange_booth_keypair.public_key)
    create_exchange_booth(client, exchange_booth_keypair,
        program_id, admin)
    oracle_keypair = Keypair()

    init(
        client,
        InitParams(
            program_id=program_id,
            admin=admin,
            exchange_booth_acct=exchange_booth_keypair.public_key,
            mint_a=mint_a_key,
            mint_b=mint_b_key,
            vault_a=vault_a_keypair.public_key,
            vault_b=vault_b_keypair.public_key,
            oracle=oracle_keypair.public_key))



