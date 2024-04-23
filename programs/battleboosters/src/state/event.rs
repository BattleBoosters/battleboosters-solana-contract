use super::program::ProgramData;
use crate::constants::*;
use crate::state::mintable_game_asset::{MintableGameAssetData, MintableGameAssetLinkData};
use crate::state::rank::RankData;
use crate::types::TournamentType;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump
    )]
    pub program: Box<Account<'info, ProgramData>>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, EVENT, program.event_nonce.to_le_bytes().as_ref()],
    bump,
    space = 8 + 1 + 1 + 8 + 8 + 4 + (30 * 31) + 8 + 10
    )]
    pub event: Box<Account<'info, EventData>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump
    )]
    pub program: Box<Account<'info, ProgramData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event.nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Box<Account<'info, EventData>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeEventLink<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event.nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Box<Account<'info, EventData>>,
    #[account(
    init,
    payer = creator,
    space = 120,
    seeds = [MY_APP_PREFIX, EVENT, event.key().as_ref(), creator.key().as_ref()],
    bump
    )]
    pub event_link: Box<Account<'info, EventLinkData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, champions_pass_asset.nonce.to_le_bytes().as_ref()],
    // constraint = points_booster_asset.as_ref().is_burned == true,
    // close = signer,
    bump
    )]
    pub champions_pass_asset: Option<Box<Account<'info, MintableGameAssetData>>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, champions_pass_link.nonce.to_le_bytes().as_ref(), creator.key().as_ref()],
    bump
    )]
    pub champions_pass_link: Option<Box<Account<'info, MintableGameAssetLinkData>>>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, RANK, event.key().as_ref(), event.rank_nonce.to_le_bytes().as_ref()],
    bump,
    space = 8 + 33 + 9 + 50 + 8 + 8
    )]
    pub rank: Box<Account<'info, RankData>>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct EventLinkData {
    /// `Event` PDA public key for direct ref
    pub event_pubkey: Pubkey,
    /// Tracker to link the `EventLink` PDA to the `Event` PDA
    pub event_nonce_tracker: u64,
    /// User rank nonce to recreate the pda
    pub rank_nonce: u64,
    /// Ensure a champions pass have been used for `MainCard` access
    /// `champions_pass_asset` PDA public key for direct ref
    pub champions_pass_pubkey: Option<Pubkey>,
    /// Tracker to link the `champions_pass` PDA
    pub champions_pass_nonce_tracker: Option<u64>,
    /// Prevents the player to claim multiple time the rewards
    /// If this occurs, it should close and refund the creator of the EventLink PDA?
    pub is_consumed: bool,
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
    /// Represent the current amount of player
    pub rank_nonce: u64,
    /// Nonce of the `event`
    pub nonce: u64,
}

/*
   TODO:
       Consider adding predefined tiers for rewards for simplicity and user predicable
       rewards for fighters, boosters and champions pass
*/
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RankReward {
    pub start_rank: u64,            //  Defines the beginning rank of a reward tier.
    pub end_rank: Option<u64>, // Explicitly indicates the ending rank (inclusive) with the use of `Option` to handle possible open-ended tiers.
    pub prize_amount: f64,     // Currency or token reward
    pub fighter_amount: i16,   // Quantities of fighter in-game assets awarded
    pub booster_amount: i16,   // Quantities of booster in-game assets awarded
    pub champions_pass_amount: i16, // Quantities of champion's pass in-game assets awarded
}

/*
   TODO: Delete Event
*/
