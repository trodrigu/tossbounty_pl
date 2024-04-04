use anchor_lang::prelude::*;
use anchor_lang::prelude::Pubkey;
use example::{cpi::accounts::Pause, cpi::pause, program::Example};

declare_id!("9AUryp328sF7BKnv63arhPvrqHNqmwpQEvm7ry6oEnHh");

#[program]
pub mod tossbounty_pauser {
    use super::*;

    pub fn pause_example(ctx: Context<PauseExample>) -> Result<()> {
        let context = CpiContext::new(
            ctx.accounts.program_id.to_account_info(),
            Pause {
                state: ctx.accounts.state.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        );

        pause(context)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PauseExample<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"pause", authority.key.as_ref()], seeds::program = program_id.key(), bump)]
    /// CHECK: manual checks in callee
    pub state: UncheckedAccount<'info>,
    pub program_id: Program<'info, Example>,
    pub system_program: Program<'info, System>,
}

