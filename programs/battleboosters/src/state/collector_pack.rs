use super::program::ProgramData;
use crate::constants::*;
use crate::state::player::PlayerData;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(player_pubkey: Pubkey)]
pub struct InitializeCollectorPack<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        seeds = [MY_APP_PREFIX, PLAYER, player_pubkey.as_ref()],
        bump,
    )]
    pub player_account: Account<'info, PlayerData>,
    #[account(
        mut,
        seeds = [MY_APP_PREFIX, COLLECTOR, player_pubkey.as_ref(), player_account.order_nonce.to_le_bytes().as_ref()],
        bump,
    )]
    pub collector_pack: Account<'info, CollectorPack>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CollectorPack {
    pub fighter_mint_allowance: u64,
    pub booster_mint_allowance: u64,
    pub randomness: Option<Vec<u8>>,
}
