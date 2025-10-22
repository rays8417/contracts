# Tenjaku Token Swap 🔄

A decentralized Automated Market Maker (AMM) built on Solana, enabling token swaps, liquidity provision, and decentralized exchange functionality using the Anchor framework.

## Overview

The Tenjaku Token Swap program enables users to:
- Create and manage Automated Market Makers (AMMs)
- Create liquidity pools for token pairs
- Deposit and withdraw liquidity
- Perform token swaps with configurable fees
- Execute token swaps with slippage protection

## Program Details

- **Program ID (Devnet)**: `7VFS76Bvrfj35GDho97B8HxgToW9Rpk6zLnx1VwwJhKD`
- **Language**: Rust (Anchor Framework)
- **Framework**: Anchor v0.30.0
- **Network**: Devnet

## Prerequisites

- [Rust](https://rustup.rs/) (v1.70+)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.14+)
- [Anchor](https://www.anchor-lang.com/docs/installation) (v0.30.0)
- [Node.js](https://nodejs.org/) (v16+)
- pnpm (for package management)

## Installation

1. Install dependencies:

```bash
pnpm install
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

### `create_amm`

Creates a new Automated Market Maker (AMM).

**Parameters:**
- `id` (Pubkey): Unique identifier for the AMM
- `fee` (u16): Trading fee in basis points (e.g., 250 = 2.5%)

**Accounts Required:**
- `admin`: Signer & AMM administrator
- `amm`: PDA account storing AMM data
- `system_program`: System program

### `create_pool`

Creates a new liquidity pool for a token pair within an AMM.

**Parameters:** None

**Accounts Required:**
- `amm`: The AMM account
- `pool`: PDA account for the pool
- `mint_a`: First token mint
- `mint_b`: Second token mint
- `payer`: Signer & transaction fee payer
- `system_program`: System program
- `token_program`: SPL Token program

### `deposit_liquidity`

Deposits tokens into a liquidity pool and receives LP tokens.

**Parameters:**
- `amount_a` (u64): Amount of token A to deposit
- `amount_b` (u64): Amount of token B to deposit

**Accounts Required:**
- `amm`: The AMM account
- `pool`: The liquidity pool
- `depositor`: Signer & liquidity provider
- `depositor_token_account_a`: Depositor's token A account
- `depositor_token_account_b`: Depositor's token B account
- `pool_token_account_a`: Pool's token A vault
- `pool_token_account_b`: Pool's token B vault
- `lp_mint`: Liquidity provider token mint
- `depositor_lp_token_account`: Depositor's LP token account
- `token_program`: SPL Token program

### `withdraw_liquidity`

Withdraws tokens from the pool by burning LP tokens.

**Parameters:**
- `amount` (u64): Amount of LP tokens to burn

**Accounts Required:**
- `amm`: The AMM account
- `pool`: The liquidity pool
- `withdrawer`: Signer & liquidity provider
- `withdrawer_lp_token_account`: Withdrawer's LP token account
- `withdrawer_token_account_a`: Withdrawer's token A account
- `withdrawer_token_account_b`: Withdrawer's token B account
- `pool_token_account_a`: Pool's token A vault
- `pool_token_account_b`: Pool's token B vault
- `lp_mint`: Liquidity provider token mint
- `token_program`: SPL Token program

### `swap_exact_tokens_for_tokens`

Performs a token swap with input amount specified and minimum output enforced.

**Parameters:**
- `swap_a` (bool): If true, swap token A for token B; if false, swap token B for token A
- `input_amount` (u64): Exact amount of input tokens
- `min_output_amount` (u64): Minimum acceptable output amount (slippage protection)

**Accounts Required:**
- `amm`: The AMM account
- `pool`: The liquidity pool
- `user`: Signer & swap initiator
- `user_input_token_account`: User's input token account
- `user_output_token_account`: User's output token account
- `pool_input_token_account`: Pool's input token vault
- `pool_output_token_account`: Pool's output token vault
- `token_program`: SPL Token program

## Usage Example

```rust
// Create an AMM with 2.5% fee
program.rpc
    .create_amm(
        amm_id,
        250, // 2.5% in basis points
    )
    .accounts(/* accounts */)
    .signers(&[/* signers */])
    .send()
    .await?;

// Swap token A for token B with slippage protection
program.rpc
    .swap_exact_tokens_for_tokens(
        true,           // swap_a = true (A to B)
        1_000_000,      // input 1 million tokens
        500_000,        // minimum 500k tokens output
    )
    .accounts(/* accounts */)
    .signers(&[/* signers */])
    .send()
    .await?;
```


## Error Codes

| Error | Description |
|-------|-------------|
| `InvalidFee` | Fee value is outside acceptable range |
| `InvalidMint` | Provided mint is not valid for the pool |
| `DepositTooSmall` | Liquidity deposit is below minimum threshold |
| `OutputTooSmall` | Swap output is below minimum expected (slippage exceeded) |
| `InvariantViolated` | AMM invariant (x * y = k) not maintained |
| `MathOverflow` | Mathematical operation resulted in overflow |

## Support

For issues or questions, please refer to the main project repository or create an issue.
