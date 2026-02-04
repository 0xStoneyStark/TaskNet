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
        task.result = None;
        Ok(())
    }

    pub fn claim_task(ctx: Context<ClaimTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        require!(task.status == TaskStatus::Created, TaskError::InvalidState);
        task.claimant = Some(ctx.accounts.claimer.key());
        task.status = TaskStatus::Claimed;
        Ok(())
    }

    pub fn submit_result(ctx: Context<SubmitResult>, result: Vec<u8>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        require!(task.status == TaskStatus::Claimed, TaskError::InvalidState);
        require!(task.claimant == Some(ctx.accounts.claimer.key()), TaskError::NotClaimant);
        task.result = Some(result);
        task.status = TaskStatus::Completed;
        Ok(())
    }

    pub fn settle_task(ctx: Context<SettleTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        require!(task.status == TaskStatus::Completed, TaskError::InvalidState);
        task.status = TaskStatus::Settled;
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

#[derive(Accounts)]
pub struct SubmitResult<'info> {
    #[account(mut)]
    pub task: Account<'info, Task>,
    pub claimer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SettleTask<'info> {
    #[account(mut)]
    pub task: Account<'info, Task>,
}

#[account]
pub struct Task {
    pub creator: Pubkey,
    pub claimant: Option<Pubkey>,
    pub status: TaskStatus,
    pub result: Option<Vec<u8>>,
}

impl Task {
    pub const SIZE: usize = 32 + 1 + 32 + 1 + 4 + 256;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Created,
    Claimed,
    Completed,
    Settled,
}

#[error_code]
pub enum TaskError {
    #[msg("Invalid task state")]
    InvalidState,
    #[msg("Caller is not the task claimant")]
    NotClaimant,
}
