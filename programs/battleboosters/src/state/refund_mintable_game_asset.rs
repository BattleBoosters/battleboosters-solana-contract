use crate::constants::*;
use crate::state::event::EventData;
use crate::state::fight_card::{FightCardData, FightCardLinkData};
use crate::state::mystery_box::MysteryBoxData;
use crate::state::player::PlayerData;
use crate::state::program::ProgramData;
use crate::state::rarity::RarityData;
use anchor_lang::prelude::*;
use anchor_lang::{account, AnchorDeserialize, AnchorSerialize};
use solana_program::pubkey::Pubkey;

use crate::state::mintable_game_asset::{MintableGameAssetData, MintableGameAssetLinkData};

/*
   TODO: Refund game mintable asset
*/

#[derive(Accounts)]
#[instruction(
    fighter_game_asset_link_nonce: u64,
    points_game_asset_link_nonce: u64,
    shield_game_asset_link_nonce: u64,
    player_pubkey: Pubkey
)]
pub struct RefundMintableGameAsset<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PLAYER, player_pubkey.as_ref()],
    bump,
    )]
    pub player_account: Box<Account<'info, PlayerData>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event.nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Box<Account<'info, EventData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), fight_card.nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub fight_card: Box<Account<'info, FightCardData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), fight_card.key().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub fight_card_link: Box<Account<'info, FightCardLinkData>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fight_card_link.fighter_nonce_tracker.unwrap().to_le_bytes().as_ref()],
    bump
    )]
    pub fighter_asset: Box<Account<'info, MintableGameAssetData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fight_card_link.shield_booster_nonce_tracker.unwrap().to_le_bytes().as_ref()],
    // constraint = shield_booster_asset.as_ref().is_burned == true,
    // close = signer,
    bump
    )]
    pub shield_booster_asset: Option<Box<Account<'info, MintableGameAssetData>>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fight_card_link.points_booster_nonce_tracker.unwrap().to_le_bytes().as_ref()],
    // constraint = points_booster_asset.as_ref().is_burned == true,
    // close = signer,
    bump
    )]
    pub points_booster_asset: Option<Box<Account<'info, MintableGameAssetData>>>,

    #[account(
    init_if_needed,
    payer = signer,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fighter_game_asset_link_nonce.to_le_bytes().as_ref(), player_pubkey.as_ref()],
    space = 8 + 32 + 8 + 1 + 8 + 8,
    bump,
    )]
    pub fighter_link: Box<Account<'info, MintableGameAssetLinkData>>,
    #[account(
    init_if_needed,
    payer = signer,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, points_game_asset_link_nonce.to_le_bytes().as_ref(), player_pubkey.as_ref()],
    space = 8 + 32 + 8 + 1 + 8 + 8,
    bump,
    )]
    pub points_booster_link: Option<Box<Account<'info, MintableGameAssetLinkData>>>,
    #[account(
    init_if_needed,
    payer = signer,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, shield_game_asset_link_nonce.to_le_bytes().as_ref(), player_pubkey.as_ref()],
    space = 8 + 32 + 8 + 1 + 8 + 8,
    bump,
    )]
    pub shield_booster_link: Option<Box<Account<'info, MintableGameAssetLinkData>>>,

    pub system_program: Program<'info, System>,
}
