use super::program::ProgramData;
use crate::constants::*;
use crate::state::fight_card::FightCardData;
use anchor_lang::prelude::*;

// Struct for initializing player
#[derive(Accounts)]
#[instruction(player_pubkey: Pubkey)]
pub struct InitializePlayer<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut, seeds = [MY_APP_PREFIX, PROGRAM_STATE], bump)]
    pub program: Account<'info, ProgramData>,
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
    pub creator: Signer<'info>,
    #[account(mut, seeds = [MY_APP_PREFIX, PROGRAM_STATE], bump)]
    pub program: Account<'info, ProgramData>,
    #[account(mut,
    seeds = [MY_APP_PREFIX, INVENTORY, creator.key().as_ref()],
    bump)]
    pub inventory: Account<'info, InventoryData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct InventoryData {
    /// Represent the current amount of fighter mint allowance available
    pub fighter_mint_allowance: u64,
    /// Represent the current amount of booster mint allowance available
    pub booster_mint_allowance: u64,
    /// This data prevent re-initialization
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
