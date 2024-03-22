use super::event::EventData;
use super::program::ProgramData;
use crate::constants::*;
use crate::types::FighterColorSide;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(event_nonce: u64)]
pub struct CreateFightCard<'info> {
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
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), event.fight_card_nonce.to_le_bytes().as_ref()],
    bump,
    space = 8 + 32 + 8 + 1 + 5 + 5 + 9 + 2 + 2
    )]
    pub fight_card: Account<'info, FightCardData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_nonce: u64, fight_card_nonce: u8)]
pub struct UpdateFightCard<'info> {
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
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), fight_card_nonce.to_le_bytes().as_ref()],
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
    /// The `Pubkey` of the booster used
    pub energy_booster_used: Option<Pubkey>,
    /// Tracker to link the `Booster` PDA to the `FightCardLink` PDA
    pub energy_booster_nonce_tracker: Option<u64>,
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
    /// If this occurs, it should close and refund the creator of the fighCardLink PDA
    pub is_consumed: bool,
    /// Prevent accidental multiple initializations of a PDA
    pub is_initialized: bool,
}


#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct SharedStrength {
    pub takedowns_attempted: u8,
    pub takedowns_landed: u8,
    pub striking_strength: StrikingStrength,
    pub grappling_strength: GrapplingStrength,
}
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct StrikingStrength {
    /*
        TODO:
            Finish Striking Strength implementation
            Do not forget to update the space in account
    */
    example: u8,
}
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct GrapplingStrength {
    /*
        TODO:
            Finish Grappling Strength implementation
            Do not forget to update the space in account
    */
    example: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum FightCardResult {
    KoTko,
    Decision,
    Submission,
    Disqualification,
    NoContest,
    Draw,
    InternalCancellation,
}

/*
   TODO: Delete fight card
*/
