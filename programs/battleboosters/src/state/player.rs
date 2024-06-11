use crate::constants::*;
use anchor_lang::prelude::*;
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
    space = 8 + 32 + 8 + 8 + 1 + 5
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

#[account]
pub struct PlayerData {
    pub creator: Pubkey,
    /// Represent the nonce of the current amount orders the player have created
    pub order_nonce: u64,
    /// Represent the nonce of the current player game asset link the player have created
    pub player_game_asset_link_nonce: u64,
    /// Prevent accidental multiple initializations of a PDA
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
