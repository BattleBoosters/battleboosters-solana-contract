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
    space = 8 + 8
    )]
    pub player_account: Account<'info, PlayerData>,
    #[account(
    init,
    payer = creator,
    seeds = [MY_APP_PREFIX, INVENTORY, player_pubkey.as_ref()],
    bump,
    space = 8 + 8 + 8 + 1
    )]
    pub inventory: Account<'info, InventoryData>,
    pub system_program: Program<'info, System>,
}

// Struct for managing player inventory
#[derive(Accounts)]
pub struct PlayerInventory<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut,
    seeds = [MY_APP_PREFIX, INVENTORY, signer.key().as_ref()],
    bump)]
    pub inventory: Account<'info, InventoryData>,
    pub system_program: Program<'info, System>,
}

// // Struct for managing player inventory
// #[derive(Accounts)]
// pub struct ConsumeRandomness<'info> {
//     /// We need to make sure the randomness service signed this requests so it can only be invoked by a PDA and not a user.
//     #[account(
//     signer,
//     seeds = [b"STATE"],
//     seeds::program = SolanaRandomnessServiceID,
//     bump = randomness_state.bump,
//     )]
//     pub randomness_state: Box<Account<'info, solana_randomness_service::State>>,
//     pub request: Box<Account<'info, SimpleRandomnessV1Account>>,
//
//     #[account(mut)]
//     pub recipient: AccountInfo<'info>,
//     #[account(
//     mut,
//     seeds = [MY_APP_PREFIX, RARITY],
//     bump
//     )]
//     pub rarity: Account<'info, RarityData>,
//     /// CHECK: account constraints checked in account trait
//     #[account(address = sysvar::instructions::ID)]
//     pub sysvar_instructions: AccountInfo<'info>,
//     pub token_program: Program<'info, Token>,
//     pub system_program: Program<'info, System>,
// }

#[account]
pub struct InventoryData {
    /// Represent the current amount of fighter mint allowance available
    pub fighter_mint_allowance: u64,
    /// Represent the current amount of booster mint allowance available
    pub booster_mint_allowance: u64,
    /// This data prevent re-initialization
    pub is_initialized: bool,
}

#[account]
pub struct PlayerData {
    /// Represent the nonce of the current amount orders the player have made
    pub order_nonce: u64,
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
