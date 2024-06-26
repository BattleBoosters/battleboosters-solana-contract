use super::event::EventData;
use super::program::ProgramData;
use crate::constants::*;
use crate::types::{FightCardResult, FighterColorSide};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateFightCard<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump
    )]
    pub program: Account<'info, ProgramData>,
    #[account(
        mut,
        seeds = [MY_APP_PREFIX, EVENT, event.nonce.to_le_bytes().as_ref()],
        bump
    )]
    pub event: Account<'info, EventData>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), event.fight_card_nonce.to_le_bytes().as_ref()],
    bump,
    space = 8 + 32 + 8 + 1 + (1 + (24 * 4) ) + (1 + (24 * 4) ) + 9 + 2 + 2 + 50 + 8
    )]
    pub fight_card: Account<'info, FightCardData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFightCard<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump
    )]
    pub program: Account<'info, ProgramData>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event.nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Account<'info, EventData>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), fight_card.nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub fight_card: Account<'info, FightCardData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FightCardData {
    /// Public key of the event account this fight card is part of
    pub event_pubkey: Pubkey,
    /// Nonce of the event PDA this fight card is part of
    pub event_nonce_tracker: u64,
    /// Indicates whether this fight is a title fight
    pub title_fight: bool,
    /// Final fight data about the fighter left position
    /// This is None if the fight have not yet finished
    pub fighter_blue: Option<SharedStrength>,
    /// Final fight data about the fighter right position
    /// This is None if the fight have not yet finished
    pub fighter_red: Option<SharedStrength>,
    /// Fight duration in seconds
    pub fight_duration: Option<i64>,
    /// Result of the fight
    pub result: Option<FightCardResult>,
    /// Winner of the fight
    /// This is None in case of a draw when fight is finished
    pub winner: Option<FighterColorSide>,
    /// Nonce of the `fight_card`
    pub nonce: u8,
}
#[account]
pub struct FightCardLinkData {
    /// `fight_card` PDA public key for direct ref
    pub fight_card_pubkey: Pubkey,
    /// Tracker to link the `FightCardLink` PDA to the `FightCard` PDA
    pub fight_card_nonce_tracker: u8,
    /// The `Pubkey` of the booster used
    pub fighter_used: Option<Pubkey>,
    /// Tracker to link the `Fighter` PDA to the `FightCardLink` PDA
    pub fighter_nonce_tracker: Option<u64>,
    /// The `Pubkey` of the fighter link used
    pub fighter_link_used: Option<Pubkey>,
    /// Tracker to link the `FighterLink` PDA to the `FightCardLink` PDA
    pub fighter_link_used_nonce_tracker: Option<u64>,
    /// The `Pubkey` of the booster used
    pub shield_booster_used: Option<Pubkey>,
    /// Tracker to link the `Booster` PDA to the `FightCardLink` PDA
    pub shield_booster_nonce_tracker: Option<u64>,
    /// The `Pubkey` of the booster used
    pub points_booster_used: Option<Pubkey>,
    /// Tracker to link the `Booster` PDA to the `FightCardLink` PDA
    pub points_booster_nonce_tracker: Option<u64>,
    /// The fighter side chosen by the player `Red Gloves` or `Blue Gloves`
    pub fighter_color_side: FighterColorSide,
    /// Prevents the calculation of points for the same fightCard multiple times
    /// If this occurs, it should close and refund the creator of the fighCardLink PDA?
    pub is_consumed: bool,
    /// Prevent accidental multiple initializations of a PDA
    pub is_initialized: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct SharedStrength {
    pub takedowns_attempted: u16,
    pub takedowns_landed: u16,
    pub takedowns_slams: u16,
    pub sig_clinch_head_strikes_attempted: u16,
    pub sig_clinch_head_strikes_landed: u16,
    pub sig_clinch_body_strikes_attempted: u16,
    pub sig_clinch_body_strikes_landed: u16,
    pub sig_clinch_leg_strikes_attempted: u16,
    pub sig_clinch_leg_strikes_landed: u16,
    pub sig_ground_head_strikes_attempted: u16,
    pub sig_ground_head_strikes_landed: u16,
    pub sig_ground_body_strikes_attempted: u16,
    pub sig_ground_body_strikes_landed: u16,
    pub sig_ground_leg_strikes_attempted: u16,
    pub sig_ground_leg_strikes_landed: u16,
    pub striking_strength: StrikingStrength,
    pub grappling_strength: GrapplingStrength,
}
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct StrikingStrength {
    pub knockdowns: u16,
    pub sig_distance_head_strikes_attempted: u16,
    pub sig_distance_head_strikes_landed: u16,
    pub sig_distance_body_strikes_attempted: u16,
    pub sig_distance_body_strikes_landed: u16,
    pub sig_distance_leg_strikes_attempted: u16,
    pub sig_distance_leg_strikes_landed: u16,
}
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct GrapplingStrength {
    pub reversals: u16,
    pub submissions: u16,
    pub seconds_in_control: u16,
    pub advance_to_half_guard: u16,
    pub advance_to_slide: u16,
    pub advance_to_mount: u16,
    pub advance_to_back: u16,
}

/*
   TODO: Delete fight card
*/
