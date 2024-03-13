use anchor_lang::prelude::*;

declare_id!("2obgbc7tYuJ63HfgHJeXXSf78YHT1Uy6hRptLA7Jfc2V");

#[program]
pub mod mock_program {
    use super::*;

    pub fn simulate_exploit(ctx: Context<SimulateExploit>, data: u64) -> Result<()> {
        // Simulate exploit behavior, e.g., unauthorized state change
        dbg!("Simulating exploit");
        let mut target_account = &mut ctx.accounts.target_account;
        target_account.data = data; // Simulated unauthorized change
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SimulateExploit<'info> {
    #[account(mut)]
    pub target_account: Account<'info, TargetData>,
}

#[account]
pub struct TargetData {
    pub data: u64,
}
