use crate::errors::ErrorCode;
use crate::state::fight_card::*;
use crate::state::fighter_base::*;
use crate::state::mintable_game_asset::{
    Attribute, MintableGameAssetData, MintableGameAssetLinkData, NftMetadata,
};
use crate::state::rarity::{Stats, TierProbabilities};
use crate::types::*;
use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};

pub fn verify_equality(expected: &Pubkey, actual: &Pubkey) -> Result<()> {
    require!(expected == actual, ErrorCode::Unauthorized);
    Ok(())
}

pub fn verify_equality_mintable_asset(expected: &Pubkey, actual: &Pubkey) -> Result<()> {
    require!(
        expected == actual,
        ErrorCode::GameAssetLinkNotLinkedToAssetPDA
    );
    Ok(())
}

pub fn process_game_asset_for_action(
    mintable_game_asset: Option<&mut Box<Account<MintableGameAssetData>>>,
    mintable_game_asset_link: Option<&mut Box<Account<MintableGameAssetLinkData>>>,
    burn: bool,
) -> Result<()> {
    if let Some(mintable_asset) = mintable_game_asset {
        // Ensure the owner of the mintable asset is the signer
        let mintable_asset_owner = mintable_asset
            .owner
            .ok_or(ErrorCode::MintableAssetHasNoOwner)?;

        require!(
            mintable_asset.is_burned == false,
            ErrorCode::MintableAssetBurned
        );
        require!(
            mintable_asset.is_locked == false,
            ErrorCode::MintableAssetLocked
        );
        require!(
            mintable_asset.is_minted == false,
            ErrorCode::MintableAssetMintedAndUnavailable
        );

        if let Some(mintable_asset_link) = mintable_game_asset_link {
            verify_equality_mintable_asset(
                &mintable_asset_owner,
                &mintable_asset_link.to_account_info().key(),
            )?;
            // TODO: We probably doesn't need to do this check since it is unlikely to happen within
            //      a `mintable_game_asset` with `is_burned`, `is_locked` and `is_minted` is false
            // Double check also the `mintable_asset_lint` is not set to free
            // require!(!mintable_asset_link.is_free, ErrorCode::Unauthorized);

            // Check the PDA `mintable_asset_link` is linked to the PDA `mintable_asset`
            verify_equality_mintable_asset(
                &mintable_asset.to_account_info().key(),
                &mintable_asset_link.mintable_game_asset_pubkey,
            )?;
            if burn {
                // We set the mintabable game asset to burn true
                // Important! Do not close until the point calculation have been executed
                mintable_asset.is_burned = true;
                // Remove the owner
                mintable_asset.owner = None;
                // We free the PDA for re-usability
                mintable_asset_link.is_free = true;
            }
            // In ()both case even if burn is true we lock the asset
            // because it is being used into the fight card
            // Lock the asset
            mintable_asset.is_locked = true
        } else {
            return Err(error!(ErrorCode::MintableAssetLinkRequired));
        }
    }

    Ok(())
}

pub fn process_and_verify_game_asset_type(
    mintable_game_asset: Option<&Box<Account<MintableGameAssetData>>>,
    fight_card_link: &mut Account<FightCardLinkData>,
) -> Result<()> {
    if let Some(mintable_asset) = mintable_game_asset {
        for attr in mintable_asset.metadata.attributes.iter() {
            match attr.trait_type.as_str() {
                "Fighter Type" => {
                    require!(
                        FighterType::from_name(&attr.value).is_some()
                            && fight_card_link.fighter_used.is_none()
                            && fight_card_link.fighter_nonce_tracker.is_none(),
                        ErrorCode::FightCardLinkedToGameAsset
                    );

                    fight_card_link.fighter_used = Some(mintable_asset.to_account_info().key());
                    fight_card_link.fighter_nonce_tracker = Some(mintable_asset.nonce);
                }
                "Booster Type" => match BoosterType::from_name(&attr.value) {
                    Some(BoosterType::Points) => {
                        require!(
                            fight_card_link.points_booster_used.is_none()
                                && fight_card_link.points_booster_nonce_tracker.is_none(),
                            ErrorCode::FightCardLinkedToGameAsset
                        );

                        fight_card_link.points_booster_used =
                            Some(mintable_asset.to_account_info().key());
                        fight_card_link.points_booster_nonce_tracker = Some(mintable_asset.nonce);
                    }
                    Some(BoosterType::Shield) => {
                        require!(
                            fight_card_link.shield_booster_used.is_none()
                                && fight_card_link.shield_booster_nonce_tracker.is_none(),
                            ErrorCode::FightCardLinkedToGameAsset
                        );

                        fight_card_link.shield_booster_used =
                            Some(mintable_asset.to_account_info().key());
                        fight_card_link.shield_booster_nonce_tracker = Some(mintable_asset.nonce);
                    }
                    // Some(BoosterType::Energy) => {
                    //     require!(
                    //         fight_card_link.energy_booster_used.is_none()
                    //             && fight_card_link.energy_booster_nonce_tracker.is_none()
                    //             && game_asset_nonce.is_some(),
                    //         ErrorCode::FightCardLinkedToGameAsset
                    //     );
                    //     fight_card_link.energy_booster_used =
                    //         Some(mintable_asset.to_account_info().key());
                    //     fight_card_link.energy_booster_nonce_tracker =
                    //         Some(game_asset_nonce.unwrap().clone());
                    // }
                    _ => return Err(ErrorCode::BoosterTypeNotFound.into()),
                },
                "Champions Pass Type" => return Err(ErrorCode::Unauthorized.into()),
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn set_fight_card_properties(
    fight_card: &mut FightCardData,
    params: &FightCardData,
    fight_card_nonce: Option<u8>,
) {
    fight_card.event_pubkey = params.event_pubkey;
    fight_card.title_fight = params.title_fight.clone();
    if let Some(nonce) = fight_card_nonce {
        fight_card.nonce = nonce;
    }

    if let Some(result) = params.result.clone() {
        fight_card.result = Some(result);
    } else {
        fight_card.result = None;
    }

    if let Some(winner) = params.winner.clone() {
        fight_card.winner = Some(winner);
    } else {
        fight_card.winner = None;
    }

    if let Some(fight_duration) = params.fight_duration.clone() {
        fight_card.fight_duration = Some(fight_duration);
    } else {
        fight_card.fight_duration = None;
    }

    if let Some(fight_stats_fighter_1) = params.fighter_blue.clone() {
        fight_card.fighter_blue = Some(fight_stats_fighter_1);
    } else {
        fight_card.fighter_blue = None;
    }

    if let Some(fight_stats_fighter_2) = params.fighter_red.clone() {
        fight_card.fighter_red = Some(fight_stats_fighter_2);
    } else {
        fight_card.fighter_red = None;
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
    hasher.update(public_key_bytes); // Ensures each iteration has a unique input
    hasher.update(nonce_byte.to_be_bytes());
    if let Some(x) = extra_byte {
        hasher.update(x.to_be_bytes());
    }
    let random_result = hasher.finalize();
    u64::from_le_bytes(random_result[0..8].try_into().unwrap())
}

pub fn find_rarity(tier_probabilities: TierProbabilities, random_number: u8) -> usize {
    let probabilities = tier_probabilities.get_probability_for_tier();

    let mut cumulative_probs = vec![];
    let mut sum = 0;

    for prob in probabilities {
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
    image: Option<String>,
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

fn collect_fields<'a>(
    fight_metrics: &'a FightMetrics,
    shared_strength: &'a SharedStrength,
) -> [(&'a Metrics, &'a u16); 29] {
    [
        // Shared strength
        (
            &fight_metrics.takedowns_attempted,
            &shared_strength.takedowns_attempted,
        ),
        (
            &fight_metrics.takedowns_landed,
            &shared_strength.takedowns_landed,
        ),
        (
            &fight_metrics.takedowns_slam,
            &shared_strength.takedowns_slams,
        ),
        (
            &fight_metrics.sig_clinch_head_strikes_attempted,
            &shared_strength.sig_clinch_head_strikes_attempted,
        ),
        (
            &fight_metrics.sig_clinch_head_strikes_landed,
            &shared_strength.sig_clinch_head_strikes_landed,
        ),
        (
            &fight_metrics.sig_clinch_body_strikes_attempted,
            &shared_strength.sig_clinch_body_strikes_attempted,
        ),
        (
            &fight_metrics.sig_clinch_body_strikes_landed,
            &shared_strength.sig_clinch_body_strikes_landed,
        ),
        (
            &fight_metrics.sig_clinch_leg_strikes_attempted,
            &shared_strength.sig_clinch_leg_strikes_attempted,
        ),
        (
            &fight_metrics.sig_clinch_leg_strikes_landed,
            &shared_strength.sig_clinch_leg_strikes_landed,
        ),
        (
            &fight_metrics.sig_ground_head_strikes_attempted,
            &shared_strength.sig_ground_head_strikes_attempted,
        ),
        (
            &fight_metrics.sig_ground_head_strikes_landed,
            &shared_strength.sig_ground_head_strikes_landed,
        ),
        (
            &fight_metrics.sig_ground_body_strikes_attempted,
            &shared_strength.sig_ground_body_strikes_attempted,
        ),
        (
            &fight_metrics.sig_ground_body_strikes_landed,
            &shared_strength.sig_ground_body_strikes_landed,
        ),
        (
            &fight_metrics.sig_ground_leg_strikes_attempted,
            &shared_strength.sig_ground_leg_strikes_attempted,
        ),
        (
            &fight_metrics.sig_ground_leg_strikes_landed,
            &shared_strength.sig_ground_leg_strikes_landed,
        ),
        // Striker strength
        (
            &fight_metrics.knock_downs,
            &shared_strength.striking_strength.knockdowns,
        ),
        (
            &fight_metrics.sig_distance_head_strikes_attempted,
            &shared_strength
                .striking_strength
                .sig_distance_head_strikes_attempted,
        ),
        (
            &fight_metrics.sig_distance_head_strikes_landed,
            &shared_strength
                .striking_strength
                .sig_distance_head_strikes_landed,
        ),
        (
            &fight_metrics.sig_distance_body_strikes_attempted,
            &shared_strength
                .striking_strength
                .sig_distance_body_strikes_attempted,
        ),
        (
            &fight_metrics.sig_distance_body_strikes_landed,
            &shared_strength
                .striking_strength
                .sig_distance_body_strikes_landed,
        ),
        (
            &fight_metrics.sig_distance_leg_strikes_attempted,
            &shared_strength
                .striking_strength
                .sig_distance_leg_strikes_attempted,
        ),
        (
            &fight_metrics.sig_distance_leg_strikes_landed,
            &shared_strength
                .striking_strength
                .sig_distance_leg_strikes_landed,
        ),
        // Grappler strength
        (
            &fight_metrics.submissions,
            &shared_strength.grappling_strength.submissions,
        ),
        (
            &fight_metrics.reversals,
            &shared_strength.grappling_strength.reversals,
        ),
        (
            &fight_metrics.seconds_in_control,
            &shared_strength.grappling_strength.seconds_in_control,
        ),
        (
            &fight_metrics.advance_to_half_guard,
            &shared_strength.grappling_strength.advance_to_half_guard,
        ),
        (
            &fight_metrics.advance_to_side,
            &shared_strength.grappling_strength.advance_to_slide,
        ),
        (
            &fight_metrics.advance_to_mount,
            &shared_strength.grappling_strength.advance_to_mount,
        ),
        (
            &fight_metrics.advance_to_back,
            &shared_strength.grappling_strength.advance_to_back,
        ),
    ]
}
fn calculate_shared_strength(
    fight_metrics: u32,
    power_multiplier: f32,
    shared_strength: f32,
) -> u32 {
    fight_metrics
        .checked_mul((power_multiplier * shared_strength).round() as u32)
        .unwrap()
}
pub fn metrics_calculation(
    fighter_chosen: &SharedStrength,
    fighter_opponent: &SharedStrength,
    fight_metrics: &FightMetrics,
    power_multiplier: f32,
) -> (u32, u32) {
    let fighter_chosen_arr = collect_fields(fight_metrics, fighter_chosen);
    let fighter_opponent_arr = collect_fields(fight_metrics, fighter_opponent);

    // Calculate Points
    let points_value = fighter_chosen_arr
        .iter()
        .fold(0_u32, |acc, (metric, &fighter_metric)| {
            acc + calculate_shared_strength(metric.points, power_multiplier, fighter_metric as f32)
        });

    // Calculate Damage
    let damage_value = fighter_opponent_arr
        .iter()
        .fold(0_u32, |acc, (metric, &fighter_metric)| {
            acc + calculate_shared_strength(metric.damage, 1.0, fighter_metric as f32)
        });

    (points_value, damage_value)
}

pub fn asset_metadata_value(asset_metadata: &NftMetadata, trait_type: String) -> u32 {
    let mut asset_multiplier = 0_u32;
    if let Some(attribute) = asset_metadata
        .attributes
        .iter()
        .find(|x| x.trait_type == trait_type)
    {
        asset_multiplier = attribute.value.parse::<u32>().unwrap();
    }
    asset_multiplier
}

pub fn process_game_asset(
    game_asset_link_nonce: u64,
    player_game_asset_link_nonce: &mut u64,
    is_free: bool,
    link_nonce: &mut u64,
) -> Result<()> {
    require!(
        game_asset_link_nonce <= *player_game_asset_link_nonce,
        ErrorCode::WrongPlayerGameAssetLinkNonce
    );

    if game_asset_link_nonce < *player_game_asset_link_nonce {
        require!(is_free, ErrorCode::NotFreePDA);
    } else {
        // Save the nonce for seeds easier re-generation
        *link_nonce = *player_game_asset_link_nonce;
        // increase the player game asset link nonce for the next game asset generation
        *player_game_asset_link_nonce += 1;
    }

    Ok(())
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
