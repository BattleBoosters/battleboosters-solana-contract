use super::program::ProgramData;
use crate::constants::*;
use crate::types::TournamentType;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub program: Box<Account<'info, ProgramData>>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, EVENT, program.event_nonce.to_le_bytes().as_ref()],
    bump,
    space = 8 + 1 + 1 + 8 + 8 + 4 + (30 * 31)
    )]
    pub event: Box<Account<'info, EventData>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_nonce: u64)]
pub struct UpdateEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub program: Box<Account<'info, ProgramData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event_nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Box<Account<'info, EventData>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_nonce: u64)]
pub struct InitializeEventLink<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event_nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Box<Account<'info, EventData>>,
    #[account(
    init,
    payer = creator,
    space = 110,
    seeds = [MY_APP_PREFIX, EVENT, event.key().as_ref(), creator.key().as_ref()],
    bump
    )]
    pub event_link: Box<Account<'info, EventLinkData>>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct EventLinkData {
    /// `Event` PDA public key for direct ref
    pub event_pubkey: Pubkey,
    /// Tracker to link the `EventLink` PDA to the `Event` PDA
    pub event_nonce_tracker: u64,
    /// Ensure a champions pass have been used for `MainCard` access
    /// `champions_pass_asset` PDA public key for direct ref
    pub champions_pass_pubkey: Option<Pubkey>,
    /// Tracker to link the `champions_pass` PDA
    pub champions_pass_nonce_tracker: Option<u64>,
    /// Prevent accidental multiple initializations of a PDA
    pub is_initialized: bool,
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
    /// Rank rewards for prize distribution
    pub rank_rewards: Vec<RankReward>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RankReward {
    pub start_rank: u64,            //  Defines the beginning rank of a reward tier.
    pub end_rank: Option<u64>, // Explicitly indicates the ending rank (inclusive) with the use of `Option` to handle possible open-ended tiers.
    pub prize_amount: u64,     // Currency or token reward
    pub fighter_amount: i16,   // Quantities of fighter in-game assets awarded
    pub booster_amount: i16,   // Quantities of booster in-game assets awarded
    pub champions_pass_amount: i16, // Quantities of champion's pass in-game assets awarded
}

/*
   TODO: Delete Event
*/
