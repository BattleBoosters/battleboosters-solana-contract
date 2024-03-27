use super::program::ProgramData;
use crate::constants::*;
use crate::state::rank::RankData;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(event_pubkey: Pubkey, rank_nonce: u64)]
pub struct CollectRewards<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump
    )]
    pub program: Box<Account<'info, ProgramData>>,
    /// CHECK: This is a PDA used as the bank
    #[account(mut, seeds = [MY_APP_PREFIX, BANK], bump = program.bank_bump)]
    pub bank: AccountInfo<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, RANK, event_pubkey.as_ref(), rank_nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub rank: Account<'info, RankData>,
    pub system_program: Program<'info, System>,
}
