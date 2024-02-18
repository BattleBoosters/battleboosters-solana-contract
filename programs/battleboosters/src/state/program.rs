use anchor_lang::prelude::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(init, payer = creator,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump,
    space = 8 + 8 + 32 + 8 + 8 + 8 + 8 + 1 + 1 + 1)]
    pub program: Account<'info, ProgramData>,
    /// CHECK: This is a PDA used as the mint authority
    #[account(mut, seeds = [MY_APP_PREFIX, MINT_AUTHORITY], bump = authority_bump)]
    pub mint_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProgramData {
    /// Represent the current amount of created event
    pub event_counter: u64,
    /// The authority which are allowed to administrate the contract
    pub admin_pubkey: Pubkey,
    /// The price in USD of each NFT fighter pack
    pub fighter_pack_price: u64,
    /// The price in USD of each NFT points booster
    pub booster_points_price: u64,
    /// The price in USD of each NFT energy booster
    pub booster_energy_price: u64,
    /// The price in USD of each NFT shield booster
    pub booster_shield_price: u64,
    /// The amount of fighters contained on each NFT fighter pack
    pub fighter_pack_amount: u8,
    /// This data prevent re-initialization
    pub is_initialized: bool,
    /// Authority bump
    pub authority_bump: u8,
}
