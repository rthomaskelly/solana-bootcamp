import argparse
from typing import NamedTuple
import struct
import base64

from solana.publickey import PublicKey
from solana.transaction import AccountMeta, TransactionInstruction, Transaction
from solana.system_program import CreateAccountParams, create_account, SYS_PROGRAM_ID

from solana.keypair import Keypair
from solana.rpc.api import Client
from solana.rpc.types import TxOpts
from solana.rpc.commitment import Confirmed

pack_str = lambda s: struct.pack("<I" + (len(s) * "B"), len(s), *s.encode("ascii"))

class AuthorizedBuffer(NamedTuple):
    bump_seed: int
    buffer_seed: int
    text: str

    @staticmethod
    def parse(data):
        decoded = base64.b64decode(data)
        print(len(decoded))
        bump_seed, buffer_seed = struct.unpack("<BQ", decoded[0:9])
        text = decoded[9:].decode("ascii").rstrip("\0")
        return AuthorizedBuffer(bump_seed=bump_seed, buffer_seed=buffer_seed, text=text)


class InitializeAuthorizedEchoParams(NamedTuple):
    program_id: PublicKey
    authorized_buffer: PublicKey
    authority: PublicKey
    buffer_seed: int
    buffer_size: int


class AuthorizedEchoParams(NamedTuple):
    program_id: PublicKey
    authorized_buffer: PublicKey
    authority: PublicKey
    data: str


def initialize_authorized_echo(
    params: InitializeAuthorizedEchoParams,
) -> TransactionInstruction:
    data = struct.pack("<BQQ", 1, params.buffer_seed, params.buffer_size)

    return TransactionInstruction(
        keys=[
            AccountMeta(
                pubkey=params.authorized_buffer, is_signer=False, is_writable=True
            ),
            AccountMeta(pubkey=params.authority, is_signer=True, is_writable=False),
            AccountMeta(pubkey=SYS_PROGRAM_ID, is_signer=False, is_writable=False),
        ],
        program_id=params.program_id,
        data=data,
    )


def authorized_echo(
    params: AuthorizedEchoParams, authority_is_signer: bool = True
) -> TransactionInstruction:
    data = b"".join([struct.pack("<B", 2), pack_str(params.data)])

    # authority should always sign the transaction in normal use
    # provide the ability to not sign for the purposes of testing
    return TransactionInstruction(
        keys=[
            AccountMeta(
                pubkey=params.authorized_buffer, is_signer=False, is_writable=True
            ),
            AccountMeta(
                pubkey=params.authority,
                is_signer=authority_is_signer,
                is_writable=False,
            ),
        ],
        program_id=params.program_id,
        data=data,
    )

def pack_str(s: str):
    return struct.pack("<I" + (len(s) * "B"), len(s), *s.encode("utf-8"))


def decode_str(data: bytes):
    return base64.b64decode(data).decode("ascii").rstrip("\0")

def get_authorized_echo_pda(
    authority: PublicKey, buffer_seed: int, program_id: PublicKey
):
    seeds = [b"authority", bytes(authority), struct.pack("<q", buffer_seed)]
    return PublicKey.find_program_address(seeds, program_id)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("program_id", help="Devnet program ID (base58 encoded string) of the deployed Echo Program")
    parser.add_argument("echo", help="The string to copy on-chain")
    args = parser.parse_args()
    buffer_seed = 0
    program_id = PublicKey(args.program_id)
    echo = args.echo
    authority = Keypair()
    client = Client("https://api.devnet.solana.com")
    print("Requesting Airdrop of 1 SOL...")
    client.request_airdrop(authority.public_key, int(1e9))
    print("Airdrop received")
# find PDA address to pass to instruciton
    pda_address, bump_seed = get_authorized_echo_pda(authority.public_key, buffer_seed, program_id)
    
    initialize_authorized_echo_ix = initialize_authorized_echo(
        InitializeAuthorizedEchoParams(
            program_id=program_id,
            authorized_buffer=pda_address,
            authority=authority.public_key,
            buffer_seed=0,
            buffer_size=20)
        )
    
    authorized_echo_ix = authorized_echo(
        AuthorizedEchoParams(
            program_id=program_id,
            authorized_buffer=pda_address,
            authority=authority.public_key,
            data=echo
        ), True
    )

    tx = Transaction().add(initialize_authorized_echo_ix, authorized_echo_ix)
    # signers = fee_payer
    result = client.send_transaction(
        tx,
        authority,
        opts=TxOpts(
            skip_preflight=True,
        ),
    )
    tx_hash = result["result"]
    client.confirm_transaction(tx_hash, commitment="confirmed")

    print(f"https://explorer.solana.com/tx/{tx_hash}?cluster=devnet")

    # acct_info = client.get_account_info(pda_address, commitment=Confirmed)
    # if acct_info["result"]["value"] is None:
    #     raise RuntimeError(f"Failed to get account. address={pda_address}")
    # data = base64.b64decode(acct_info["result"]["value"]["data"][0]).decode("ascii")
    # print("Echo Buffer Text:", data)
    
    data = client.get_account_info(
        pda_address, commitment=Confirmed
    )['result']['value']['data'][0]

    print(AuthorizedBuffer.parse(data))
