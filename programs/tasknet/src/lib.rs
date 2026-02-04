use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod tasknet {
    use super::*;

    pub fn create_task(ctx: Context<CreateTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.creator = ctx.accounts.creator.key();
        task.status = TaskStatus::Created;
        task.claimant = None;
        Ok(())
    }

    pub fn claim_task(ctx: Context<ClaimTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        require!(task.status == TaskStatus::Created, TaskError::AlreadyClaimed);
        task.claimant = Some(ctx.accounts.claimer.key());
        task.status = TaskStatus::Claimed;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTask<'info> {
    #[account(init, payer = creator, space = 8 + Task::SIZE)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimTask<'info> {
    #[account(mut)]
    pub task: Account<'info, Task>,
    pub claimer: Signer<'info>,
}

#[account]
pub struct Task {
    pub creator: Pubkey,
    pub claimant: Option<Pubkey>,
    pub status: TaskStatus,
}

impl Task {
    pub const SIZE: usize = 32 + 1 + 32 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Created,
    Claimed,
}

#[error_code]
pub enum TaskError {
    #[msg("Task already claimed")]
    AlreadyClaimed,
}
