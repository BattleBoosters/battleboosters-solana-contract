use super::event::EventData;
use super::program::ProgramData;
use crate::constants::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateFightCard<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub program: Account<'info, ProgramData>,
    #[account(mut)]
    pub event: Account<'info, EventData>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.fight_card_id_counter.to_le_bytes().as_ref()],
    bump,
    space = 8 + 8 + 32 + 1 + 1 + 4 + 4 + 8 + 1 + 1
    )]
    pub fight_card: Account<'info, FightCardData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(fight_card_id: u8)]
pub struct UpdateFightCard<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub program: Account<'info, ProgramData>,
    #[account(mut)]
    pub event: Account<'info, EventData>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, fight_card_id.to_le_bytes().as_ref()],
    bump
    )]
    pub fight_card: Account<'info, FightCardData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FightCardData {
    /// Unique identifier for the fight card entry
    pub id: u64,
    /// Public key of the event account this fight card is part of
    pub event_pubkey: Pubkey,
    /// The type of tournament MainCard, Prelims or Early Prelims
    pub tournament: TournamentType,
    /// Indicates whether this fight is a title fight
    pub title_fight: bool,
    /// Final fight data about the fighter left position
    /// This is None if the fight have not yet finished
    pub fighter_left: Option<SharedStrength>,
    /// Final fight data about the fighter right position
    /// This is None if the fight have not yet finished
    pub fighter_right: Option<SharedStrength>,
    /// Fight duration in seconds
    pub fight_duration: Option<i64>,
    /// Result of the fight
    pub result: Option<FightCardResult>,
    /// Winner of the fight
    /// This is None in case of a draw when fight is finished
    pub winner: Option<Fighter>,
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
pub enum Fighter {
    FighterLeft,
    FighterRight,
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

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum TournamentType {
    MainCard,
    Prelims,
    EarlyPrelims,
}

/*
   TODO: Delete fight card
*/
