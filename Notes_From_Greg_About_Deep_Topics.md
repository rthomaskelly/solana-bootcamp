Vaults and Mints revolve around Tokens

Solana has a concept of Tokens.
  - SOL is the native Token which is not really a Token.
  - USDC repr's USD. A company stores USD and issues USDC.
    - Known as a 'stable coin'.
  - Could repr BTC and ETH.
  - Could repr NFTs.
    - NFT: Non-fungible Token. Only one exists. Not tradeable.

When people say "Token" they mean "Mint".
Tokens are represented by a Token Account.
  - Mint is the addr of the Token.

Token Program
  - xfers tokens
  - creates tokens (known as "Minting")
  - destroys tokens (known as "Burning")
  - Knows how to interact with the Mint Accounts and all Token Accounts.
  - There is only one Token Program in all of Solana.

Token Account is for a user (aka, a real person)
  - Mint refers to the Token type (USDC, BTC, ETH).
    - Mint Authority is the public key for the Mint.
    - Each Token type has only one Mint.
    - Solana Explorer can show the Mint and even the biggest holders of a Token.
  - Has an "owner" (points to a Wallet address. That wallet has SOL).
    - But is actually owned by the Token Program.
  - Has a balance.

A Vault is temporary storage location for Tokens.
  - Move Tokens from Token Account into a Vault owned by an Exchange program.
    - Solves the problem of two people needing to sign the same tx.
    - Each party moves Tokens into Vaults, signing each individually.
      - Exchange Program is then trusted to transact properly.
  - The Vault is owned by a PDA
    - Allows the Exchange Program to sign to move out of Vault.
    - Limits Authority over Vaults to Exchange Program.
      - If it wasn't a PDA, then owner of Token Program could steal the Vault.

Prices don't belong to Tokens. An Exchange Program would have and deal with Prices.

Pyth is an Oracle that takes market data, aggregates it, and publishes it to Solana.

Signers
  - Signing a transaction means proving you know the private key for some pubkey.
    - Private Key is never actually sent or used.
  - PDA is to get a Program Id associated with some PublicKey.
    - Prevents other Programs from transacting with that PublicKey.


