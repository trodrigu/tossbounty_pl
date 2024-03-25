use anchor_lang::prelude::*;
use anchor_lang::prelude::Pubkey;
use anchor_spl::token::{self, Transfer, TokenAccount, Token};

declare_id!("BYzWEaZXS7Zf4SY6dcqnsjySp9qLEmB9C3WvyigxtpYQ");

#[program]
pub mod tossbounty {
    use super::*;

    pub fn create_bounty(ctx: Context<CreateBounty>, description: String, org: String, amount: u64, bump: u8) -> Result<()> {
        let bounty = &mut ctx.accounts.bounty;
        bounty.description = description;
        bounty.org = org;
        bounty.amount = amount;
        bounty.funding_account = *ctx.accounts.funding_account.to_account_info().key;
        bounty.status = BountyStatus::Unclaimed;
        bounty.bump = bump;

        Ok(())
    }

    pub fn claim_bounty(ctx: Context<ClaimBounty>) -> Result<()> {
        let bounty = &mut ctx.accounts.bounty;
        bounty.status = BountyStatus::Claimed;

        token::transfer(ctx.accounts.transfer_context(), ctx.accounts.bounty.amount)?;

        Ok(())
    }
}

#[account]
pub struct Bounty {
    pub description: String,
    pub org: String,
    pub amount: u64,
    pub status: BountyStatus,
    pub funding_account: Pubkey,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BountyStatus {
    Unclaimed,
    Claimed,
    Canceled,
}

#[derive(Accounts)]
pub struct CreateBounty<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, seeds = [b"bounty", authority.key.as_ref()], bump, space = 10240)]
    pub bounty: Account<'info, Bounty>,
    #[account(mut)]
    pub funding_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimBounty<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub whitehat_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub funding_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"bounty", authority.key.as_ref()], bump = bounty.bump, has_one = funding_account)]
    pub bounty: Account<'info, Bounty>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimBounty<'info> {
    pub fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.funding_account.to_account_info(),
            to: self.whitehat_token_account.to_account_info(),
            authority: self.authority.to_account_info(),
        };

        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

