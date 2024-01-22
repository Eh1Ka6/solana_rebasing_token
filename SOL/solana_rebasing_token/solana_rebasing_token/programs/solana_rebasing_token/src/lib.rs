use anchor_lang::prelude::*;
use crate::state::{RebasingTokenAccount, RebasingTokenExtension};
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};
mod state;
use state::*;
declare_id!("YourProgramID"); // Replace with your actual program ID.

#[program]
pub mod solana_rebasing_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_supply: u64) -> ProgramResult {
        let token_account = &mut ctx.accounts.token_account;
        token_account.total_supply = initial_supply;
        token_account.total_shares = initial_supply; // Initial 1:1 ratio
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        let token_account = &mut ctx.accounts.token_account;
        let user_account = &mut ctx.accounts.user_account;
        let user_token_account = &mut ctx.accounts.user_token_account;
        let program_token_account = &mut ctx.accounts.program_token_account;
        let token_program = &ctx.accounts.token_program;

    
        // Convert deposit amount to shares
        let shares = amount_to_shares(amount, token_account);
        user_account.shares += shares;
        token_account.total_shares += shares;

        // Perform the SPL token transfer
        let cpi_accounts = Transfer {
            from: user_token_account.to_account_info(),
            to: program_token_account.to_account_info(),
            authority: user.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
    
    // Implement the withdraw logic
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let token_account = &mut ctx.accounts.token_account;
        let user_account = &mut ctx.accounts.user_account;
        let user_token_account = &mut ctx.accounts.user_token_account;
        let program_token_account = &mut ctx.accounts.program_token_account;
        let token_program = &ctx.accounts.token_program;
    
        // Convert withdrawal amount from shares to token amount
        let shares = shares_to_amount(amount, token_account);
        
        // Ensure the user has enough shares to withdraw the specified amount
        if user_account.shares < shares {
            return Err(ProgramError::InsufficientFunds);
        }
    
        // Deduct the shares from the user's account
        user_account.shares -= shares;
        token_account.total_shares -= shares;
    
        // Perform the SPL token transfer from the program's account to the user's account
        let cpi_accounts = Transfer {
            from: program_token_account.to_account_info(),
            to: user_token_account.to_account_info(),
            authority: program_token_account.to_account_info(), // The program itself is the authority here
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
    
        Ok(())
    }
    
    pub fn rebase(ctx: Context<Rebase>, new_supply: u64) -> ProgramResult {
        let token_account = &mut ctx.accounts.token_account;
    
        // Ensure that the new supply is not zero to prevent division by zero in later calculations
        if new_supply == 0 {
            return Err(ProgramError::InvalidArgument);
        }
    
        // Update the total supply to the new supply
        token_account.total_supply = new_supply;
    
        // The share-to-token ratio is implicitly updated by changing the total_supply.
        // Since total_shares remain constant, the value of each share in terms of tokens changes.
    
        // Additional logic can be added here if needed, for example, to emit events or logs.
    
        Ok(())
    }
    
    
    /// Convert amount to shares based on current share-to-token ratio.
    /// 
    /// # Arguments
    /// * `amount` - The amount of tokens to convert to shares.
    /// * `token_account` - The account containing the token state, including total supply and shares.
    /// 
    /// # Returns
    /// The equivalent number of shares for the given token amount.
    fn amount_ui_shares(amount: u64, token_account: &Account<TokenState>) -> u64 {
        if token_account.total_shares == 0 || token_account.total_supply == 0 {
            // Edge case: If total shares or total supply is zero, treat the conversion ratio as 1:1
            return amount;
        }

        // Calculate the share-to-token ratio
        let ratio = token_account.total_shares as f64 / token_account.total_supply as f64;

        // Convert the token amount to shares using the ratio
        (amount as f64 * ratio).round() as u64
    }

    /// Convert shares to token amount based on current share-to-token ratio.
    /// 
    /// # Arguments
    /// * `shares` - The number of shares to convert to tokens.
    /// * `token_account` - The account containing the token state, including total supply and shares.
    /// 
    /// # Returns
    /// The equivalent token amount for the given number of shares.
    fn shares_to_amount(shares: u64, token_account: &Account<TokenState>) -> u64 {
        if token_account.total_shares == 0 {
            // Edge case: If total shares is zero, treat the conversion ratio as 1:1
            return shares;
        }

        // Calculate the token-to-share ratio
        let ratio = token_account.total_supply as f64 / token_account.total_shares as f64;

        // Convert the shares to token amount using the ratio
        (shares as f64 * ratio).round() as u64
    }
}

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(init, payer = user, space = 8 + size_of::<RebasingTokenAccount>())]
        pub token_account: Account<'info, RebasingTokenAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct Deposit<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        #[account(mut)]
        pub user_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        pub program_token_account: Account<'info, TokenAccount>,
        pub token_program: Program<'info, Token>,
        // ... other fields
    }

    #[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // ... other fields
}

    #[derive(Accounts)]
    pub struct Rebase<'info> {
        #[account(mut)]
        pub token_account: Account<'info, TokenState>,
        // Include other necessary accounts, like an authority account to authorize the rebase
    }
