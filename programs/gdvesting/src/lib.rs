use anchor_lang::prelude::*;

declare_id!("7w9oX4fSFFW9YK7iWYqBUzEwXJHa3UY3wP4y8HvpaU2s");

#[program]
pub mod gdvesting {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let a = 2;
        msg!("VERSION {:?}", a);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
