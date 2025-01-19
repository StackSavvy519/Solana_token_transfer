use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};

declare_id!("1skh4xsCzeWV73h49vuWkqPtDZE4oa7JFMLXiwg16yM");

#[program]
pub mod token_transfer {
    use super::*;

    pub fn transfer_tokens(ctx: Context<TransferAccounts>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferAccounts<'info> {
    /// CHECK: safe
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: safe
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>
}