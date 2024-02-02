use super::program::ProgramData;
use crate::constants::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub program: Account<'info, ProgramData>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, EVENT, program.event_counter.to_le_bytes().as_ref()],
    bump,
    space = 8 + 1 + 8 + 8
    )]
    pub event: Account<'info, EventData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct UpdateEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub program: Account<'info, ProgramData>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event_id.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Account<'info, EventData>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct EventData {
    pub fight_card_id_counter: u8,
    pub start_date: i64,
    pub end_date: i64,
}

/*
   TODO: Delete Event
*/
