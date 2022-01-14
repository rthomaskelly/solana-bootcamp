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

# data = b"".join([struct.pack("<B", 2), pack_str(params.data)])

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
    return result['result']['value'][0]['pubkey']

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
    
    print('Pubkey for Token Acct: ',
        create_token_account(
            client,
            admin.public_key, 
            mint,
            authority))
