use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
    init,
    payer = creator,
    space = 8 + 8 + 32 + 8 + 8 + 8 + 8 + 1,
    )]
    pub program: Account<'info, ProgramData>,
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
    pub booster_points: u64,
    /// The price in USD of each NFT energy booster
    pub booster_energy: u64,
    /// The price in USD of each NFT shield booster
    pub booster_shield: u64,
    /// The amount of fighters contained on each NFT fighter pack
    pub fighter_pack_amount: u8,
}
