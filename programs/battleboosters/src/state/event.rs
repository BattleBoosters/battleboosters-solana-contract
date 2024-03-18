use super::program::ProgramData;
use crate::constants::*;
use crate::types::TournamentType;
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
    seeds = [MY_APP_PREFIX, EVENT, program.event_nonce.to_le_bytes().as_ref()],
    bump,
    space = 8 + 1 + 1 + 8 + 8
    )]
    pub event: Account<'info, EventData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_nonce: u64)]
pub struct UpdateEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub program: Account<'info, ProgramData>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event_nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Account<'info, EventData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EventData {
    /// Represent the current amount of created fight card
    /// On average, a UFC event typically features around 12 to 15 fights
    /// We set it as `u8` because there will be never more than `255` per events in an MMA fight week
    pub fight_card_nonce: u8,
    /// The type of tournament MainCard, Prelims or Early Prelims
    pub tournament_type: TournamentType,
    /// Start date in seconds
    pub start_date: i64,
    /// End date in seconds
    pub end_date: i64,
}

/*
   TODO: Delete Event
*/
