use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("7w9oX4fSFFW9YK7iWYqBUzEwXJHa3UY3wP4y8HvpaU2s");

// consts
pub const VESTING_START_TIMESTAMP: u64 = 1685548800;

#[program]
pub mod gdvesting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
     pub fn claim(ctx: Context<Claim>) -> Result<()> {
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Claim {}

#[account]
#[derive(Default)]
pub struct VestingContract {
    pub investor: Pubkey,
    pub vault: Pubkey,
    pub vesting_rate: u64, // gigs / second
    pub claimed_amount: u64,
    pub total_allocated_amount: u64,
}

// utils
pub fn transfer_tokens<'a>(
    signer: &Signer<'a>,
    tx_acct_info: AccountInfo<'a>,
    rx_acct_info: AccountInfo<'a>,
    token_program_info: AccountInfo<'a>,
    amount: u64
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: tx_acct_info,
        to: rx_acct_info,
        authority: signer.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(token_program_info, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;
    Ok(())
}

// custom errors
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient Proposal Amount.")]
    InsufficientAmount,
    #[msg("Invalid Authorizer PDA.")]
    InvalidAuthPda,
    #[msg("Proposal Not Active.")]
    ProposalNotActive,
    #[msg("Invalid Proposal ID.")]
    InvalidProposalId,
}



