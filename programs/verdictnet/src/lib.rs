use anchor_lang::prelude::*;

declare_id!("22222222222222222222222222222222");

#[program]
pub mod verdictnet {
    use super::*;

    pub fn open_case(ctx: Context<OpenCase>, subject_hash: [u8;32], stake_required: u64) -> Result<()> {
        let case = &mut ctx.accounts.case;
        case.opener = ctx.accounts.opener.key();
        case.subject_hash = subject_hash;
        case.stake_required = stake_required;
        case.status = CaseStatus::Open;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenCase<'info> {
    #[account(init, payer = opener, space = 8 + Case::SIZE)]
    pub case: Account<'info, Case>,
    #[account(mut)]
    pub opener: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Case {
    pub opener: Pubkey,
    pub subject_hash: [u8;32],
    pub stake_required: u64,
    pub status: CaseStatus,
}

impl Case {
    pub const SIZE: usize = 32 + 32 + 8 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum CaseStatus {
    Open,
    Finalized,
}
