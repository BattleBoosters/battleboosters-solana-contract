use super::collector_pack::CollectorPack;
use super::program::ProgramData;
use crate::constants::*;
use crate::state::fight_card::FightCardData;
use anchor_lang::prelude::*;

use crate::state::event::EventData;
use crate::state::rarity::RarityData;
use crate::types::FighterColorSide;
use anchor_lang::solana_program::sysvar;
use solana_randomness_service::SimpleRandomnessV1Account;
use solana_randomness_service::{
    program::SolanaRandomnessService, ID as SolanaRandomnessServiceID,
};
use switchboard_solana::prelude::*;

// Struct for initializing player
#[derive(Accounts)]
#[instruction(player_pubkey: Pubkey)]
pub struct InitializePlayer<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, PLAYER, player_pubkey.as_ref()],
    bump,
    space = 8 + 8 + 8 + 1
    )]
    pub player_account: Account<'info, PlayerData>,
    pub system_program: Program<'info, System>,
}

// // Struct for managing player inventory
// #[derive(Accounts)]
// pub struct PlayerInventory<'info> {
//     #[account(mut)]
//     pub signer: Signer<'info>,
//     #[account(mut,
//     seeds = [MY_APP_PREFIX, INVENTORY, signer.key().as_ref()],
//     bump)]
//     pub inventory: Account<'info, InventoryData>,
//     pub system_program: Program<'info, System>,
// }

#[derive(Accounts)]
#[instruction(player_game_asset_link_nonce: u64)]
pub struct GenerateNftPreMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [MY_APP_PREFIX, PROGRAM_STATE], bump)]
    pub program: Account<'info, ProgramData>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PLAYER, signer.key().as_ref()],
    bump,
    )]
    pub player_account: Box<Account<'info, PlayerData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, COLLECTOR, signer.key().as_ref(), player_account.order_nonce.to_le_bytes().as_ref()],
    bump,
    )]
    pub collector_pack: Box<Account<'info, CollectorPack>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, RARITY],
    bump,
    )]
    pub rarity: Option<Box<Account<'info, RarityData>>>,
    #[account(
    init,
    payer = signer,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, program.mintable_game_asset_nonce.to_le_bytes().as_ref()],
    space = 8 + 1 + 1 + 32 + (4 + 20) + (4 + 100) + (4 + 100) + (4 + 100) + (4 + 100) + (4 + 480),
    bump
    )]
    pub mintable_game_asset: Box<Account<'info, MintableGameAssetData>>,

    #[account(
    init_if_needed,
    payer = signer,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, player_game_asset_link_nonce.to_le_bytes().as_ref(), signer.key().as_ref()],
    space = 8 + 32 + 8 + 1,
    bump,
    )]
    pub player_game_asset_link: Box<Account<'info, PlayerGameAssetLinkData>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    event_id: u64,
    fight_card_id: u8,
    fighter_asset_id: u64,
    energy_booster_asset_id: Option<u64>,
    shield_booster_asset_id: Option<u64>,
    points_booster_asset_id: Option<u64>,
    champions_pass_asset_id: Option<u64>,
    fighter_link_id: u64,
    energy_booster_link_id: Option<u64>,
    shield_booster_link_id: Option<u64>,
    points_booster_link_id: Option<u64>,
    champions_pass_link_id: Option<u64>,
)]
pub struct JoinFightCard<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump,
    )]
    pub program: Box<Account<'info, ProgramData>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event_id.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Box<Account<'info, EventData>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fighter_asset_id.to_le_bytes().as_ref()],
    bump
    )]
    pub fighter_asset: Box<Account<'info, MintableGameAssetData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, energy_booster_asset_id.unwrap().to_le_bytes().as_ref()],
    // constraint = energy_booster_asset.as_ref().is_burned == true,
    // close = signer,
    bump
    )]
    pub energy_booster_asset: Option<Box<Account<'info, MintableGameAssetData>>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, shield_booster_asset_id.unwrap().to_le_bytes().as_ref()],
    // constraint = shield_booster_asset.as_ref().is_burned == true,
    // close = signer,
    bump
    )]
    pub shield_booster_asset: Option<Box<Account<'info, MintableGameAssetData>>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, points_booster_asset_id.unwrap().to_le_bytes().as_ref()],
    // constraint = points_booster_asset.as_ref().is_burned == true,
    // close = signer,
    bump
    )]
    pub points_booster_asset: Option<Box<Account<'info, MintableGameAssetData>>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, champions_pass_asset_id.unwrap().to_le_bytes().as_ref()],
    // constraint = points_booster_asset.as_ref().is_burned == true,
    // close = signer,
    bump
    )]
    pub champions_pass_asset: Option<Box<Account<'info, MintableGameAssetData>>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fighter_link_id.to_le_bytes().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub fighter_link: Box<Account<'info, PlayerGameAssetLinkData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, energy_booster_link_id.unwrap().to_le_bytes().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub energy_booster_link: Option<Box<Account<'info, PlayerGameAssetLinkData>>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, shield_booster_link_id.unwrap().to_le_bytes().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub shield_booster_link: Option<Box<Account<'info, PlayerGameAssetLinkData>>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, points_booster_link_id.unwrap().to_le_bytes().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub points_booster_link: Option<Box<Account<'info, PlayerGameAssetLinkData>>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, champions_pass_link_id.unwrap().to_le_bytes().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub champions_pass_link: Option<Box<Account<'info, PlayerGameAssetLinkData>>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), fight_card_id.to_le_bytes().as_ref()],
    bump
    )]
    pub fight_card: Box<Account<'info, FightCardData>>,

    #[account(
    init,
    payer = signer,
    space = 8 + 32 + 32 + 1 + 33 + 9 + 33 + 9 + 33 + 9 + 33 + 9 + 2 + 1 + 1,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), fight_card_id.to_le_bytes().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub fight_card_link: Box<Account<'info, FightCardLinkData>>,

    #[account(
    init_if_needed,
    payer = signer,
    space = 8 + 32 + 32 + 1 + 33 + 9 + 1,
    seeds = [MY_APP_PREFIX, EVENT, event.key().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub event_link: Account<'info, EventLinkData>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlayerData {
    /// Represent the nonce of the current amount orders the player have created
    pub order_nonce: u64,
    /// Represent the nonce of the current player game asset link the player have created
    pub player_game_asset_link_nonce: u64,
    /// Prevent accidental multiple initializations of a PDA
    pub is_initialized: bool,
}

#[account]
pub struct EventLinkData {
    /// Signer of the tx
    pub creator: Pubkey,
    /// `Event` PDA public key for direct ref
    pub event_pubkey: Pubkey,
    /// Tracker to link the `EventLink` PDA to the `Event` PDA
    pub event_nonce_tracker: u64,
    /// Ensure a champions pass have been used for `MainCard` access
    /// `champions_pass_asset` PDA public key for direct ref
    pub champions_pass_pubkey: Option<Pubkey>,
    /// Tracker to link the `champions_pass` PDA
    pub champions_pass_nonce_tracker: Option<u64>,
    /// Prevent accidental multiple initializations of a PDA
    pub is_initialized: bool,
    /*
       TODO: Probably store the Pubkey of the metadata Champion's pass
             + the nonce tracker ?
    */
}
/*

   TODO: Store the PDA used to get back the Metadata when resolving the event

   TODO: UPDATE THE SPAAAAAAAAAAAAACEEEEEEEEEEEE!!!

*/

#[account]
pub struct FightCardLinkData {
    /// Signer of the tx
    pub creator: Pubkey,
    /// `fight_card` PDA public key for direct ref
    pub fight_card_pubkey: Pubkey,
    /// Tracker to link the `FightCardLink` PDA to the `FightCard` PDA
    pub fight_card_nonce_tracker: u8,
    /// The `Pubkey` of the booster used
    pub fighter_used: Option<Pubkey>,
    /// Tracker to link the `Fighter` PDA to the `FightCardLink` PDA
    pub fighter_nonce_tracker: Option<u64>,
    /// The `Pubkey` of the booster used
    pub energy_booster_used: Option<Pubkey>,
    /// Tracker to link the `Booster` PDA to the `FightCardLink` PDA
    pub energy_booster_nonce_tracker: Option<u64>,
    /// The `Pubkey` of the booster used
    pub shield_booster_used: Option<Pubkey>,
    /// Tracker to link the `Booster` PDA to the `FightCardLink` PDA
    pub shield_booster_nonce_tracker: Option<u64>,
    /// The `Pubkey` of the booster used
    pub points_booster_used: Option<Pubkey>,
    /// Tracker to link the `Booster` PDA to the `FightCardLink` PDA
    pub points_booster_nonce_tracker: Option<u64>,
    /// The fighter side chosen by the player `Red Gloves` or `Blue Gloves`
    pub fighter_color_side: FighterColorSide,
    /// Prevents the calculation of points for the same fightCard multiple times
    /// If this occurs, it should close and refund the creator of the fighCardLink PDA
    pub is_consumed: bool,
    /// Prevent accidental multiple initializations of a PDA
    pub is_initialized: bool,
}

#[account]
pub struct PlayerGameAssetLinkData {
    /// `Pubkey` of the mintable_game_asset
    pub mintable_game_asset_pubkey: Pubkey,
    /// this is the link to the address of the pda
    pub mintable_game_asset_nonce_tracker: u64,
    /// Checks if a PDA is eligible to update its `mintable_game_asset_nonce`.
    /// The PDA becomes eligible upon minting and withdrawing a `mintable_game_asset`,
    /// which break the link with the last `mintable_game_asset_nonce`.
    pub is_free: bool,
}

#[account]
pub struct MintableGameAssetData {
    /// is Locked will mean the PDA is in use and cannot be minted or re used
    pub is_locked: bool,
    /// is Burned will mean the PDA have been used and cannot be minted or re used
    pub is_burned: bool,
    /// is Minted mean the PDA have been minted
    pub is_minted: bool,
    /// owner of the PDA can use it in-game,
    /// on mint the owner is set to None which mean it is not available in the game until re-deposited
    pub owner: Option<Pubkey>,
    /// The metadata on-chain, which allow dynamic use on our game
    pub metadata: NftMetadata,
}

/// Metatada Standards copy on-chain
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub animation_url: Option<String>,
    pub external_url: Option<String>,
    pub attributes: Vec<Attribute>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

// #[derive(Accounts)]
// pub struct CreateEvent<'info> {
//     #[account(mut)]
//     pub creator: Signer<'info>,
//     #[account(mut)]
//     pub program: Account<'info, ProgramData>,
//     #[account(mut)]
//     pub fight_card: Account<'info, FightCardData>,
//     #[account(
//     init,
//     payer = creator,
//     seeds = [MY_APP_PREFIX, EVENT, program.event_counter.to_le_bytes().as_ref()],
//     bump,
//     space = 8 + 1 + 8 + 8
//     )]
//     pub registration: Account<'info, EventData>,
//     pub system_program: Program<'info, System>,
// }
