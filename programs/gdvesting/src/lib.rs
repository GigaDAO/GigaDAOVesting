use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("7w9oX4fSFFW9YK7iWYqBUzEwXJHa3UY3wP4y8HvpaU2s");

// consts
pub const MIN_ACCOUNT_LEN: usize = 9;
const AUTH_PDA_SEED: &[u8] = b"auth_pda_seed";
pub const VESTING_START_TIMESTAMP: u64 = 1685548800;
pub const VESTING_CONTRACT_LEN: usize = 100;

#[program]
pub mod gdvesting {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        investor: Pubkey,
        vesting_rate: u64,
        total_allocated_amount: u64,
    ) -> Result<()> {
        ctx.accounts.vesting_contract.investor = investor;
        ctx.accounts.vesting_contract.vault = ctx.accounts.gigs_vault.key();
        ctx.accounts.vesting_contract.mint = ctx.accounts.gigs_mint.key();
        ctx.accounts.vesting_contract.vesting_rate = vesting_rate;
        ctx.accounts.vesting_contract.total_allocated_amount = total_allocated_amount;
        ctx.accounts.vesting_contract.claimed_amount = 0;
        Ok(())
    }
     pub fn claim(
         ctx: Context<Claim>,
         amount: u64,
     ) -> Result<()> {

         let contract = &mut ctx.accounts.vesting_contract;

         // check start date
         let current_timestamp = Clock::get().unwrap().unix_timestamp as u64;
         if current_timestamp < VESTING_START_TIMESTAMP {
             return err!(ErrorCode::VestingStartDateNotReached);
         }

         // calculate amount vested
         let total_seconds_vested = current_timestamp - VESTING_START_TIMESTAMP;
         let total_amount_vested = total_seconds_vested * contract.vesting_rate;
         let claimable_amount = total_amount_vested - contract.claimed_amount;

         if amount > claimable_amount {
             return err!(ErrorCode::AmountMoreThanClaimable);
         }

         // transfer earned tokens
         let signer_handle = &ctx.accounts.signer;
         let tx_handle = ctx.accounts.receiver_gigs_ata.to_account_info();
         let rx_handle = ctx.accounts.gigs_vault.to_account_info();
         let token_program_acct_info = ctx.accounts.token_program.to_account_info();
         transfer_tokens(signer_handle, tx_handle, rx_handle, token_program_acct_info, amount)?;

         // update claimed amount
         contract.claimed_amount += amount;

         Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init_if_needed,
    seeds = [AUTH_PDA_SEED],
    bump,
    payer = signer,
    space = MIN_ACCOUNT_LEN,
    )]
    pub auth_pda: Account<'info, AuthAccount>,
    #[account(
    init,
    payer = signer,
    space = VESTING_CONTRACT_LEN,
    )]
    pub vesting_contract: Account<'info, VestingContract>,
    pub gigs_mint: Account<'info, Mint>,
    #[account(
    init,
    token::mint = gigs_mint,
    token::authority = auth_pda,
    payer = signer,
    )]
    pub gigs_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    seeds = [AUTH_PDA_SEED],
    bump,
    )]
    pub auth_pda: Account<'info, AuthAccount>,
    #[account(
    mut,
    constraint = vesting_contract.investor == signer.key(),
    constraint = vesting_contract.vault == gigs_vault.key(),
    constraint = vesting_contract.mint == gigs_mint.key(),
    )]
    pub vesting_contract: Account<'info, VestingContract>,
    pub gigs_mint: Account<'info, Mint>,
    #[account(
    mut,
    token::mint = gigs_mint,
    )]
    pub gigs_vault: Account<'info, TokenAccount>,
    #[account(
    mut,
    token::mint = gigs_mint,
    constraint = receiver_gigs_ata.owner.key() == signer.key(),
    )]
    pub receiver_gigs_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct VestingContract {
    pub investor: Pubkey,
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub vesting_rate: u64, // gigs / second
    pub claimed_amount: u64,
    pub total_allocated_amount: u64,
}

#[account]
#[derive(Default)]
pub struct AuthAccount {}

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
    #[msg("Vesting Start Date Not Reached.")]
    VestingStartDateNotReached,
    #[msg("Amount More Than Claimable.")]
    AmountMoreThanClaimable,
}



