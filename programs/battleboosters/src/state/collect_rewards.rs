use super::program::ProgramData;
use crate::constants::*;
use crate::state::event::EventData;
use crate::state::mystery_box::MysteryBoxData;
use crate::state::player::PlayerData;
use crate::state::rank::RankData;
use crate::state::rarity::RarityData;
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

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
    pub rank: Box<Account<'info, RankData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PLAYER, player_account.creator.key().as_ref()],
    bump,
    )]
    pub player_account: Box<Account<'info, PlayerData>>,
    #[account(
    init,
    payer = signer,
    seeds = [MY_APP_PREFIX, MYSTERY_BOX, player_account.order_nonce.to_le_bytes().as_ref(), rank.player_account.key().as_ref()],
    bump,
    space = 128
    )]
    pub mystery_box: Box<Account<'info, MysteryBoxData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, RARITY],
    bump,
    )]
    pub rarity: Box<Account<'info, RarityData>>,
    // /// CHECK: Switchboard network price feed id
    // pub price_feed: AccountInfo<'info>,
    pub price_feed: Account<'info, PriceUpdateV2>,

    pub system_program: Program<'info, System>,
}
