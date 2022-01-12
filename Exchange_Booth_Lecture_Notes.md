# Exchange Booth

## Needed fns
 - Create
 - Deposit
 - Withdraw
 - Exch

## Main structure outline
    struct EB {
        admin
        vault A
        vault B
        oracle
        mint A
        mint B
        verification
        fee structure
    }

### Create fn
Takes in list of Accounts and Data and is part of Exchange Booth Program.

Must haves:
  - Accounts
    - Admin
      - Must be a signer.
    - Mint A 
    - Mint B
    - Vault A
      - Must be writable. Will be created & allocated.
      - May want to make it a PDA.
    - Vault B
      - Must be writable. Will be created & allocated.
      - May want to make it a PDA.
    - Exchange Booth
      - Must be writable.
      - May want to make it a PDA.
    - Oracle Id
    - Token Program
    - System Program


#### Choices for possible Seeds if PDAs are Used.
Why don't Vault's need to be PDAs? Because authority that auth's the Vault is not necessarily the Vault itself. If the owner is a PDA, then the Vault needn't be.

Possible Seeds for Vaults if PDAs.
  - Could be (Mint, Admin).
    - But then a different Exchnage Booth would use the same Vault.
  - Could be only (Exchange Booth).

Possible Seeds for Exchange Booth if PDA.
  - Could be (Admin, Mint A, Mint B)
  - Could be (Admin, Mint A, Mint B, Oracle)

Pros and Cons to all choices so far. PDA seeds are just different ways of "keying a hashmap."

### Withdraw fn.
Sps Vault A is a PDA token Account associated with Mint A. Want to move Token from Vault A to some User's Token Account.
  - Want this to be permissionless.
    - "Way to have a program dictate authority is to use a PDA."
  - Who should own Vault A & B? [_Exercise for audience_]
    - And what should the seeds be?
      - No one right answer to above Exercise.

Fundamentally, Withdraw is simple. Is Token transfer from Vault back to User.

### Exchange fn.
Must haves:
  - Accounts
    - Exchange Booth
      - May be writable. Depends on impl.
    - Oracle
      - Need explicitly. Can't get from Exchange Booth. (Even though Exchange Booth knows it.)
      - Gives the "price" to use for the exchange.
    - Vault A
      - Need explicitly. Can't get from Exchange Booth. (Even though Exchange Booth knows it.)
      - Writable.
    - Vault B
      - Need explicitly. Can't get from Exchange Booth. (Even though Exchange Booth knows it.)
      - Writable.
    - Customer
      - Needs to be a signer.
    - Token Program
    - Customer Token Account
      - Writable.
    - PDA Signer
      - Not necessary if Vault A and B are owned by Exchange Booth or themselves.
      - Doesn't need to be a signer. Will be implicitly added as a signer to CPI.
    - Mint A
      - Needed for "decimals" of the Token.
    - Mint B
      - Needed for "decimals" of the Token.
  - Parameters
    - Amount to swap.
      - Associated "decimals" comes from Mint A.
    - Fee amount.
      - May be a flat fee, percent, or some other form.
    - Could take in "direction" (aka A to B or B to A).
      - Not necessary.
      - If provided, then only need one Vault.
  - Outputs
    - Yields the amount out.
      - Associated "decimals" comes from Mint B.

#### Representations of "Decimals" (Fixed point calcs)
Mint holds a field called `decimals: u8`
Token Account holds a field called  `amount: u64`

Calcing the decimal: amount * 10^-decimals

Gotchas:
  - Numeric overflow.
  - Rounding.

### Deposit fn.
Not discussed. Assumedly similar to Withdraw.
