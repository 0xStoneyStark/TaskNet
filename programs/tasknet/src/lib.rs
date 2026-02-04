use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod tasknet {
    use super::*;

    pub fn create_task(ctx: Context<CreateTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.creator = ctx.accounts.creator.key();
        task.status = TaskStatus::Created;
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

#[account]
pub struct Task {
    pub creator: Pubkey,
    pub status: TaskStatus,
}

impl Task {
    pub const SIZE: usize = 32 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Created,
}
