use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Token};

declare_id!("BYzWEaZXS7Zf4SY6dcqnsjySp9qLEmB9C3WvyigxtpYQ");

#[program]
pub mod tossbounty {
    use super::*;

    pub fn create_and_fund_bounty(ctx: Context<CreateAndFundBounty>, description: String, org: String, amount: u64) -> Result<()> {
        let bounty = &mut ctx.accounts.bounty;
        bounty.description = description;
        bounty.org = org;
        bounty.amount = amount;
        bounty.to_token_account = *ctx.accounts.to_token_account.to_account_info().key;
        bounty.status = BountyStatus::Unclaimed;

        token::transfer(ctx.accounts.transfer_context(), amount)?;

        Ok(())
    }
}

#[account]
pub struct Bounty {
    pub description: String,
    pub org: String,
    pub amount: u64,
    pub status: BountyStatus,
    pub to_token_account: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BountyStatus {
    Unclaimed,
    Claimed,
    Canceled,
}

#[derive(Accounts)]
pub struct CreateAndFundBounty<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, seeds = [b"bounty", payer.key.as_ref()], bump, space = 10240)]
    pub bounty: Account<'info, Bounty>,
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> CreateAndFundBounty<'info> {
    pub fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.from_token_account.to_account_info(),
            to: self.to_token_account.to_account_info(),
            authority: self.payer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
