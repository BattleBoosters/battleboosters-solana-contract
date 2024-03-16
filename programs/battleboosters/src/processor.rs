use crate::errors::ErrorCode;
use crate::events::*;
use crate::state::event::{CreateEvent, UpdateEvent};
use crate::state::fight_card::{CreateFightCard, FightCardData, UpdateFightCard};
use crate::state::program::InitializeProgram;
use crate::state::rarity::{InitializeRarity, RarityBooster, RarityFighter};
use crate::utils::{set_fight_card_properties, verify_equality};
use anchor_lang::prelude::*;

pub fn initialize(
    ctx: Context<InitializeProgram>,
    authority_bump: u8,
    bank_bump: u8,
    admin_pubkey: Pubkey,
    nft_fighter_pack_price: u64,
    booster_price: u64,
    fighter_pack_amount: u8,
) -> Result<()> {
    let program = &mut ctx.accounts.program;
    require!(!program.is_initialized, ErrorCode::AlreadyInitialized);

    program.authority_bump = authority_bump;
    program.bank_bump = bank_bump;
    program.event_nonce = 0_u64;
    program.mintable_game_asset_nonce = 0_u64;
    program.admin_pubkey = admin_pubkey;
    program.fighter_pack_price = nft_fighter_pack_price;
    program.booster_price = booster_price;
    program.fighter_pack_amount = fighter_pack_amount;
    program.is_initialized = true;

    msg!("Program Initialized");

    Ok(())
}

pub fn initialize_rarity(
    ctx: Context<InitializeRarity>,
    fighter: Vec<RarityFighter>,
    energy_booster: Vec<RarityBooster>,
    shield_booster: Vec<RarityBooster>,
    points_booster: Vec<RarityBooster>,
    fighter_probabilities: Vec<u8>,
    booster_probabilities: Vec<u8>,
) -> Result<()> {
    let rarity = &mut ctx.accounts.rarity;
    require!(!rarity.is_initialized, ErrorCode::AlreadyInitialized);

    rarity.fighter = fighter;
    rarity.energy_booster = energy_booster;
    rarity.shield_booster = shield_booster;
    rarity.points_booster = points_booster;
    rarity.fighter_probabilities = fighter_probabilities;
    rarity.booster_probabilities = booster_probabilities;
    rarity.is_initialized = true;

    msg!("Rarity Initialized");

    Ok(())
}

pub fn create_new_event(ctx: Context<CreateEvent>, start_date: i64, end_date: i64) -> Result<()> {
    let program = &mut ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

    // Create event account and set data
    let create_event = &mut ctx.accounts.event;
    create_event.fight_card_id_counter = 0_u8;
    create_event.start_date = start_date;
    create_event.end_date = end_date;

    emit!(EventCreated {
        event_id: program.event_nonce
    });

    // Increment event counter
    program.event_nonce += 1_u64;

    Ok(())
}

pub fn update_event(
    ctx: Context<UpdateEvent>,
    event_id: u64, // used in instruction
    start_date: i64,
    end_date: i64,
) -> Result<()> {
    let program = &ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

    let update_event = &mut ctx.accounts.event;
    update_event.start_date = start_date;
    update_event.end_date = end_date;

    emit!(EventUpdated {
        event_id: program.event_nonce
    });

    Ok(())
}

pub fn create_new_fight_card(
    ctx: Context<CreateFightCard>,
    event_id: u64, // used in instruction
    params: FightCardData
) -> Result<()> {
    let program = &ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

    let fight_card = &mut ctx.accounts.fight_card;
    set_fight_card_properties(fight_card, &params);

    let event = &mut ctx.accounts.event;
    event.fight_card_id_counter = event.fight_card_id_counter.checked_add(1_u8).unwrap();

    emit!(FightCardCreated {
        fight_card_id: fight_card.id
    });

    Ok(())
}

pub fn update_fight_card(
    ctx: Context<UpdateFightCard>,
    event_id: u64, // used in instruction
    fight_card_id: u8, // used in instruction
    params: FightCardData,
) -> Result<()> {
    let program = &ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

    let fight_card = &mut ctx.accounts.fight_card;
    set_fight_card_properties(fight_card, &params);

    emit!(FightCardUpdated {
        fight_card_id: fight_card.id
    });

    Ok(())
}
