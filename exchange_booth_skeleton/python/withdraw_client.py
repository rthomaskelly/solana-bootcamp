import argparse
import struct

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

pack_str = lambda s: struct.pack("<I" + (len(s) * "B"), len(s), *s.encode("ascii"))


def create_token_account(client, owner: PublicKey, mint: PublicKey, fee_payer: Keypair) -> PublicKey:
    send_create_token_account_ix(owner, mint, fee_payer)
    return get_token_account_pubkey(client, owner, mint, fee_payer)

def send_create_token_account_ix(owner: PublicKey, mint: PublicKey, fee_payer: Keypair) -> None:
    txn = Transaction(fee_payer=fee_payer.public_key)
    txn.add(
        create_associated_token_account(
            fee_payer.public_key, owner, mint)
    )
    signers = [fee_payer]
    result = send_transaction(txn, signers)
    print(result)

def get_token_account_pubkey(client, owner: PublicKey, mint: PublicKey, fee_payer: Keypair) -> int:
    token = Token(client, mint, TOKEN_PROGRAM_ID, fee_payer.public_key)
    result = token.get_accounts(owner)
    token_acct_key = result['result']['value'][0]['pubkey']

    print('Pubkey for Token Acct: ', token_acct_key)
    return token_acct_key

def airdrop_sol_to_fee_payer(client, fee_payer: PublicKey) -> None:
    print("Requesting Airdrop of 1 SOL...")
    client.request_airdrop(fee_payer, int(1e9))
    print("Airdrop received")

class WithdrawParams(NamedTuple):
    program_id: PublicKey
    admin: PublicKey
    admins_token_acct: PublicKey
    vault: PublicKey
    mint: PublicKey
    exchange_booth_acct: PublicKey
    token_program: PublicKey
    amount_to_withdraw: int

def withdraw(client, params: WithdrawParams, fee_payer: Keypair) -> int:
    withdraw_ix = get_withdraw_ix(params)

    tx = Transaction().add(withdraw_ix)
    send_and_confirm_tx(client, tx, [fee_payer])

    return get_token_account_balance(
        client, params.mint, params.admins_token_acct, fee_payer)

def get_withdraw_ix(params: WithdrawParams) -> TransactionInstruction:
    data = b"".join([struct.pack("<B", 2), pack_str(params.data)])

    return TransactionInstruction(
        keys=[
            AccountMeta(
                pubkey=params.admin,
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
        data=amount_to_withdraw,
    )

def send_and_confirm_tx(client, tx: Transaction, signers=list) -> None:
    result = client.send_transaction(
        tx,
        *signers,
        opts=TxOpts(
            skip_preflight=True,
        ),
    )
    tx_hash = result["result"]
    client.confirm_transaction(tx_hash, commitment="confirmed")
    print(f"https://explorer.solana.com/tx/{tx_hash}?cluster=devnet")

def get_token_account_balance(client, token_account: PublicKey,
                              mint: PublicKey, fee_payer: Keypair) -> int:
    token = Token(client, mint, TOKEN_PROGRAM_ID, fee_payer.public_key)
    result = token.get_balance(token_account)
    
    print('get_token_account_balance result:')
    print(result)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("program_id", help="Devnet program ID (base58 encoded string) of the deployed Echo Program")
    parser.add_argument("amount_to_withdraw", help="The amount to withdraw out of the vault to the Admin's token account.")
    args = parser.parse_args()

    program_id = PublicKey(args.program_id)
    amount_to_withdraw = args.amount_to_withdraw

    authority = Keypair()
    admin = Keypair()

    client = Client("https://api.devnet.solana.com")
    airdrop_sol_to_fee_payer(client, authority.public_key)

    mint_key = create_test_mint(authority)
    
    token_acct_key = create_token_account(
            client,
            admin.public_key, 
            mint_key,
            authority)

    withdraw(
        client,
        WithdrawParams(
            program_id=program_id,
            admin=admin.public_key,
            admins_token_acct=token_acct_key,
            vault=vault_key,
            mint=mint,
            exchange_booth_acct=exchange_booth_acct,
            token_program=TOKEN_PROGRAM_ID,
            amount_to_withdraw=amount_to_withdraw),
        authority)

