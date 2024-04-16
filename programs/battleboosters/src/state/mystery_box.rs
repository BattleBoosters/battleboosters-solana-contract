use crate::constants::*;
use crate::state::player::PlayerData;
use crate::state::rarity::TierProbabilities;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(player_pubkey: Pubkey)]
pub struct InitializeMysteryBox<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        seeds = [MY_APP_PREFIX, PLAYER, player_pubkey.as_ref()],
        bump,
    )]
    pub player_account: Account<'info, PlayerData>,
    #[account(
        init,
        payer = creator,
        seeds = [MY_APP_PREFIX, MYSTERY_BOX, player_account.order_nonce.to_le_bytes().as_ref(), player_pubkey.as_ref()],
        bump,
        space = 128 + 32 + 8
    )]
    pub mystery_box: Account<'info, MysteryBoxData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MysteryBoxData {
    pub fighter_mint_allowance: u64,
    pub booster_mint_allowance: u64,
    pub champions_pass_mint_allowance: u64,
    pub randomness_account: Pubkey, // Reference to the Switchboard randomness account
    pub randomness: Option<Vec<u8>>,
    pub probability_tier: TierProbabilities,
    /// Nonce of the `mystery_box`
    pub nonce: u64,
}
// {
// "attributes": [
//     {
//     "trait_type": "Fighting Style",
//     "value": "Boxing"
//     },
//     {
//     "trait_type": "Category",
//     "value": "Striker"
//     },
//     {
//     "trait_type": "Rarity",
//     "value": "Common"
//     },
//     {
//     "trait_type": "Health",
//     "value": 100
//     },
//     {
//     "trait_type": "Power",
//     "value": 100
//     },
//     {
//     "trait_type": "Energy",
//     "value": 200
//     },
// ]
// }
