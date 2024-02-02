use crate::constants::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
    init,
    payer = creator,
    space = 8 + 8 + 32 + 5 + 13 + 8 + 8,
    )]
    pub program: Account<'info, ProgramData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProgramData {
    pub event_counter: u64,
    pub admin_pubkey: Pubkey,
    pub rarity_probabilities: Vec<u8>,
    pub rarity: Rarity,
    pub nft_fighter_pack_price: u64,
    pub nft_booster_pack_price: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum Rarity {
    Common {
        power_min: u16,
        power_max: u16,
        lifespan_min: u8,
        lifespan_max: u8,
        energy_min: u8,
        energy_max: u8,
    },
    Uncommon {
        power_min: u16,
        power_max: u16,
    },
}
