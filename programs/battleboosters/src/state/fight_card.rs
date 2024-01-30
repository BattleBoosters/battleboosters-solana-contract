use super::event::EventData;
use super::global_state::GlobalStateData;
use crate::constants::*;
use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Accounts)]
pub struct CreateFightCard<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub global_state: Account<'info, GlobalStateData>,
    #[account(mut)]
    pub event: Account<'info, EventData>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.fight_card_id_counter.to_le_bytes().as_ref()],
    bump,
    space = 8 + 8 + 8 + 8
    )]
    pub fight_card_account: Account<'info, FightCardData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FightCardData {
    pub id: u64,
    pub event_pubkey: Pubkey,
    pub tournament: Option<TournamentType>,
    pub title_fight: bool,
    pub fight_stats_fighter_1: Option<SharedStrength>,
    pub fight_stats_fighter_2: Option<SharedStrength>,
    pub fight_duration: Option<i64>,
    pub result: Option<FightCardResult>,
    pub winner: Option<Fighter>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct SharedStrength {
    pub takedowns_attempted: u8,
    pub takedowns_landed: u8,
    pub striking_strength: StrikingStrength,
    pub grappling_strength: GrapplingStrength,
}
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct StrikingStrength {
    example: u8,
}
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GrapplingStrength {
    example: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum Fighter {
    Fighter1,
    Fighter2,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum FightCardResult {
    KoTko,
    Decision,
    Submission,
    Disqualification,
    NoContest,
    Draw,
    InternalCancellation,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum TournamentType {
    MainCard,
    Prelims,
    EarlyPrelims,
}

/*
   TODO: Update fight card
*/
