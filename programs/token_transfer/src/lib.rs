use anchor_lang::prelude::*;

declare_id!("5a4U1aJy23hnXWG4adUJK4Y6t7tvp3XiCTDg7Eoj6ijS");

#[program]
pub mod token_transfer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
