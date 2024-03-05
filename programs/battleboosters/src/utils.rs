use crate::errors::ErrorCode;
use crate::state::fight_card::*;
use crate::types::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{initialize_mint, InitializeMint};

pub fn only_admin(creator: &Pubkey, admin: &Pubkey) -> Result<()> {
    require!(creator == admin, ErrorCode::Unauthorized);
    Ok(())
}
pub fn set_fight_card_properties(fight_card: &mut FightCardData, params: &FightCardData) {
    fight_card.id = params.id.clone();
    fight_card.event_pubkey = params.event_pubkey;
    fight_card.title_fight = params.title_fight.clone();
    fight_card.result = None;
    fight_card.winner = None;
    fight_card.tournament = params.tournament.clone();

    if let Some(fight_duration) = params.fight_duration.clone() {
        fight_card.fight_duration = Some(fight_duration);
    } else {
        fight_card.fight_duration = None
    }

    if let Some(fight_stats_fighter_1) = params.fighter_left.clone() {
        fight_card.fighter_left = Some(fight_stats_fighter_1);
    } else {
        fight_card.fighter_left = None
    }

    if let Some(fight_stats_fighter_2) = params.fighter_right.clone() {
        fight_card.fighter_right = Some(fight_stats_fighter_2);
    } else {
        fight_card.fighter_right = None
    }
}

pub fn xorshift64(seed: u64) -> u64 {
    let mut new_seed = seed;
    new_seed ^= new_seed.clone() << 13;
    new_seed ^= new_seed.clone() >> 7; // Changed for better distribution with u64
    new_seed ^= new_seed.clone() << 17;
    new_seed
}
pub fn find_rarity(rarity: Vec<u8>, random_number: u8) -> usize {
    let mut cumulative_probs = vec![];
    let mut sum = 0;

    for prob in rarity {
        sum += prob; // Sum the probabilities to make them cumulative
        cumulative_probs.push(sum.clone());
    }

    cumulative_probs
        .iter()
        .position(|&prob| random_number <= prob)
        .unwrap_or(cumulative_probs.len() - 1)
}

pub fn check_unique_nft_types(purchase_requests: Option<Vec<PurchaseRequest>>) -> bool {
    if let Some(requests) = purchase_requests {
        let mut booster_found = false;
        let mut fighter_pack_found = false;

        for request in requests {
            match request.nft_type {
                NftType::Booster => {
                    if booster_found {
                        // A Booster type was already found before, so return false.
                        return false;
                    }
                    booster_found = true;
                }
                NftType::FighterPack => {
                    if fighter_pack_found {
                        // A FighterPack type was already found before, so return false.
                        return false;
                    }
                    fighter_pack_found = true;
                }
            }
        }
    }
    // If we get here, it means there are at most one of each type.
    true
}

// pub fn create_game_token_mint(
//     mint: AccountInfo,
//     rent: AccountInfo,
//     token_program: AccountInfo,
//     mint_authority: &Pubkey,
//     freeze_authority: Option<&Pubkey>,
// ) -> Result<()> {
//     let cpi_accounts = InitializeMint { mint, rent };
//     let cpi_program = token_program;
//     let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
//     initialize_mint(
//         cpi_context,
//         0,                // Decimals
//         mint_authority,   // Mint Authority
//         freeze_authority, // Optional Freeze Authority
//     )?;
//     Ok(())
// }
