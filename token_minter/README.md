# Token Minter 🪙

A Solana program that creates and mints custom SPL tokens with metadata support using the Metaplex Token Metadata standard.

## Overview

The Token Minter program enables users to:
- Create new SPL token mints
- Attach metadata (name, symbol, URI) to tokens using Metaplex Token Metadata

## Program Details

- **Program ID (Devnet)**: `Eu53phZD5Yok9zfYebBP29jAH5DEZKqjKgu88G7tnXrP`
- **Language**: Rust (Anchor Framework)
- **Network**: Devnet

## Prerequisites

- [Rust](https://rustup.rs/) (v1.70+)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.14+)
- [Anchor](https://www.anchor-lang.com/docs/installation) (v0.31.1)
- [Node.js](https://nodejs.org/) (v16+)
- pnpm or yarn (for package management)

## Installation

1. Install dependencies:

```bash
yarn install
# or
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

### `create_token_mint`

Creates a new SPL token with metadata.

**Parameters:**
- `token_decimals` (u8): Number of decimal places (e.g., 6 for USDC-like tokens)
- `token_name` (String): Display name of the token
- `token_symbol` (String): Ticker symbol (e.g., "TOKEN")
- `token_uri` (String): URI pointing to token metadata JSON

**Accounts Required:**
- `payer`: Signer & transaction fee payer
- `mint_account`: The new SPL token mint
- `metadata_account`: PDA for token metadata
- `token_account`: Token account to receive initial supply
- `token_program`: SPL Token program
- `token_metadata_program`: Metaplex Token Metadata program
- `system_program`: System program
- `rent`: Rent sysvar



## Usage Example

```rust
// Create a token with 6 decimal places
let token_name = "My Token".to_string();
let token_symbol = "MYT".to_string();
let token_uri = "https://example.com/token-metadata.json".to_string();

program.rpc
    .create_token_mint(
        6,
        token_name,
        token_symbol,
        token_uri,
    )
    .accounts(/* accounts */)
    .signers(&[/* signers */])
    .send()
    .await?;
```



## Support

For issues or questions, please refer to the main project repository or create an issue.
