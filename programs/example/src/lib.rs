use anchor_lang::prelude::*;

declare_id!("wyt78UXHtukcbQsJbRPFGf979jxPMvbEij8qAqNsgUx");

#[program]
pub mod example {
    use super::*;

    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.paused = true;

        Ok(())
    }
}

#[account]
pub struct ExampleState {
    pub paused: bool,
}

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, seeds = [b"pause", authority.key.as_ref()], bump, space = 8 + 1)]
    pub state: Account<'info, ExampleState>,
    pub system_program: Program<'info, System>,
}

