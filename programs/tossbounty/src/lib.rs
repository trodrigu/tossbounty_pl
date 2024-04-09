use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("F6xSV3U35V8stCdd1zpuVujRVZm9LimfG3YhRT2csUGm");

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

    pub fn create_bounty_example(
        ctx: Context<CreateBountyExample>,
        description: String,
        org: String,
        amount: u64,
        bump: u8,
    ) -> Result<()> {
        let registry: Vec<String> = vec![
            "wyt78UXHtukcbQsJbRPFGf979jxPMvbEij8qAqNsgUx".to_string(),
            "EbLTbDtQoUtqab4mUqquvEJQVuJvewFGVCQjY9mitREt".to_string(),
            "3goACQjYU2pmueD5zxiyXCzs7tXkq22qyG8hmm6MzS2n".to_string(),
            "3eA2wNQhthWn1HGcZyzpFaZwemrcazV15vNHXjLMhm1c".to_string(),
            "GY41EV1W7wiACD77HDq79cs5VMQJpQTXoaoNdiF4HQQ6".to_string(),
        ];

        let bounty = &mut ctx.accounts.bounty;
        bounty.description = description;
        bounty.org = org;
        bounty.amount = amount;
        bounty.funding_account = *ctx.accounts.funding_account.to_account_info().key;
        bounty.status = BountyStatus::Unclaimed;
        bounty.bump = bump;

        // check program id is a part of the registry
        if registry.contains(&ctx.accounts.program_id.key.to_string()) {
            bounty.program_id = *ctx.accounts.program_id.key;
        } else {
            return Err(ErrorCode::InvalidProgramId.into());
        }

        Ok(())
    }

    pub fn claim_bounty_example(ctx: Context<ClaimBountyExample>) -> Result<()> {
        let bounty = &mut ctx.accounts.bounty;
        bounty.status = BountyStatus::Claimed;

        let registry: Vec<String> = vec![
            "wyt78UXHtukcbQsJbRPFGf979jxPMvbEij8qAqNsgUx".to_string(),
            "EbLTbDtQoUtqab4mUqquvEJQVuJvewFGVCQjY9mitREt".to_string(),
            "3goACQjYU2pmueD5zxiyXCzs7tXkq22qyG8hmm6MzS2n".to_string(),
            "3eA2wNQhthWn1HGcZyzpFaZwemrcazV15vNHXjLMhm1c".to_string(),
            "GY41EV1W7wiACD77HDq79cs5VMQJpQTXoaoNdiF4HQQ6".to_string(),
        ];

        // check program id is a part of the registry
        if registry.contains(&ctx.accounts.program_id.key.to_string()) {
            bounty.program_id = *ctx.accounts.program_id.key;
        } else {
            return Err(ErrorCode::InvalidProgramId.into());
        }

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
    pub program_id: Pubkey,
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
    #[account(init, payer = authority, seeds = [b"bounty", authority.key.as_ref(), program_id.key.as_ref(), funding_account.key().as_ref()], bump, space = DISCRIMINATOR_LENGTH + DESCRIPTION_LENGTH + ORG_LENGTH + AMOUNT_LENGTH + PUBLIC_KEY_LENGTH + STATUS_LENGTH + BUMP_LENGTH + PUBLIC_KEY_LENGTH)]
    pub bounty: Account<'info, Bounty>,
    #[account(mut)]
    pub funding_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    /// CHECK: via the program id Registry
    pub program_id: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct ClaimBountyExample<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub whitehat_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub funding_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"bounty", authority.key.as_ref(), program_id.key.as_ref(), funding_account.key().as_ref()], bump = bounty.bump, has_one = funding_account)]
    pub bounty: Account<'info, Bounty>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /// CHECK: via the program id Registry
    pub program_id: UncheckedAccount<'info>,
}

impl<'info> ClaimBountyExample<'info> {
    pub fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.funding_account.to_account_info(),
            to: self.whitehat_token_account.to_account_info(),
            authority: self.authority.to_account_info(),
        };

        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
