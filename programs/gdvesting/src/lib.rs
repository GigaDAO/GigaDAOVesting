use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("7w9oX4fSFFW9YK7iWYqBUzEwXJHa3UY3wP4y8HvpaU2s");

// consts
pub const MIN_ACCOUNT_LEN: usize = 9;
const AUTH_PDA_SEED: &[u8] = b"auth_pda_seed";
pub const VESTING_CONTRACT_LEN: usize = 500;
// pub const VESTING_START_TIMESTAMP: u64 = 1685548800; // June 1, 2023

#[program]
pub mod gdvesting {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        investor: Pubkey,
        vesting_rate: u64,
        total_allocated_amount: u64,
        vesting_start_timestamp: u64,
    ) -> Result<()> {
        ctx.accounts.vesting_contract.investor = investor;
        ctx.accounts.vesting_contract.vault = ctx.accounts.gigs_vault.key();
        ctx.accounts.vesting_contract.mint = ctx.accounts.gigs_mint.key();
        ctx.accounts.vesting_contract.vesting_rate = vesting_rate;
        ctx.accounts.vesting_contract.vesting_start_timestamp = vesting_start_timestamp;
        ctx.accounts.vesting_contract.total_allocated_amount = total_allocated_amount;
        ctx.accounts.vesting_contract.claimed_amount = 0;
        Ok(())
    }
    pub fn claim(
        ctx: Context<Claim>,
    ) -> Result<()> {

        let contract = &mut ctx.accounts.vesting_contract;

        // check start date
        let current_timestamp = Clock::get().unwrap().unix_timestamp as u64;
        if current_timestamp < contract.vesting_start_timestamp {
            return err!(ErrorCode::VestingStartDateNotReached);
        }

        // calculate amount vested
        let total_seconds_vested = current_timestamp - contract.vesting_start_timestamp;
        let total_amount_vested = total_seconds_vested * contract.vesting_rate;
        let claimable_amount = total_amount_vested - contract.claimed_amount;

        let (auth_pda, bump_seed) = Pubkey::find_program_address(&[AUTH_PDA_SEED], ctx.program_id);
        let seeds = &[&AUTH_PDA_SEED[..], &[bump_seed]];
        let signer = &[&seeds[..]];

        // check pda addy correct
        if auth_pda != ctx.accounts.auth_pda.key() {
            return Err(ErrorCode::InvalidAuthPda.into());
        }

        // transfer wsol
        let cpi_accounts = Transfer {
            from: ctx.accounts.gigs_vault.to_account_info(),
            to: ctx.accounts.receiver_gigs_ata.to_account_info(),
            authority: ctx.accounts.auth_pda.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, claimable_amount)?;

        // update claimed amount
        contract.claimed_amount += claimable_amount;

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
    pub vesting_start_timestamp: u64, // unix
    pub vesting_rate: u64, // gigs / second
    pub claimed_amount: u64,
    pub total_allocated_amount: u64,
}

#[account]
#[derive(Default)]
pub struct AuthAccount {}

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



