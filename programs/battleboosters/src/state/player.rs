use super::collector_pack::CollectorPack;
use super::program::ProgramData;
use crate::constants::*;
use crate::state::fight_card::FightCardData;
use anchor_lang::prelude::*;

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
    mut,
    seeds = [MY_APP_PREFIX, NFT_PRE_MINT, program.pre_mint_nonce.to_le_bytes().as_ref()],
    bump,
    )]
    pub nft_pre_mint: Box<Account<'info, NftPreMintData>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, NFT_PRE_MINT, signer.key().as_ref(), player_account.nft_pre_mint_player_nonce.to_le_bytes().as_ref()],
    bump,
    )]
    pub nft_pre_mint_player: Box<Account<'info, NftPreMintPlayerData>>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct NftPreMintPlayerData {
    pub nft_pre_mint_nonce: u64,
}

#[account]
pub struct NftPreMintData {
    pub is_locked: bool,
    pub is_minted: bool,
    pub owner: Pubkey,
    pub metadata: NftMetadata,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct NftMetadata {
    name: String,
    description: String,
    image: String,
    animation_url: Option<String>,
    external_url: Option<String>,
    attributes: Vec<Attribute>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Attribute {
    trait_type: String,
    value: String,
}

#[account]
pub struct PlayerData {
    /// Represent the nonce of the current amount orders the player have made
    pub order_nonce: u64,
    pub nft_pre_mint_player_nonce: u64,
    pub is_initialized: bool,
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
