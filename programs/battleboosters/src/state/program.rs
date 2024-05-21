use crate::constants::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(authority_bump: u8, bank_bump: u8)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(init, payer = creator,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump,
    space = 8 + 8 + 8 + 32 + 8 + 8 + 1 + 1 + 1 + 1 + 1)]
    pub program: Box<Account<'info, ProgramData>>,
    /// CHECK: This is a PDA used as the bank
    #[account(mut, seeds = [MY_APP_PREFIX, BANK], bump = bank_bump)]
    pub bank: AccountInfo<'info>,
    /// CHECK: This is a PDA used as the mint authority
    #[account(mut, seeds = [MY_APP_PREFIX, MINT_AUTHORITY], bump = authority_bump)]
    pub mint_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProgram<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump
    )]
    pub program: Box<Account<'info, ProgramData>>,
    pub system_program: Program<'info, System>,
}

/* TODO: change fighter pack by fighter for convenience remove the pack stuffs*/
#[account]
pub struct ProgramData {
    /// Represent the current amount of created event
    pub event_nonce: u64,
    /// Represent the current amount of mintable game asset pack
    pub mintable_game_asset_nonce: u64,
    /// The authority which are allowed to administrate the contract
    pub admin_pubkey: Pubkey,
    /// The price in USD of each NFT fighter pack
    pub fighter_pack_price: u64,
    /// The price in USD of each NFT booster
    pub booster_price: u64,
    /// The amount of fighters contained on each NFT fighter pack
    pub fighter_pack_amount: u8,
    /// This data prevent re-initialization
    pub is_initialized: bool,
    /// Authority bump
    pub authority_bump: u8,
    /// Bank bump
    pub bank_bump: u8,
    pub env: Env,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum Env {
    Dev,
    Prod,
}
