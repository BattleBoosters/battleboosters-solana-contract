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
    pub fight_card_account: Account<'info, FightCardData>,
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
    pub fight_card_account: Account<'info, FightCardData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FightCardData {
    pub id: u64,
    pub event_pubkey: Pubkey,
    pub tournament: TournamentType,
    pub title_fight: bool,
    pub fight_stats_fighter_1: Option<SharedStrength>,
    pub fight_stats_fighter_2: Option<SharedStrength>,
    pub fight_duration: Option<i64>,
    pub result: Option<FightCardResult>,
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
    example: u8,
}
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct GrapplingStrength {
    example: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum Fighter {
    Fighter1,
    Fighter2,
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
   TODO: Update fight card
*/
