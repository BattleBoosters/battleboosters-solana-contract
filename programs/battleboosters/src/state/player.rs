use super::mystery_box::MysteryBoxData;
use super::program::ProgramData;
use crate::constants::*;
use crate::state::fight_card::FightCardData;
use anchor_lang::prelude::*;

use crate::state::event::{EventData, EventLinkData};
use crate::state::rarity::RarityData;
use crate::types::FighterColorSide;
use anchor_lang::solana_program::sysvar;
use solana_randomness_service::SimpleRandomnessV1Account;
use solana_randomness_service::{
    program::SolanaRandomnessService, ID as SolanaRandomnessServiceID,
};
use switchboard_solana::prelude::*;
use crate::state::mintable_game_asset::{MintableGameAssetData, MintableGameAssetLinkData};

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
    pub collector_pack: Box<Account<'info, MysteryBoxData>>,
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
    pub player_game_asset_link: Box<Account<'info, MintableGameAssetLinkData>>,

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

/*

   TODO: Store the PDA used to get back the Metadata when resolving the event

   TODO: UPDATE THE SPAAAAAAAAAAAAACEEEEEEEEEEEE!!!

*/





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
