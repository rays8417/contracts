# Tenjaku Smart Contracts 

A comprehensive collection of Solana smart contracts built with Anchor, featuring token creation, decentralized token swaps, and secure token vaults.

##  Project Overview

This workspace contains three interconnected Solana programs designed to work together:

| Project | Description | Status |
|---------|-------------|--------|
| **Token Minter** | Creates custom SPL tokens with Metaplex metadata | ✅ Active |
| **Tenjaku Swap** | Decentralized AMM for token swaps and liquidity provision | ✅ Active |
| **Depositor (Token Vault)** | Secure vault for controlled token deposits/withdrawals | ✅ Active |

## 🏗️ Architecture

```
Cyberpunk Contracts
├── Token Minter
│   └── Creates SPL tokens with metadata
├── Tenjaku Swap (AMM)
│   ├── Creates liquidity pools
│   ├── Manages deposits/withdrawals
│   └── Executes token swaps
└── Depositor (Vault)
    ├── Stores tokens securely
    └── Enforces authorization rules
```



##  Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (v1.70+)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.14+)
- [Anchor](https://www.anchor-lang.com/docs/installation) (v0.30.0 - v0.31.1)
- [Node.js](https://nodejs.org/) (v16+)
- yarn or pnpm

### Configuration

Set up your Solana environment:

```bash
# Configure cluster
solana config set --url devnet

# Set your keypair
solana config set --keypair ~/.config/solana/id.json

# Get SOL airdrop for testing (if needed)
solana airdrop 2
```

### Building All Programs

```bash
# Build depositor
cd contracts/depositor
anchor build

# Build tenjaku_swap
cd ../tenjaku_swap
anchor build

# Build token_minter
cd ../token_minter
anchor build
```

### Deploying Programs

```bash
# Deploy from each project directory
cd contracts/depositor
anchor deploy

cd ../tenjaku_swap
anchor deploy

cd ../token_minter
anchor deploy
```

## Program Documentation

### 1. Token Minter 🪙

**Purpose:** Create custom SPL tokens with full Metaplex metadata support.

**Key Features:**
- Create SPL tokens with custom decimals
- Attach metadata (name, symbol, URI)
- Mint initial token supply (20 million by default)
- Solana Program ID: `Eu53phZD5Yok9zfYebBP29jAH5DEZKqjKgu88G7tnXrP`

**Instructions:**
- `create_token_mint` - Creates a new token with metadata

📖 [Full Documentation](./token_minter/README.md)

---

### 2. Tenjaku Token Swap 🔄

**Purpose:** Decentralized Automated Market Maker (AMM) for trading tokens and providing liquidity.

**Key Features:**
- Create custom AMMs with configurable fees
- Create liquidity pools for token pairs
- Deposit/withdraw liquidity with LP tokens
- Execute token swaps with slippage protection
- Maintain constant product formula (x * y = k)
- Solana Program ID: `7VFS76Bvrfj35GDho97B8HxgToW9Rpk6zLnx1VwwJhKD`

**Instructions:**
- `create_amm` - Creates a new AMM
- `create_pool` - Creates a liquidity pool
- `deposit_liquidity` - Deposits tokens and receives LP tokens
- `withdraw_liquidity` - Burns LP tokens and receives underlying tokens
- `swap_exact_tokens_for_tokens` - Swaps tokens with slippage protection

📖 [Full Documentation](./tenjaku_swap/README.md)

---

### 3. Depositor (Token Vault) 🏦

**Purpose:** Secure vault for controlled token storage with authorization rules.

**Key Features:**
- Initialize vaults for specific token mints
- Deposit tokens with event tracking
- Authority-only withdrawals
- Comprehensive validation and error handling
- Solana Program ID: `FjJCYKXSdtdqQcfJBZnofj2s3vxSR9dSHUdLTDSeBrLA`

**Instructions:**
- `initialize` - Creates a vault for a specific token
- `deposit` - Deposits tokens with event emission
- `withdraw` - Authority withdraws all vault funds

📖 [Full Documentation](./depositor/README.md)

---
