use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::extension::{Extension, ExtensionType};
pub mod extension;
// Define a struct for your rebasing token account
#[account]
pub struct RebaseConfig {
    pub total_supply: u64,
    pub total_shares: u64,
}

impl Extension for RebaseExtension {
    const TYPE: ExtensionType = ExtensionType::RebaseConfig;
}

// Context struct for the Initialize instruction
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<RebasingTokenAccount>())]
    pub token_account: Account<'info, RebasingTokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Context struct for the Deposit instruction
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
   
}

// Context struct for the Withdraw instruction
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
   
}

// Context struct for the Rebase instruction
#[derive(Accounts)]
pub struct Rebase<'info> {
    #[account(mut)]
    pub token_account: Account<'info, RebasingTokenAccount>,
    // Include other necessary accounts, like an authority account to authorize the rebase
}