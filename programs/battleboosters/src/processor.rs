use crate::errors::ErrorCode;
use crate::state::rarity::{InitializeRarity, RarityBooster, RarityFighter};
use anchor_lang::prelude::*;

pub fn initialize_rarity(
    ctx: Context<InitializeRarity>,
    fighter: Vec<RarityFighter>,
    energy_booster: Vec<RarityBooster>,
    shield_booster: Vec<RarityBooster>,
    points_booster: Vec<RarityBooster>,
    fighter_probabilities: Vec<u8>,
    booster_probabilities: Vec<u8>,
) -> Result<()> {
    let rarity = &mut ctx.accounts.rarity;
    require!(!rarity.is_initialized, ErrorCode::AlreadyInitialized);

    rarity.fighter = fighter;
    rarity.energy_booster = energy_booster;
    rarity.shield_booster = shield_booster;
    rarity.points_booster = points_booster;
    rarity.fighter_probabilities = fighter_probabilities;
    rarity.booster_probabilities = booster_probabilities;
    rarity.is_initialized = true;

    msg!("Rarity Initialized");

    Ok(())
}
