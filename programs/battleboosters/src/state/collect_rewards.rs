use super::program::ProgramData;
use crate::constants::*;
use crate::state::event::EventData;
use crate::state::mystery_box::MysteryBoxData;
use crate::state::player::PlayerData;
use crate::state::rank::RankData;
use crate::state::rarity::RarityData;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(event_nonce: u64, rank_nonce: u64)]
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
    #[account(mut, seeds = [MY_APP_PREFIX, BANK], bump = program.bank_bump)]
    pub bank: AccountInfo<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event_nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Box<Account<'info, EventData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, RANK, event.key().as_ref(), rank_nonce.to_le_bytes().as_ref()],
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
    seeds = [MY_APP_PREFIX, MYSTERY_BOX, rank.player_account.key().as_ref(), player_account.order_nonce.to_le_bytes().as_ref()],
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

    pub system_program: Program<'info, System>,
}