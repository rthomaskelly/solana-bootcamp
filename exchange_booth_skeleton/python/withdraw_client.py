import argparse

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

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    # parser.add_argument("program_id", help="Devnet program ID (base58 encoded string) of the deployed Echo Program")
    # parser.add_argument("echo", help="The string to copy on-chain")
    # args = parser.parse_args()

    # program_id = PublicKey(args.program_id)

    authority = Keypair()
    admin = Keypair()
    client = Client("https://api.devnet.solana.com")
    print("Requesting Airdrop of 1 SOL...")
    client.request_airdrop(authority.public_key, int(1e9))
    print("Airdrop received")

    mint = create_test_mint(authority)

    txn = Transaction(fee_payer=authority.public_key)
    txn.add(
        create_associated_token_account(
            authority.public_key, admin.public_key, mint)
    )
    signers = [authority]
    result = send_transaction(txn, signers)
    print(result)


    print(client.get_account_info(
        admin.public_key, commitment=Confirmed
    ))

    token = Token(client, mint, TOKEN_PROGRAM_ID, authority.public_key)
    result = token.get_accounts(admin.public_key)
    print(result['result']['value'][0]['pubkey'])
