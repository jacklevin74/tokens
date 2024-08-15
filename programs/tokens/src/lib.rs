use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Burn};

declare_id!("4viYp2XpQ7fZoTgrnUHMxH9UyLa4bNYY1chcAykN39Mf");

#[program]
pub mod xspl_token_program {
    use super::*;

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Mint the tokens
        token::mint_to(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.source.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Burn the tokens
        token::burn(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    /// CHECK: This is the token mint account, and its safety is ensured by the SPL Token program
    pub mint: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: This is the recipient's token account, its safety is ensured by the SPL Token program
    pub recipient: AccountInfo<'info>,

    #[account(signer)]
    /// CHECK: This is the mint authority account, its safety is ensured by the SPL Token program
    pub mint_authority: AccountInfo<'info>,

    /// CHECK: This is the SPL Token program account, its safety is ensured by the SPL Token program
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    /// CHECK: This is the token mint account, its safety is ensured by the SPL Token program
    pub mint: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: This is the source token account, its safety is ensured by the SPL Token program
    pub source: AccountInfo<'info>,

    #[account(signer)]
    /// CHECK: This is the burn authority account, its safety is ensured by the SPL Token program
    pub authority: AccountInfo<'info>,

    /// CHECK: This is the SPL Token program account, its safety is ensured by the SPL Token program
    pub token_program: AccountInfo<'info>,
}

