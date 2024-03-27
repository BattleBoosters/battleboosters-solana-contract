use super::program::ProgramData;
use crate::constants::*;
use crate::state::fight_card::SharedStrength;
use crate::types::FighterType;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(fighter_type: FighterType)]
pub struct CreateFighter<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub program: Account<'info, ProgramData>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, FIGHTER, &[fighter_type.clone() as u8]],
    bump,
    space = 8 + 2 + (1 + (24 * 4) ) + (1 + (24 * 4) ) + 50
    )]
    pub fighter: Account<'info, FighterData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(fighter_type: FighterType)]
pub struct UpdateFighter<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub program: Account<'info, ProgramData>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHTER, &[fighter_type.clone() as u8]],
    bump,
    )]
    pub fighter: Account<'info, FighterData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FighterData {
    pub fighter_type: FighterType,
    pub shared_strength: SharedStrength,
}
