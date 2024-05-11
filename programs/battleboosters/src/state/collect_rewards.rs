use super::program::ProgramData;
use crate::constants::*;
use crate::state::event::EventData;
use crate::state::mystery_box::MysteryBoxData;
use crate::state::player::PlayerData;
use crate::state::rank::RankData;
use crate::state::rarity::RarityData;
use crate::ErrorCode;
use anchor_lang::prelude::*;
use std::str::FromStr;
use switchboard_solana::prelude::*;
use switchboard_solana::AggregatorAccountData;

#[derive(Accounts)]
pub struct CollectRewards<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump
    )]
    pub program: Box<Account<'info, ProgramData>>,
    /// CHECK: This is a PDA used as the bank
    #[account(mut, seeds = [MY_APP_PREFIX, BANK], bump)]
    pub bank: AccountInfo<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event.nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Box<Account<'info, EventData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, RANK, event.key().as_ref(), rank.nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub rank: Account<'info, RankData>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PLAYER, rank.player_account.key().as_ref()],
    bump,
    )]
    pub player_account: Account<'info, PlayerData>,
    #[account(
    init,
    payer = signer,
    seeds = [MY_APP_PREFIX, MYSTERY_BOX, rank.player_account.key().as_ref()],
    bump,
    space = 128
    )]
    pub mystery_box: Account<'info, MysteryBoxData>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, RARITY],
    bump,
    )]
    pub rarity: Account<'info, RarityData>,
    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
    /// CHECK: Switchboard network price feed id
    #[account(address = Pubkey::from_str(SOL_USD_FEED_MAINNET).unwrap() @ ErrorCode::InvalidPriceFeed)]
    pub price_feed: AccountLoader<'info, AggregatorAccountData>,

    pub system_program: Program<'info, System>,
}
