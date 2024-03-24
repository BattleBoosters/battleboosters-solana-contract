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
        seeds = [MY_APP_PREFIX, MYSTERY_BOX, player_pubkey.key().as_ref(), player_account.order_nonce.to_le_bytes().as_ref()],
        bump,
        space = 528
    )]
    pub mystery_box: Account<'info, MysteryBoxData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MysteryBoxData {
    pub fighter_mint_allowance: u64,
    pub booster_mint_allowance: u64,
    pub champions_pass_mint_allowance: u64,
    pub randomness: Option<Vec<u8>>,
    pub probability_tier: TierProbabilities,
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
