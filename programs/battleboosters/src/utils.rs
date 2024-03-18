use crate::errors::ErrorCode;
use crate::state::fight_card::*;
use crate::state::player::{
    Attribute, EventLinkData, FightCardLinkData, MintableGameAssetData, NftMetadata,
    PlayerGameAssetLinkData,
};
use crate::state::rarity::Stats;
use crate::types::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{initialize_mint, InitializeMint};
use sha2::{Digest, Sha256};
use std::ops::Deref;

pub fn verify_equality(expected: &Pubkey, actual: &Pubkey) -> Result<()> {
    require!(expected == actual, ErrorCode::Unauthorized);
    Ok(())
}
/*
   TODO: Improve error msg's
*/
pub fn process_game_asset_for_action(
    mintable_game_asset: Option<&mut Box<Account<MintableGameAssetData>>>,
    mintable_game_asset_link: Option<&mut Box<Account<PlayerGameAssetLinkData>>>,
    signer: &Pubkey,
    burn: bool,
) -> Result<()> {
    if let Some(mintable_asset) = mintable_game_asset {
        // Ensure the owner of the mintable asset is the signer
        let mintable_asset_owner = mintable_asset.owner.ok_or(ErrorCode::Unauthorized)?;
        verify_equality(&mintable_asset_owner, &signer)?;

        require!(mintable_asset.is_burned == false, ErrorCode::Unauthorized);
        require!(mintable_asset.is_locked == false, ErrorCode::Unauthorized);
        require!(mintable_asset.is_minted == false, ErrorCode::Unauthorized);

        if let Some(mintable_asset_link) = mintable_game_asset_link {
            // TODO: We probably doesn't need to do this check since it is unlikely to happen within
            //      a `mintable_game_asset` with `is_burned`, `is_locked` and `is_minted` is false
            // Double check also the `mintable_asset_lint` is not set to free
            // require!(!mintable_asset_link.is_free, ErrorCode::Unauthorized);

            // Check the PDA `mintable_asset_link` is linked to the PDA `mintable_asset`
            verify_equality(
                &mintable_asset.to_account_info().key(),
                &mintable_asset_link.mintable_game_asset_pubkey,
            )?;
            if burn {
                // We set the mintabable game asset to burn true
                // TODO: Check if we can close the account to send back the rent to the creator.
                mintable_asset.is_burned = true;
                // We free the PDA for re-usability
                mintable_asset_link.is_free = true;
            }
            // In both case even if burn is true we lock the asset
            // because it is being used into the fight card
            // Lock the asset
            mintable_asset.is_locked = true
        } else {
            return Err(error!(ErrorCode::Unauthorized));
        }
    }

    Ok(())
}

pub fn process_and_verify_game_asset_type(
    mintable_game_asset: Option<&Box<Account<MintableGameAssetData>>>,
    fight_card_link: &mut Account<FightCardLinkData>,
    event_link: &mut Account<EventLinkData>,
    require_tournament_type: Option<&TournamentType>,
    game_asset_nonce: Option<u64>,
) -> Result<()> {
    if let Some(mintable_asset) = mintable_game_asset {
        for attr in mintable_asset.metadata.attributes.iter() {
            match attr.trait_type.as_str() {
                "Fighter Type" => {
                    require!(
                        FighterType::from_name(&attr.value).is_some()
                            && fight_card_link.fighter_used.is_none()
                            && fight_card_link.fighter_nonce_tracker.is_none()
                            && game_asset_nonce.is_some(),
                        ErrorCode::Unauthorized
                    );

                    fight_card_link.fighter_used = Some(mintable_asset.to_account_info().key());
                    fight_card_link.fighter_nonce_tracker = Some(game_asset_nonce.unwrap().clone());
                }
                "Booster Type" => match BoosterType::from_name(&attr.value) {
                    Some(BoosterType::Points) => {
                        require!(
                            fight_card_link.points_booster_used.is_none()
                                && fight_card_link.points_booster_nonce_tracker.is_none()
                                && game_asset_nonce.is_some(),
                            ErrorCode::Unauthorized
                        );

                        fight_card_link.points_booster_used =
                            Some(mintable_asset.to_account_info().key());
                        fight_card_link.points_booster_nonce_tracker =
                            Some(game_asset_nonce.unwrap().clone());
                    }
                    Some(BoosterType::Shield) => {
                        require!(
                            fight_card_link.shield_booster_used.is_none()
                                && fight_card_link.shield_booster_nonce_tracker.is_none()
                                && game_asset_nonce.is_some(),
                            ErrorCode::Unauthorized
                        );

                        fight_card_link.shield_booster_used =
                            Some(mintable_asset.to_account_info().key());
                        fight_card_link.shield_booster_nonce_tracker =
                            Some(game_asset_nonce.unwrap().clone());
                    }
                    Some(BoosterType::Energy) => {
                        require!(
                            fight_card_link.energy_booster_used.is_none()
                                && fight_card_link.energy_booster_nonce_tracker.is_none()
                                && game_asset_nonce.is_some(),
                            ErrorCode::Unauthorized
                        );
                        fight_card_link.energy_booster_used =
                            Some(mintable_asset.to_account_info().key());
                        fight_card_link.energy_booster_nonce_tracker =
                            Some(game_asset_nonce.unwrap().clone());
                    }
                    _ => return Err(ErrorCode::Unauthorized.into()),
                },
                "Champions Pass Type" => match require_tournament_type {
                    Some(TournamentType::MainCard) => {
                        require!(
                            event_link.champions_pass_pubkey.is_none()
                                && event_link.champions_pass_nonce_tracker.is_none()
                                && game_asset_nonce.is_some(),
                            ErrorCode::Unauthorized
                        );

                        event_link.champions_pass_pubkey =
                            Some(mintable_asset.to_account_info().key());
                        event_link.champions_pass_nonce_tracker =
                            Some(game_asset_nonce.unwrap().clone())
                    }
                    _ => return Err(ErrorCode::Unauthorized.into()),
                },
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn set_fight_card_properties(fight_card: &mut FightCardData, params: &FightCardData) {
    fight_card.event_pubkey = params.event_pubkey;
    fight_card.event_nonce_tracker = params.event_nonce_tracker.clone();
    fight_card.title_fight = params.title_fight.clone();
    fight_card.result = None;
    fight_card.winner = None;

    if let Some(fight_duration) = params.fight_duration.clone() {
        fight_card.fight_duration = Some(fight_duration);
    } else {
        fight_card.fight_duration = None
    }

    if let Some(fight_stats_fighter_1) = params.fighter_blue.clone() {
        fight_card.fighter_blue = Some(fight_stats_fighter_1);
    } else {
        fight_card.fighter_blue = None
    }

    if let Some(fight_stats_fighter_2) = params.fighter_red.clone() {
        fight_card.fighter_red = Some(fight_stats_fighter_2);
    } else {
        fight_card.fighter_red = None
    }
}

pub fn create_rng_seed(
    randomness: &[u8],
    public_key_bytes: &[u8],
    nonce_byte: &u8,
    extra_byte: Option<u8>,
) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(&randomness);
    hasher.update(public_key_bytes.clone()); // Ensures each iteration has a unique input
    hasher.update(nonce_byte.to_be_bytes());
    if let Some(x) = extra_byte {
        hasher.update(x.to_be_bytes());
    }
    let random_result = hasher.finalize();
    u64::from_le_bytes(random_result[0..8].try_into().unwrap())
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

pub fn find_scaled_rarity(value: &Stats, rng_seed: u64) -> u32 {
    let range = (&value.max - &value.min + 1) as u64;
    let scaled_random_number = ((rng_seed % range) + value.clone().min as u64) as u32;
    scaled_random_number
}

pub fn create_nft_metadata(
    name: String,
    description: String,
    image: String,
    animation_url: Option<String>,
    external_url: Option<String>,
    attributes: Vec<Attribute>,
) -> NftMetadata {
    NftMetadata {
        name,
        description,
        image,
        animation_url,
        external_url,
        attributes,
    }
}

// pub fn check_unique_nft_types(purchase_requests: Option<Vec<PurchaseRequest>>) -> bool {
//     if let Some(requests) = purchase_requests {
//         let mut booster_found = false;
//         let mut fighter_pack_found = false;
//
//         for request in requests {
//             match request.nft_type {
//                 NftType::Booster => {
//                     if booster_found {
//                         // A Booster type was already found before, so return false.
//                         return false;
//                     }
//                     booster_found = true;
//                 }
//                 NftType::FighterPack => {
//                     if fighter_pack_found {
//                         // A FighterPack type was already found before, so return false.
//                         return false;
//                     }
//                     fighter_pack_found = true;
//                 }
//             }
//         }
//     }
//     // If we get here, it means there are at most one of each type.
//     true
// }

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
