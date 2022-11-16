use anchor_lang::prelude::*;

declare_id!("7w9oX4fSFFW9YK7iWYqBUzEwXJHa3UY3wP4y8HvpaU2s");

#[program]
pub mod gdvesting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("VERSION 2");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
