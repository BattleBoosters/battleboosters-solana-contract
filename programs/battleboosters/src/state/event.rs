use anchor_lang::prelude::*;
use crate::constants::*;
use super::global::GlobalData;

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub global_state : Account<'info, GlobalData>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, EVENT, global_state.event_counter.to_le_bytes().as_ref()],
    bump,
    space = 8 + 8 + 8 + 8
    )]
    pub event_account: Account<'info, EventData>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct EventData {
    pub fight_card_id_counter: u8,
    pub start_date: i64,
    pub end_date: i64,
}
