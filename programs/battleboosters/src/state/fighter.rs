use super::program::ProgramData;
use crate::constants::*;
use crate::types::FighterType;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(fighter_type: FighterType)]
pub struct CreateFighter<'info> {
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
    seeds = [MY_APP_PREFIX, FIGHTER, &[fighter_type.clone() as u8]],
    bump,
    space = 8 + 2 + (1 + (72 * 4) ) + (1 + (72 * 4) ) + 50
    )]
    pub fighter: Account<'info, FighterData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(fighter_type: FighterType)]
pub struct UpdateFighter<'info> {
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
    seeds = [MY_APP_PREFIX, FIGHTER, &[fighter_type.clone() as u8]],
    bump,
    )]
    pub fighter: Account<'info, FighterData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FighterData {
    pub fighter_type: FighterType,
    pub offensive_metrics: OffensiveMetrics,
    pub defensive_metrics: DefensiveMetrics,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Metrics {
    points: u32,
    energy: u32,
    damage: u32,
}
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct OffensiveMetrics {
    // Shared skills
    pub takedowns_attempted: Metrics,
    pub takedowns_landed: Metrics,
    pub sig_clinch_head_strikes_attempted: Metrics,
    pub sig_clinch_head_strikes_landed: Metrics,
    pub sig_clinch_body_strikes_attempted: Metrics,
    pub sig_clinch_body_strikes_landed: Metrics,
    pub sig_clinch_leg_strikes_attempted: Metrics,
    pub sig_clinch_leg_strikes_landed: Metrics,
    // Striking skills
    pub knockdowns: Metrics,
    pub sig_distance_head_strikes_attempted: Metrics,
    pub sig_distance_head_strikes_landed: Metrics,
    pub sig_distance_body_strikes_attempted: Metrics,
    pub sig_distance_body_strikes_landed: Metrics,
    pub sig_distance_leg_strikes_attempted: Metrics,
    pub sig_distance_leg_strikes_landed: Metrics,
    // Grappling skills
    pub reversals: Metrics,
    pub submissions: Metrics,
    pub seconds_in_control: Metrics,
    pub sig_ground_head_strikes_attempted: Metrics,
    pub sig_ground_head_strikes_landed: Metrics,
    pub sig_ground_body_strikes_attempted: Metrics,
    pub sig_ground_body_strikes_landed: Metrics,
    pub sig_ground_leg_strikes_attempted: Metrics,
    pub sig_ground_leg_strikes_landed: Metrics,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct DefensiveMetrics {
    pub takedowns_dodged: Metrics,
    pub takedowns_received: Metrics,
    pub sig_clinch_head_strikes_dodged: Metrics,
    pub sig_clinch_head_strikes_received: Metrics,
    pub sig_clinch_body_strikes_dodged: Metrics,
    pub sig_clinch_body_strikes_received: Metrics,
    pub sig_clinch_leg_strikes_dodged: Metrics,
    pub sig_clinch_leg_strikes_received: Metrics,

    pub knockdowns_received: Metrics,
    pub sig_distance_head_strikes_dodged: Metrics,
    pub sig_distance_head_strikes_received: Metrics,
    pub sig_distance_body_strikes_dodged: Metrics,
    pub sig_distance_body_strikes_received: Metrics,
    pub sig_distance_leg_strikes_dodged: Metrics,
    pub sig_distance_leg_strikes_received: Metrics,

    pub reversals_received: Metrics,
    pub submissions_received: Metrics,
    pub seconds_in_controls_received: Metrics,
    pub sig_ground_head_strikes_dodged: Metrics,
    pub sig_ground_head_strikes_received: Metrics,
    pub sig_ground_body_strikes_dodged: Metrics,
    pub sig_ground_body_strikes_received: Metrics,
    pub sig_ground_leg_strikes_dodged: Metrics,
    pub sig_ground_leg_strikes_received: Metrics,
}
