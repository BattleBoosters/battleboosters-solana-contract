use crate::constants::*;
use crate::state::fight_card::{FightCardData, FightCardLinkData};
use anchor_lang::prelude::*;

use crate::state::event::{EventData, EventLinkData};
use crate::state::mintable_game_asset::{MintableGameAssetData, MintableGameAssetLinkData};
use switchboard_solana::prelude::*;

#[derive(Accounts)]
pub struct JoinFightCard<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event.nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Box<Account<'info, EventData>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fighter_asset.nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub fighter_asset: Box<Account<'info, MintableGameAssetData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, shield_booster_asset.nonce.to_le_bytes().as_ref()],
    // constraint = shield_booster_asset.as_ref().is_burned == true,
    // close = signer,
    bump
    )]
    pub shield_booster_asset: Option<Box<Account<'info, MintableGameAssetData>>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, points_booster_asset.nonce.to_le_bytes().as_ref()],
    // constraint = points_booster_asset.as_ref().is_burned == true,
    // close = signer,
    bump
    )]
    pub points_booster_asset: Option<Box<Account<'info, MintableGameAssetData>>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, fighter_link.nonce.to_le_bytes().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub fighter_link: Box<Account<'info, MintableGameAssetLinkData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, shield_booster_link.nonce.to_le_bytes().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub shield_booster_link: Option<Box<Account<'info, MintableGameAssetLinkData>>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, points_booster_link.nonce.to_le_bytes().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub points_booster_link: Option<Box<Account<'info, MintableGameAssetLinkData>>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), fight_card.nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub fight_card: Box<Account<'info, FightCardData>>,

    #[account(
    init,
    payer = signer,
    space = 260,
    seeds = [MY_APP_PREFIX, FIGHT_CARD, event.key().as_ref(), fight_card.key().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub fight_card_link: Box<Account<'info, FightCardLinkData>>,

    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event.key().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub event_link: Box<Account<'info, EventLinkData>>,

    pub system_program: Program<'info, System>,
}
