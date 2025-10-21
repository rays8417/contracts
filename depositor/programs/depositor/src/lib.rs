use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("FjJCYKXSdtdqQcfJBZnofj2s3vxSR9dSHUdLTDSeBrLA");

#[program]
pub mod token_vault {
    use super::*;

    /// Initialize the vault with a specific authority and token mint
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.token_mint = ctx.accounts.token_mint.key();
        vault.bump = ctx.bumps.vault;
        
        msg!("Vault initialized with authority: {}", vault.authority);
        msg!("Accepted token mint: {}", vault.token_mint);
        
        Ok(())
    }

    /// Deposit tokens into the vault
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);
        
        let vault = &ctx.accounts.vault;
        
        // Transfer tokens from depositor to vault
        let cpi_accounts = Transfer {
            from: ctx.accounts.depositor_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        
        // Emit deposit event
        emit!(DepositEvent {
            depositor: ctx.accounts.depositor.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Deposited {} tokens from {}", amount, ctx.accounts.depositor.key());
        
        Ok(())
    }

    /// Withdraw all tokens from the vault (only authority can call this)
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let amount = ctx.accounts.vault_token_account.amount;
        
        require!(amount > 0, VaultError::NoFundsToWithdraw);
        
        // PDA seeds for signing
        let authority_key = vault.authority.key();
        let seeds = &[
            b"vault",
            authority_key.as_ref(),
            &[vault.bump],
        ];
        let signer = &[&seeds[..]];
        
        // Transfer all tokens from vault to authority
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.authority_token_account.to_account_info(),
            authority: vault.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::transfer(cpi_ctx, amount)?;
        
        msg!("Withdrawn {} tokens to authority {}", amount, ctx.accounts.authority.key());
        
        Ok(())
    }
}

// Account structures

#[account]
pub struct Vault {
    pub authority: Pubkey,      // The wallet authorized to withdraw
    pub token_mint: Pubkey,     // The SPL token mint accepted by this vault
    pub bump: u8,               // PDA bump seed
}

impl Vault {
    pub const LEN: usize = 8 + 32 + 32 + 1;
}

// Context structures

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = Vault::LEN,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: This is the token mint that will be accepted
    pub token_mint: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        seeds = [b"vault", vault.authority.as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub depositor: Signer<'info>,
    
    #[account(
        mut,
        constraint = depositor_token_account.mint == vault.token_mint @ VaultError::InvalidTokenMint,
        constraint = depositor_token_account.owner == depositor.key() @ VaultError::InvalidTokenAccount,
    )]
    pub depositor_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = vault_token_account.mint == vault.token_mint @ VaultError::InvalidTokenMint,
        constraint = vault_token_account.owner == vault.key() @ VaultError::InvalidVaultTokenAccount,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump,
        has_one = authority @ VaultError::UnauthorizedWithdraw,
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        constraint = vault_token_account.mint == vault.token_mint @ VaultError::InvalidTokenMint,
        constraint = vault_token_account.owner == vault.key() @ VaultError::InvalidVaultTokenAccount,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = authority_token_account.mint == vault.token_mint @ VaultError::InvalidTokenMint,
        constraint = authority_token_account.owner == authority.key() @ VaultError::InvalidTokenAccount,
    )]
    pub authority_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

// Events

#[event]
pub struct DepositEvent {
    pub depositor: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

// Errors

#[error_code]
pub enum VaultError {
    #[msg("Invalid amount: must be greater than 0")]
    InvalidAmount,
    
    #[msg("Invalid token mint: does not match vault's accepted token")]
    InvalidTokenMint,
    
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    
    #[msg("Invalid vault token account")]
    InvalidVaultTokenAccount,
    
    #[msg("Unauthorized: only the vault authority can withdraw")]
    UnauthorizedWithdraw,
    
    #[msg("No funds available to withdraw")]
    NoFundsToWithdraw,
}