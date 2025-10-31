# Token Vault (Depositor) 🏦

A secure token vault program built on Solana that enables controlled token deposits and authorized withdrawals with comprehensive event tracking and error handling.

## Overview

The Token Vault program enables users to:
- Initialize vaults for specific token mints with designated authorities
- Deposit tokens into vaults with event emission for tracking
- Withdraw all vault funds (authority-only)
- Enforce strict token and account validation

## Program Details

- **Program ID (Localnet)**: `FjJCYKXSdtdqQcfJBZnofj2s3vxSR9dSHUdLTDSeBrLA`
- **Language**: Rust (Anchor Framework)
- **Framework**: Anchor v0.31.1
- **Network**: Devnet

## Prerequisites

- [Rust](https://rustup.rs/) (v1.70+)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.14+)
- [Anchor](https://www.anchor-lang.com/docs/installation) (v0.31.1)
- [Node.js](https://nodejs.org/) (v16+)
- yarn (for package management)

## Installation

1. Install dependencies:

```bash
yarn install
```

2. Configure your Solana wallet:

```bash
solana config set --url devnet
solana config set --keypair ~/.config/solana/id.json
```

## Building

Build the program:

```bash
anchor build
```

Build for release:

```bash
anchor build --release
```

## Deployment

Deploy to Devnet:

```bash
anchor deploy
```

The deployed program ID will be displayed in the output.

## Program Instructions

### `initialize`

Creates a new vault with a specific authority and accepted token mint.

**Parameters:** None

**Accounts Required:**
- `vault`: PDA vault account to be created
- `authority`: Signer & vault administrator
- `token_mint`: The SPL token mint accepted by this vault
- `system_program`: System program

**Effect:** Creates a vault that can only accept deposits of the specified token mint.

### `deposit`

Deposits tokens into the vault and emits a deposit event.

**Parameters:**
- `amount` (u64): Amount of tokens to deposit

**Accounts Required:**
- `vault`: The vault account
- `depositor`: Signer & token owner
- `depositor_token_account`: Depositor's token account
- `vault_token_account`: Vault's token account
- `token_program`: SPL Token program

**Validation:**
- Amount must be greater than 0
- Token account mint must match vault's accepted token
- Depositor must own the depositor token account
- Vault must own the vault token account

**Event Emitted:** `DepositEvent` containing depositor, amount, and timestamp.

### `withdraw`

Withdraws all funds from the vault (authority-only operation).

**Parameters:** None

**Accounts Required:**
- `vault`: The vault account
- `authority`: Signer & vault authority
- `vault_token_account`: Vault's token account
- `authority_token_account`: Authority's token account
- `token_program`: SPL Token program

**Validation:**
- Only the vault authority can call this
- Vault must have funds available
- Token accounts must match vault's token mint

**Effect:** Transfers all tokens from vault to authority's account.

## Usage Example

```rust
// Initialize a vault for a specific token
program.rpc
    .initialize()
    .accounts({
        vault: vault_pda,
        authority: authority_keypair.public_key(),
        token_mint: token_mint_address,
        system_program: system_program::ID,
    })
    .signers(&[&authority_keypair])
    .send()
    .await?;

// Deposit tokens into the vault
program.rpc
    .deposit(1_000_000)  // Deposit 1 million tokens
    .accounts({
        vault: vault_pda,
        depositor: depositor_keypair.public_key(),
        depositor_token_account: depositor_ata,
        vault_token_account: vault_ata,
        token_program: spl_token::ID,
    })
    .signers(&[&depositor_keypair])
    .send()
    .await?;

// Withdraw all funds from vault (authority only)
program.rpc
    .withdraw()
    .accounts({
        vault: vault_pda,
        authority: authority_keypair.public_key(),
        vault_token_account: vault_ata,
        authority_token_account: authority_ata,
        token_program: spl_token::ID,
    })
    .signers(&[&authority_keypair])
    .send()
    .await?;
```


## Error Codes

| Error | Description |
|-------|-------------|
| `InvalidAmount` | Deposit amount must be greater than 0 |
| `InvalidTokenMint` | Token mint does not match vault's accepted token |
| `InvalidTokenAccount` | Token account is not valid or not owned by the signer |
| `InvalidVaultTokenAccount` | Vault's token account is invalid or not owned by the vault |
| `UnauthorizedWithdraw` | Only the vault authority can execute withdrawals |
| `NoFundsToWithdraw` | No tokens available in the vault to withdraw |

## Support

For issues or questions, please refer to the main project repository or create an issue.
