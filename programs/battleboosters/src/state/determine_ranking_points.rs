/*
   TODO: Create determine ranking points accounts
*/
use crate::constants::*;
use crate::state::event::EventData;
use crate::state::fight_card::{FightCardData, FightCardLinkData};
use crate::state::fighter::FighterData;
use crate::state::mintable_game_asset::{MintableGameAssetData, MintableGameAssetLinkData};
use crate::state::player::PlayerData;
use crate::state::rank::RankData;
use crate::types::FighterType;
use anchor_lang::prelude::*;
use switchboard_solana::prelude::*;

#[derive(Accounts)]
#[instruction(fighter_type: FighterType)]
pub struct DetermineRankingPoints<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /*
       TODO:
           - Add player rank to modify the points. Rank will be determined off-chain
           - Store the sequence per rounds and Determine ranks per rounds
    */
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
    seeds = [MY_APP_PREFIX, PLAYER, rank.player_account.key().as_ref()],
    bump,
    )]
    pub player_account: Box<Account<'info, PlayerData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), fight_card.nonce.to_le_bytes().as_ref()],
    bump,
    )]
    pub fight_card: Box<Account<'info, FightCardData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), fight_card.key().as_ref(), rank.player_account.key().as_ref()],
    bump
    )]
    pub fight_card_link: Box<Account<'info, FightCardLinkData>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fight_card_link.fighter_nonce_tracker.unwrap().to_le_bytes().as_ref()],
    // close = signer,
    // constraint = fighter_asset.is_burned,
    bump
    )]
    pub fighter_asset: Box<Account<'info, MintableGameAssetData>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fighter_asset_link.nonce.to_le_bytes().as_ref(), signer.key().as_ref()],
    bump,
    )]
    pub fighter_asset_link: Box<Account<'info, MintableGameAssetLinkData>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fight_card_link.points_booster_nonce_tracker.unwrap().to_le_bytes().as_ref()],
    bump
    )]
    pub points_booster_asset: Option<Box<Account<'info, MintableGameAssetData>>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fight_card_link.shield_booster_nonce_tracker.unwrap().to_le_bytes().as_ref()],
    bump
    )]
    pub shield_booster_asset: Option<Box<Account<'info, MintableGameAssetData>>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHTER, &[fighter_type.clone() as u8]],
    bump,
    )]
    pub fighter: Account<'info, FighterData>,
}
