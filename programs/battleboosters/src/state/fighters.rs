use super::event::EventData;
use super::program::ProgramData;
use crate::constants::*;
use anchor_lang::prelude::*;
use crate::state::fight_card::FightCardData;
use crate::types::FighterType;

#[derive(Accounts)]
pub struct InitializeFighters<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub program: Account<'info, ProgramData>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX],
    bump,
    space = 8 + 32 + 8 + 1 + 5 + 5 + 9 + 2 + 2
    )]
    pub fighters: Account<'info, FightersData>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct FightersData {
    pub fighters: Vec<Fighter>
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Fighter {
    pub fighter_type: FighterType,
}