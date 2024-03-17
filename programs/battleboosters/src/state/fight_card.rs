use super::event::EventData;
use super::program::ProgramData;
use crate::constants::*;
use crate::types::FighterColorSide;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct CreateFightCard<'info> {
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
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), event.fight_card_id_counter.to_le_bytes().as_ref()],
    bump,
    space = 8 + 8 + 32 + 8 + 1 + 1 + 4 + 4 + 8 + 1 + 1
    )]
    pub fight_card: Account<'info, FightCardData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_id: u64, fight_card_id: u8)]
pub struct UpdateFightCard<'info> {
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
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), fight_card_id.to_le_bytes().as_ref()],
    bump
    )]
    pub fight_card: Account<'info, FightCardData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FightCardData {
    /*
       TODO: We should probably remove the identifier field?
           Pros: reference on chain to the event off chain for trustability
           Cons: probably no need to create this ref on chain but rather an off chain ref to on chain
    */
    /// Unique identifier for the fight card entry for off chain ref
    pub id: u64,
    /// Public key of the event account this fight card is part of
    pub event_pubkey: Pubkey,
    /// Nonce of the event PDA this fight card is part of
    pub event_nonce_tracker: u64,
    /*
       TODO:  We should probalby move the next field into the Event Directly ?
    */
    /// The type of tournament MainCard, Prelims or Early Prelims
    pub tournament: TournamentType,
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

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum TournamentType {
    MainCard,
    Prelims,
    EarlyPrelims,
}

/*
   TODO: Delete fight card
*/
