use super::collector_pack::CollectorPack;
use super::program::ProgramData;
use crate::constants::*;
use crate::state::fight_card::FightCardData;
use anchor_lang::prelude::*;

use crate::state::event::EventData;
use crate::state::rarity::RarityData;
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
pub struct GenerateRandomNftPreMint<'info> {
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
    pub rarity: Box<Account<'info, RarityData>>,
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
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, signer.key().as_ref(), player_game_asset_link_nonce.to_le_bytes().as_ref()],
    space = 8 + 8 + 1,
    bump,
    )]
    pub player_game_asset_link: Box<Account<'info, PlayerGameAssetLinkData>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(fight_card_id: u8, fighter_m_game_asset_id: u64, energy_booster_m_game_asset_id: u64, shield_booster_m_game_asset_id: u64, points_booster_m_game_asset_id: u64)]
pub struct JoinFightCard<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub program: Account<'info, ProgramData>,
    #[account(mut)]
    pub event: Account<'info, EventData>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fighter_m_game_asset_id.to_le_bytes().as_ref()],
    bump
    )]
    pub fighter_m_game_asset: Box<Account<'info, MintableGameAssetData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, energy_booster_m_game_asset_id.to_le_bytes().as_ref()],
    bump
    )]
    pub energy_booster_m_game_asset: Option<Box<Account<'info, MintableGameAssetData>>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, shield_booster_m_game_asset_id.to_le_bytes().as_ref()],
    bump
    )]
    pub shield_booster_m_game_asset: Option<Box<Account<'info, MintableGameAssetData>>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, points_booster_m_game_asset_id.to_le_bytes().as_ref()],
    bump
    )]
    pub points_booster_m_game_asset: Option<Box<Account<'info, MintableGameAssetData>>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, fight_card_id.to_le_bytes().as_ref()],
    bump
    )]
    pub fight_card: Account<'info, FightCardData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlayerData {
    /// Represent the nonce of the current amount orders the player have made
    pub order_nonce: u64,
    pub player_game_asset_link_nonce: u64,
    pub is_initialized: bool,
}

#[account]
pub struct PlayerGameAssetLinkData {
    // TODO: Probably save the `mintable_game_asset` Pubkey for convenience?
    /// this is the link to the address of the pda
    pub mintable_game_asset_nonce: u64,
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
