use anchor_lang::prelude::*;
use anchor_lang::prelude::Pubkey;
use anchor_spl::token::{self, Transfer, TokenAccount, Token};
use example::{cpi::accounts::Pause, cpi::pause, program::Example};

declare_id!("BYzWEaZXS7Zf4SY6dcqnsjySp9qLEmB9C3WvyigxtpYQ");

const DESCRIPTION_LENGTH: usize = 280 * 4; // 280 chars max.
const ORG_LENGTH: usize = 140 * 4; // 140 chars max.
const AMOUNT_LENGTH: usize = 64;
const PUBLIC_KEY_LENGTH: usize = 32;
const STATUS_LENGTH: usize = 9 * 4; // 9 chars max.
const BUMP_LENGTH: usize = 8;
const DISCRIMINATOR_LENGTH: usize = 8;

#[program]
pub mod tossbounty {
    use super::*;

    pub fn create_bounty_example(ctx: Context<CreateBountyExample>, description: String, org: String, amount: u64, bump: u8) -> Result<()> {
        let bounty = &mut ctx.accounts.bounty;
        bounty.description = description;
        bounty.org = org;
        bounty.amount = amount;
        bounty.funding_account = *ctx.accounts.funding_account.to_account_info().key;
        bounty.status = BountyStatus::Unclaimed;
        bounty.bump = bump;
        bounty.example_program_id = *ctx.accounts.example_program_id.key;

        Ok(())
    }

    pub fn pause_example(ctx: Context<PauseExample>) -> Result<()> {
        let context = CpiContext::new(
            ctx.accounts.example_program_id.to_account_info(),
            Pause {
                state: ctx.accounts.state.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        );

        pause(context)?;

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
    pub example_program_id: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BountyStatus {
    Unclaimed,
    Claimed,
    Canceled,
}

#[derive(Accounts)]
pub struct CreateBountyExample<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, seeds = [b"bounty", authority.key.as_ref()], bump, space = DISCRIMINATOR_LENGTH + DESCRIPTION_LENGTH + ORG_LENGTH + AMOUNT_LENGTH + PUBLIC_KEY_LENGTH + STATUS_LENGTH + BUMP_LENGTH + PUBLIC_KEY_LENGTH)]
    pub bounty: Account<'info, Bounty>,
    #[account(mut)]
    pub funding_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub example_program_id: Program<'info, Example>,
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

#[derive(Accounts)]
pub struct PauseExample<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"pause", authority.key.as_ref()], seeds::program = example_program_id.key(), bump)]
    /// CHECK: manual checks
    pub state: UncheckedAccount<'info>,
    /// CHECK: manual checks
    pub example_program_id: Program<'info, Example>,
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

