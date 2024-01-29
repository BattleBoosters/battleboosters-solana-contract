use anchor_lang::prelude::*;
use crate::constants::*;

#[derive(Accounts)]
pub struct GlobalState<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init,
    payer = signer,
    space = 8 + 8 + 32,
    )]
    pub new_account: Account<'info, GlobalData>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct GlobalData {
    pub event_counter: u64,
    pub admin_pubkey: Pubkey,
}
