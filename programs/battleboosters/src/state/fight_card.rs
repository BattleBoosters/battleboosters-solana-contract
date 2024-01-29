use anchor_lang::prelude::*;
use crate::constants::*;
use super::event::EventData;
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Accounts)]
pub struct FightCard<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub event: Account<'info, EventData>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.fight_card_id_counter.to_le_bytes().as_ref()],
    bump,
    space = 8 + 8 + 8 + 8
    )]
    pub fight_card_account: Account<'info, EventData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FightCardData {
    pub id: u64,
    pub tournament: TournamentType,
    pub title_fight: bool,
    pub fighter_1: SharedStrength,
    pub fighter_2: SharedStrength,
    pub fight_duration: i64,
    pub result: FightCardResult,
    pub winner: Fighter
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

}
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GrapplingStrength {

}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum Fighter {
    Fighter1,
    Fighter2,
    Unknown
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum FightCardResult {
    KoTko,
    Decision,
    Submission,
    Disqualification,
    NoContest,
    Draw,
    Pending
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum TournamentType {
    MainCard,
    Prelims,
    EarlyPrelims
}
