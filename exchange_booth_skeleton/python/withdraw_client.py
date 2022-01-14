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

class WithdrawParams(NamedTuple):
    program_id: PublicKey
    admin: Keypair
    admins_token_acct: PublicKey
    vault: PublicKey
    mint: PublicKey
    exchange_booth_acct: PublicKey
    token_program: PublicKey
    amount_to_withdraw: int

def withdraw(client, params: WithdrawParams) -> int:
    withdraw_ix = get_withdraw_ix(params)

    tx = Transaction().add(withdraw_ix)
    send_and_confirm_tx(client, tx, [params.admin])

    # return get_token_account_balance(
    #     client, params.mint, params.admins_token_acct, params.admin)

def get_withdraw_ix(params: WithdrawParams) -> TransactionInstruction:
    data = struct.pack("<BQ", 2, params.amount_to_withdraw)

    return TransactionInstruction(
        keys=[
            AccountMeta(
                pubkey=params.admin.public_key,
                is_signer=True,
                is_writable=False
            ),
            AccountMeta(
                pubkey=params.admins_token_acct,
                is_signer=False,
                is_writable=True,
            ),
            AccountMeta(
                pubkey=params.vault,
                is_signer=False,
                is_writable=True,
            ),
            AccountMeta(
                pubkey=params.mint,
                is_signer=False,
                is_writable=False,
            ),
            AccountMeta(
                pubkey=params.exchange_booth_acct,
                is_signer=False,
                is_writable=False,
            ),
            AccountMeta(
                pubkey=TOKEN_PROGRAM_ID,
                is_signer=False,
                is_writable=False,
            ),
        ],
        program_id=params.program_id,
        data=data,
    )


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

    mint_key = create_test_mint(admin)
    
    token_acct_key = create_token_account(
            client,
            admin.public_key, 
            mint_key,
            admin)

    vault = Keypair()
    exchange_booth = Keypair()

    withdraw(
        client,
        WithdrawParams(
            program_id=program_id,
            admin=admin,
            admins_token_acct=token_acct_key,
            vault=vault.public_key,
            mint=mint_key,
            exchange_booth_acct=exchange_booth.public_key,
            token_program=TOKEN_PROGRAM_ID,
            amount_to_withdraw=amount_to_withdraw))

