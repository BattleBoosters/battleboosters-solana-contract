use crate::constants::*;
use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Accounts)]
pub struct InitializeGlobalState<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init,
    payer = signer,
    space = 8 + 8 + 32 + 5 + 13 + 8 + 8,
    )]
    pub new_account: Account<'info, GlobalStateData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GlobalStateData {
    pub event_counter: u64,
    pub admin_pubkey: Pubkey,
    pub rarity_probabilities: Vec<u8>,
    pub rarity: Rarity,
    pub nft_fighter_pack_price: u64,
    pub nft_booster_pack_price: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
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
