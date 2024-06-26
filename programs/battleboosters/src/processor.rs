use crate::constants::{
    BANK, FEED_HEX, METADATA_OFF_CHAIN_URI, MINT_AUTHORITY, MY_APP_PREFIX, PRICE_DECIMALS,
    STALENESS_THRESHOLD,
};
use crate::errors::ErrorCode;
use crate::events::*;
use crate::state::collect_rewards::CollectRewards;
use crate::state::create_spl_nft::CreateSplNft;
use crate::state::determine_ranking_points::DetermineRankingPoints;
use crate::state::event::{CreateEvent, InitializeEventLink, RankReward, UpdateEvent};
use crate::state::fight_card::{CreateFightCard, FightCardData, UpdateFightCard};
use crate::state::fighter_base::{CreateFighterBase, FightMetrics};
use crate::state::join_fight_card::JoinFightCard;
use crate::state::mystery_box::UpdateMysteryBox;
// use crate::state::mint_nft_from_game_asset::MintNftFromGameAsset;
use crate::state::mintable_game_asset::Attribute;
use crate::state::mintable_game_asset::*;
use crate::state::player::InitializePlayer;
use crate::state::program::{Env, InitializeProgram, UpdateProgram};
use crate::state::rank::UpdateRank;
use crate::state::rarity::{
    InitializeRarity, RarityBooster, RarityFighter, TierProbabilities, TierType, UpdateRarity,
};
use crate::state::refund_mintable_game_asset::RefundMintableGameAsset;
use crate::state::transaction_escrow::TransactionEscrow;
use crate::types::{
    BoosterType, CollectionType, FightCardResult, FighterColorSide, FighterType, NftType,
    OpenRequest, PurchaseRequest, TournamentType,
};
use crate::utils::{
    asset_metadata_value, create_nft_metadata, create_rng_seed, find_rarity, find_scaled_rarity,
    metrics_calculation, process_and_verify_game_asset_type, process_game_asset,
    process_game_asset_for_action, set_fight_card_properties, verify_equality,
};

use anchor_lang::prelude::*;
use mpl_token_metadata::instructions::{CreateV1CpiBuilder, MintV1CpiBuilder};
use mpl_token_metadata::types::{PrintSupply, TokenStandard};
use pyth_solana_receiver_sdk::price_update::get_feed_id_from_hex;
use sha2::{Digest, Sha256};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::program::{invoke, invoke_signed};
use solana_program::system_instruction;
use switchboard_on_demand::accounts::RandomnessAccountData;

pub fn initialize(
    ctx: Context<InitializeProgram>,
    authority_bump: u8,
    bank_bump: u8,
    admin_pubkey: Pubkey,
    nft_fighter_price: u64,
    booster_price: u64,
    //fighter_pack_amount: u8,
    env: Env,
) -> Result<()> {
    let program = &mut ctx.accounts.program;
    require!(!program.is_initialized, ErrorCode::AlreadyInitialized);

    program.authority_bump = authority_bump;
    program.bank_bump = bank_bump;
    program.event_nonce = 0_u64;
    program.mintable_game_asset_nonce = 0_u64;
    program.admin_pubkey = admin_pubkey;
    program.fighter_price = nft_fighter_price;
    program.booster_price = booster_price;
    // program.fighter_pack_amount = fighter_pack_amount;
    program.env = env;
    program.is_initialized = true;

    msg!("Program Initialized");

    Ok(())
}

pub fn update_program(ctx: Context<UpdateProgram>) -> Result<()> {
    let program = &mut ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;
    program.env = Env::Dev;

    Ok(())
}

pub fn initialize_rarity(
    ctx: Context<InitializeRarity>,
    fighter: Vec<RarityFighter>,
    shield_booster: Vec<RarityBooster>,
    points_booster: Vec<RarityBooster>,
    probability_tiers: Vec<TierProbabilities>,
) -> Result<()> {
    let rarity = &mut ctx.accounts.rarity;
    require!(!rarity.is_initialized, ErrorCode::AlreadyInitialized);

    rarity.fighter = fighter;
    rarity.shield_booster = shield_booster;
    rarity.points_booster = points_booster;
    rarity.probability_tiers = probability_tiers;
    rarity.is_initialized = true;

    msg!("Rarity Initialized");

    Ok(())
}

pub fn update_rarity(
    ctx: Context<UpdateRarity>,
    fighter: Option<Vec<RarityFighter>>,
    shield_booster: Option<Vec<RarityBooster>>,
    points_booster: Option<Vec<RarityBooster>>,
    probability_tiers: Option<Vec<TierProbabilities>>,
) -> Result<()> {
    let rarity = &mut ctx.accounts.rarity;
    let program = &mut ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

    if let Some(fighter_found) = fighter {
        rarity.fighter = fighter_found;
    }
    if let Some(shield_booster_found) = shield_booster {
        rarity.shield_booster = shield_booster_found;
    }
    if let Some(points_booster_found) = points_booster {
        rarity.points_booster = points_booster_found;
    }
    if let Some(probability_tiers_found) = probability_tiers {
        rarity.probability_tiers = probability_tiers_found;
    }
    msg!("Rarity Updated");

    Ok(())
}

pub fn initialize_event_link(ctx: Context<InitializeEventLink>) -> Result<()> {
    let event_link = &mut ctx.accounts.event_link;
    let rank = &mut ctx.accounts.rank;
    let event = &mut ctx.accounts.event;

    require!(!event_link.is_initialized, ErrorCode::AlreadyInitialized);

    let (champions_pass_pubkey, champions_pass_nonce_tracker) =
        if event.tournament_type == TournamentType::MainCard {
            let champions_pass_asset = ctx
                .accounts
                .champions_pass_asset
                .as_mut()
                .ok_or(ErrorCode::MissingChampionsPassAsset)?;
            let champions_pass_link = ctx
                .accounts
                .champions_pass_link
                .as_mut()
                .ok_or(ErrorCode::MissingChampionsPassLink)?;

            verify_equality(
                &champions_pass_asset.to_account_info().key(),
                &champions_pass_link.mintable_game_asset_pubkey,
            )?;

            champions_pass_asset.owner = None;
            champions_pass_asset.is_burned = true;
            champions_pass_link.is_free = true;

            msg!(
                "Event link to champion's pass: {}",
                champions_pass_link.to_account_info().key()
            );

            (
                Some(champions_pass_link.mintable_game_asset_pubkey),
                Some(champions_pass_link.mintable_game_asset_nonce_tracker),
            )
        } else {
            (None, None)
        };

    // Configure event link
    event_link.event_pubkey = event.to_account_info().key();
    event_link.event_nonce_tracker = event.nonce;
    event_link.champions_pass_pubkey = champions_pass_pubkey;
    event_link.champions_pass_nonce_tracker = champions_pass_nonce_tracker;
    // For recreating the rank pda with nonce
    event_link.rank_nonce = event.rank_nonce;
    event_link.is_initialized = true;
    msg!("Event link created: {}", event_link.to_account_info().key());

    // Configure rank
    rank.player_account = ctx.accounts.creator.to_account_info().key();
    rank.total_points = None;
    rank.rank = None;
    rank.is_consumed = false;
    rank.nonce = event.rank_nonce;
    msg!("Rank created: {}", rank.to_account_info().key());

    // Update event nonce safely
    event.rank_nonce = event.rank_nonce.checked_add(1).unwrap();
    msg!("Event nonce updated: {}", event.rank_nonce);

    Ok(())
}
pub fn initialize_player(
    ctx: Context<InitializePlayer>,
    player_pubkey: Pubkey, /* Used in initialization */
) -> Result<()> {
    let player_account = &mut ctx.accounts.player_account;
    require!(
        !player_account.is_initialized,
        ErrorCode::AlreadyInitialized
    );

    player_account.creator = player_pubkey;
    player_account.order_nonce = 0;
    player_account.player_game_asset_link_nonce = 0;
    player_account.is_initialized = true;

    msg!("Player Initialized");

    Ok(())
}
pub fn create_nft_collection(
    ctx: Context<CreateSplNft>,
    _collection_id: CollectionType, /* Used in initialization */
    collection_name: String,
    symbol: String,
    uri: String,
    fees: u16,
) -> Result<()> {
    let program = &ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

    let metadata_program = ctx.accounts.metadata_program.to_account_info();
    let authority = ctx.accounts.mint_authority.to_account_info();
    let payer = ctx.accounts.creator.to_account_info();
    let sysvar = ctx.accounts.sysvar_instructions.to_account_info();
    let spl_token_program = ctx.accounts.token_program.to_account_info();
    let metadata = ctx.accounts.metadata.to_account_info();
    let minter = ctx.accounts.minter.to_account_info();
    let token_account = ctx.accounts.token_account.to_account_info();
    let token_record = ctx.accounts.token_record.to_account_info();

    let mut binding_create = CreateV1CpiBuilder::new(&metadata_program);

    let create_cpi = binding_create
        .metadata(&metadata)
        .mint(&minter, false)
        .authority(&authority)
        .payer(&payer)
        .update_authority(&authority, true)
        .master_edition(Some(&ctx.accounts.master_edition))
        // .collection(Collection {
        //     key: minter.key.clone(),
        //     verified: false,
        // })
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&sysvar)
        .spl_token_program(Some(&spl_token_program))
        .token_standard(TokenStandard::ProgrammableNonFungible)
        .name(collection_name)
        .symbol(symbol)
        .uri(uri)
        .seller_fee_basis_points(fees)
        .is_mutable(true)
        .print_supply(PrintSupply::Zero);

    let mut binding_mint = MintV1CpiBuilder::new(&metadata_program);
    let mint_cpi = binding_mint
        .token(&token_account)
        .token_owner(Some(&authority))
        .metadata(&metadata)
        .token_record(Some(&token_record))
        .master_edition(Some(&ctx.accounts.master_edition))
        .mint(&minter)
        .payer(&payer)
        .authority(&authority)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        .spl_token_program(&ctx.accounts.token_program)
        .spl_ata_program(&ctx.accounts.associated_token_program)
        .amount(1);

    let authority_seeds = [
        MY_APP_PREFIX,
        MINT_AUTHORITY,
        &[program.authority_bump.clone()],
    ];

    create_cpi.invoke_signed(&[&authority_seeds])?;
    mint_cpi.invoke_signed(&[&authority_seeds])?;

    Ok(())
}

pub fn create_fighter(
    ctx: Context<CreateFighterBase>,
    fighter_type: FighterType,
    fight_metrics: FightMetrics,
) -> Result<()> {
    let program = &ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

    let fighter = &mut ctx.accounts.fighter_base;
    fighter.fighter_type = fighter_type;
    fighter.fight_metrics = fight_metrics;
    Ok(())
}

pub fn update_fighter(
    ctx: Context<CreateFighterBase>,
    fighter_type: FighterType,
    fight_metrics: FightMetrics,
) -> Result<()> {
    let program = &ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

    let fighter = &mut ctx.accounts.fighter_base;
    fighter.fighter_type = fighter_type;
    fighter.fight_metrics = fight_metrics;

    Ok(())
}

/*
   TODO: Update mystery box randomness account
*/
pub fn update_randomness_mystery_box(
    ctx: Context<UpdateMysteryBox>,
    _mystery_box_nonce: u64, // Used in instruction
    _player_pubkey: Pubkey,  // Used in instruction
) -> Result<()> {
    let clock = Clock::get()?;
    let mystery_box = &mut ctx.accounts.mystery_box;
    let program = &ctx.accounts.program;

    // Used for testing in local-net without depending on external services
    match program.env {
        Env::Prod => {
            let randomness_data =
                RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow())
                    .unwrap();
            require!(
                randomness_data.seed_slot == clock.slot - 1,
                ErrorCode::RandomnessAlreadyRevealed
            );
            // Set the randomness account
            mystery_box.randomness_account =
                Some(ctx.accounts.randomness_account_data.to_account_info().key());
        }
        Env::Dev => {
            mystery_box.randomness_account = Some(ctx.accounts.creator.to_account_info().key());
        }
    }

    Ok(())
}

pub fn purchase_mystery_box(
    ctx: Context<TransactionEscrow>,
    requests: Vec<PurchaseRequest>,
) -> Result<()> {
    let program = &ctx.accounts.program;
    let feed_account = &ctx.accounts.price_feed;
    let mystery_box = &mut ctx.accounts.mystery_box;
    let player_account = &mut ctx.accounts.player_account;
    let bank = &mut ctx.accounts.bank;
    let recipient = &ctx.accounts.recipient;
    // let bank_escrow = &mut ctx.accounts.bank_escrow;
    let signer_key = &ctx.accounts.signer.to_account_info().key();
    let rarity = &ctx.accounts.rarity;

    let feed_id: [u8; 32] = get_feed_id_from_hex(FEED_HEX)?;
    let price =
        feed_account.get_price_no_older_than(&Clock::get()?, STALENESS_THRESHOLD, &feed_id)?;
    let val = (price.price as f64) * 10f64.powi(price.exponent);

    let sol_per_usd = 1.0 / val;
    let mut total_usd = 0;

    for request in &requests {
        match request.nft_type {
            NftType::Booster => {
                // update the quantity of fighter mint allowance
                mystery_box.booster_mint_allowance += request.quantity.clone();
                //total_usd += (request.quantity as f64) * program.booster_price;

                total_usd += request
                    .quantity
                    .checked_mul(program.booster_price.clone())
                    .unwrap();
            }
            NftType::Fighter => {
                // update the quantity of fighter mint allowance
                // mystery_box.fighter_mint_allowance += request
                //     .quantity
                //     .checked_mul(program.fighter_pack_amount.clone() as u64)
                //     .unwrap();
                mystery_box.fighter_mint_allowance += request.quantity.clone();
                //total_usd += (request.quantity as f64) * program.fighter_pack_price;
                total_usd += request
                    .quantity
                    .checked_mul(program.fighter_price.clone())
                    .unwrap();
            }
            _ => return Err(error!(ErrorCode::UnsupportedNftType)),
        }
    }

    require!(total_usd > 0, ErrorCode::InsufficientAmount);

    let total_sol = (total_usd as f64 / PRICE_DECIMALS as f64) * sol_per_usd;
    let total_lamports = (total_sol * LAMPORTS_PER_SOL as f64).round() as u64;
    //let bank_escrow_balance = bank_escrow.lamports();
    msg!("bank balance before: {}", bank.lamports());
    msg!("total usd: {}", total_usd);
    msg!("total sol: {}", total_sol);
    msg!("total lamports: {}", total_lamports);
    // if bank_escrow_balance < total_lamports {
    //     msg!(
    //         "Insufficient funds: required {}, available {}.",
    //         total_lamports,
    //         bank_escrow_balance
    //     );
    //     return Err(ErrorCode::InsufficientFunds.into());
    // }

    let transfer_instruction = system_instruction::transfer(
        signer_key,
        &bank.key(),
        // Withdraw the full balance
        total_lamports, // Amount in lamports to transfer
    );

    // let bank_escrow_seeds = [
    //     MY_APP_PREFIX,
    //     BANK,
    //     signer_key.as_ref(),
    //     &[bank_escrow_bump],
    // ];
    //Perform the transfer
    invoke(
        &transfer_instruction,
        &[
            ctx.accounts.signer.to_account_info(),
            bank.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    //Perform the transfer
    // invoke_signed(
    //     &transfer_instruction,
    //     &[
    //         bank_escrow.to_account_info(),
    //         bank.to_account_info(),
    //         ctx.accounts.system_program.to_account_info(),
    //     ],
    //     &[&bank_escrow_seeds],
    // )?;
    msg!("bank balance now: {}", bank.lamports());

    // Set the randomness account to default
    mystery_box.randomness_account = None;
    // Set the collector pack to default `champion_s_pass_mint_allowance`
    mystery_box.champions_pass_mint_allowance = 0;

    // Set the order nonce to regenerate the PDA
    mystery_box.nonce = player_account.order_nonce;

    if let Some(probability_tier) = rarity.get_probability_by_tier(TierType::Tier3) {
        mystery_box.probability_tier = probability_tier;
    } else {
        return Err(ErrorCode::ProbabilityTierNotFound.into());
    }

    // Increase order
    player_account.order_nonce += 1;

    msg!("Recipient account: {}", recipient.to_account_info().key());
    msg!(
        "Purchased mystery box account: {}",
        mystery_box.to_account_info().key()
    );
    Ok(())
}
// pub fn consume_randomness(
//     ctx: Context<ConsumeRandomness>,
//     _order_nonce: u64, // Used in instruction
//     // bank_escrow_bump: u8,
//     // total_lamports: u64,
//     result: Vec<u8>,
// ) -> Result<()> {
//     msg!("Randomness received: {:?}", result);
//     //msg!("order_nonce: {:?}", order_nonce);
//     msg!(
//         "fighter mint allowance:  {:?}",
//         ctx.accounts.mystery_box.fighter_mint_allowance
//     );
//     msg!(
//         "booster mint allowance:  {:?}",
//         ctx.accounts.mystery_box.booster_mint_allowance
//     );
//     let mystery_box = &mut ctx.accounts.mystery_box;
//     mystery_box.randomness = Some(result);
//
//     // // let signer = &ctx.accounts.signer.key();
//     // let bank = &mut ctx.accounts.bank;
//     // let bank_escrow = &mut ctx.accounts.bank_escrow;
//     // let bank_escrow_balance = bank_escrow.lamports();
//     // msg!("lamport balance: {}",bank_escrow_balance);
//     // msg!("total lamports to send: {}",total_lamports);
//     // if bank_escrow_balance < total_lamports {
//     //      msg!(
//     //          "Insufficient funds: required {}, available {}.",
//     //          total_lamports,
//     //          bank_escrow_balance
//     //      );
//     //      return Err(ErrorCode::InsufficientFunds.into());
//     //  }
//     // bank_escrow.sub_lamports(total_lamports)?;
//     // bank.add_lamports(total_lamports)?;
//
//     // ctx.accounts.bank_escrow.sub_lamports(total_lamports)?;
//     // ctx.accounts.bank.add_lamports(total_lamports)?;
//     // **ctx.accounts.bank_escrow.to_account_info().try_borrow_mut_lamports()? -= total_lamports;
//     // **ctx.accounts.bank.to_account_info().try_borrow_mut_lamports()? += total_lamports;
//     //
//     //Calculate the minimum balance required to remain rent-exempt
//     // let rent_exempt_balance = Rent::get()?.minimum_balance(bank_escrow.data_len());
//     // // Calculate the maximum amount that can be safely withdrawn while keeping the account rent-exempt
//     // let withdrawable_balance = bank_escrow_balance.saturating_sub(rent_exempt_balance);
//
//     // Construct the transfer instruction
//
//     // let transfer_instruction = system_instruction::transfer(
//     //     &bank_escrow.key(),
//     //     &bank.key(),
//     //     // Withdraw the full balance
//     //     total_lamports, // Amount in lamports to transfer
//     // );
//     //
//     // let bank_escrow_seeds = [MY_APP_PREFIX, BANK, signer.as_ref(), &[bank_escrow_bump]];
//
//     //Perform the transfer
//     // invoke_signed(
//     //     &transfer_instruction,
//     //     &[
//     //         bank_escrow.to_account_info(),
//     //         bank.to_account_info(),
//     //         ctx.accounts.system_program.to_account_info(),
//     //     ],
//     //     &[&bank_escrow_seeds],
//     // )?;
//
//     Ok(())
// }
pub fn create_new_event(
    ctx: Context<CreateEvent>,
    start_date: i64,
    end_date: i64,
    tournament_type: TournamentType,
    rank_reward: Vec<RankReward>,
) -> Result<()> {
    let program = &mut ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

    // Create event account and set data
    let create_event = &mut ctx.accounts.event;
    create_event.fight_card_nonce = 0_u8;
    create_event.start_date = start_date;
    create_event.end_date = end_date;
    create_event.tournament_type = tournament_type;
    create_event.rank_rewards = rank_reward;
    create_event.rank_nonce = 0_u64;
    create_event.nonce = program.event_nonce;

    emit!(EventCreated {
        event_id: program.event_nonce
    });

    // Increment event counter
    program.event_nonce += 1_u64;

    Ok(())
}

pub fn update_event(
    ctx: Context<UpdateEvent>,
    start_date: i64,
    end_date: i64,
    tournament_type: TournamentType,
    rank_reward: Vec<RankReward>,
) -> Result<()> {
    let program = &ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

    let update_event = &mut ctx.accounts.event;
    update_event.tournament_type = tournament_type;
    update_event.start_date = start_date;
    update_event.end_date = end_date;
    update_event.rank_rewards = rank_reward;

    emit!(EventUpdated {
        event_id: program.event_nonce
    });

    Ok(())
}

pub fn create_new_fight_card(ctx: Context<CreateFightCard>, params: FightCardData) -> Result<()> {
    let program = &ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

    let fight_card = &mut ctx.accounts.fight_card;
    let event = &mut ctx.accounts.event;
    set_fight_card_properties(fight_card, &params, Some(event.fight_card_nonce));
    fight_card.event_nonce_tracker = event.nonce;

    event.fight_card_nonce = event.fight_card_nonce.checked_add(1_u8).unwrap();

    // emit!(FightCardCreated {
    //     fight_card_id: fight_card.id
    // });

    Ok(())
}

pub fn update_fight_card(ctx: Context<UpdateFightCard>, params: FightCardData) -> Result<()> {
    let program = &ctx.accounts.program;
    verify_equality(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

    let fight_card = &mut ctx.accounts.fight_card;
    set_fight_card_properties(fight_card, &params, None);

    // emit!(FightCardUpdated {
    //     fight_card_id: fight_card.id
    // });

    Ok(())
}

// pub fn mint_nft_from_game_asset(
//     ctx: Context<MintNftFromGameAsset>,
//     //requests: Vec<PurchaseRequest>,
// ) -> Result<()> {
//     let program = &mut ctx.accounts.program;
//
//     let metadata_program = &ctx.accounts.metadata_program.to_account_info();
//     let metadata = &ctx.accounts.metadata.to_account_info();
//     let token_record = &ctx.accounts.token_record.to_account_info();
//     let minter = &ctx.accounts.minter.to_account_info();
//     let token_owner = &ctx.accounts.creator.to_account_info();
//     let master_edition = &ctx.accounts.master_edition.to_account_info();
//     let mint_authority = &ctx.accounts.mint_authority.to_account_info();
//     let token_account = &ctx.accounts.token_account.to_account_info();
//
//     let sysvar = &ctx.accounts.sysvar_instructions.to_account_info();
//     let spl_token_program = &ctx.accounts.token_program.to_account_info();
//
//     // Energy
//     let energy_metadata = &ctx.accounts.energy_metadata.to_account_info();
//     let energy_master_edition = &ctx.accounts.energy_master_edition.to_account_info();
//     let energy_minter = &ctx.accounts.energy_minter.to_account_info();
//
//     let mut binding_create = CreateV1CpiBuilder::new(&metadata_program);
//
//     let create_cpi = binding_create
//         .metadata(&metadata)
//         .mint(&minter, false)
//         .authority(&mint_authority)
//         .payer(&token_owner)
//         .update_authority(&mint_authority, true)
//         .master_edition(Some(&ctx.accounts.master_edition))
//         .collection(Collection {
//             key: energy_minter.key(),
//             verified: false,
//         })
//         .system_program(&ctx.accounts.system_program)
//         .sysvar_instructions(&sysvar)
//         .spl_token_program(Some(&spl_token_program))
//         .token_standard(TokenStandard::ProgrammableNonFungible)
//         .name("Energy booster".to_string())
//         .uri("https://battleboosters.com".to_string())
//         .seller_fee_basis_points(SELLER_FEE)
//         .is_mutable(true)
//         .print_supply(PrintSupply::Zero);
//
//     let mut binding = MintV1CpiBuilder::new(metadata_program);
//     let mint_cpi = binding
//         .token(token_account)
//         .token_owner(Some(token_owner))
//         .metadata(metadata)
//         .master_edition(Some(master_edition))
//         .token_record(Some(token_record))
//         .mint(minter)
//         .payer(token_owner)
//         .authority(mint_authority)
//         .system_program(&ctx.accounts.system_program)
//         .sysvar_instructions(&ctx.accounts.sysvar_instructions)
//         .spl_token_program(&ctx.accounts.token_program)
//         .spl_ata_program(&ctx.accounts.associated_token_program)
//         .amount(1);
//
//     let mut binding_verify = VerifyCollectionV1CpiBuilder::new(&metadata_program);
//     let create_cpi_verify = binding_verify
//         .collection_mint(&energy_minter)
//         .authority(&mint_authority)
//         .metadata(&metadata)
//         .collection_metadata(Some(&energy_metadata))
//         .collection_master_edition(Some(&energy_master_edition))
//         .sysvar_instructions(&ctx.accounts.sysvar_instructions)
//         .system_program(&ctx.accounts.system_program);
//
//     let authority_seeds = [
//         MY_APP_PREFIX,
//         MINT_AUTHORITY,
//         &[program.authority_bump.clone()],
//     ];
//
//     create_cpi.invoke_signed(&[&authority_seeds])?;
//     mint_cpi.invoke_signed(&[&authority_seeds])?;
//     create_cpi_verify.invoke_signed(&[&authority_seeds])?;
//
//     // for request in &requests {
//     //     match request.nft_type {
//     //         NftType::Booster => {
//     //             // update the quantity of booster mint allowance
//     //             collector_pack.booster_mint_allowance = collector_pack.booster_mint_allowance.checked_sub(1).unwrap();
//     //
//     //
//     //             let metadata_program = &ctx.accounts.metadata_program.to_account_info();
//     //             let energy_metadata = &ctx.accounts.energy_metadata.to_account_info();
//     //             let minter = &ctx.accounts.energy_minter.to_account_info();
//     //             let token_owner = &ctx.accounts.creator.to_account_info();
//     //             let master_edition = &ctx.accounts.energy_master_edition.to_account_info();
//     //             let mint_authority = &ctx.accounts.mint_authority.to_account_info();
//     //
//     //             let energy_token_account = &ctx.accounts.energy_token_account.to_account_info();
//     //
//     //             let mut binding = MintV1CpiBuilder::new(metadata_program);
//     //             let mint_cpi = binding
//     //                 .token(energy_token_account)
//     //                 .token_owner(Some(token_owner))
//     //                 .metadata(energy_metadata)
//     //                 .master_edition(Some(master_edition))
//     //                 .mint(minter)
//     //                 .payer(token_owner)
//     //                 .authority(mint_authority)
//     //                 .system_program(&ctx.accounts.system_program)
//     //                 .sysvar_instructions(&ctx.accounts.sysvar_instructions)
//     //                 .spl_token_program(&ctx.accounts.token_program)
//     //                 .spl_ata_program(&ctx.accounts.associated_token_program)
//     //                 .amount(1);
//     //
//     //             let authority_seeds = [
//     //                 MY_APP_PREFIX,
//     //                 MINT_AUTHORITY,
//     //                 &[program.authority_bump.clone()],
//     //             ];
//     //
//     //             mint_cpi.invoke_signed(&[&authority_seeds])?;
//     //
//     //         }
//     //         NftType::FighterPack => {
//     //             // update the quantity of fighter mint allowance
//     //             collector_pack.fighter_mint_allowance = collector_pack.fighter_mint_allowance.checked_sub(1).unwrap();
//     //         }
//     //     }
//     // }
//
//     program.mintable_game_asset_nonce = program.mintable_game_asset_nonce.checked_add(1).unwrap();
//     Ok(())
// }

// /*
//    TODO: Refund mintable game asset in case the fight Card have been canceled
// */
pub fn refund_mintable_game_asset(
    ctx: Context<RefundMintableGameAsset>,
    points_game_asset_link_nonce: u64,
    shield_game_asset_link_nonce: u64,
    _player_pubkey: Pubkey,
) -> Result<()> {
    let player_account = &mut ctx.accounts.player_account;
    let fight_card = &ctx.accounts.fight_card;
    let fight_card_link = &mut ctx.accounts.fight_card_link;
    let fighter_asset = &mut ctx.accounts.fighter_asset;
    // let fighter_link = &mut ctx.accounts.fighter_link;
    let points_booster_asset = &mut ctx.accounts.points_booster_asset;
    let points_booster_link = &mut ctx.accounts.points_booster_link;
    let shield_booster_asset = &mut ctx.accounts.shield_booster_asset;
    let shield_booster_link = &mut ctx.accounts.shield_booster_link;

    // Check if the fightCard have been resolved first and make sure it is a no contest before continuing
    if let Some(result) = &fight_card.result {
        require!(
            result == &FightCardResult::NoContest,
            ErrorCode::EventStillRunning
        );
    } else {
        return Err(ErrorCode::EventStillRunning.into());
    }
    // Ensure the fight card link is not consumed yet
    require!(
        fight_card_link.is_consumed == false,
        ErrorCode::ConsumedAlready
    );
    fight_card_link.is_consumed = true;

    // process_game_asset(
    //     fighter_game_asset_link_nonce,
    //     &mut player_account.player_game_asset_link_nonce,
    //     fighter_link.is_free,
    //     &mut fighter_link.nonce)?;

    if let Some(fighter_used_pubkey) = fight_card_link.fighter_used {
        require!(
            fighter_used_pubkey == fighter_asset.to_account_info().key(),
            ErrorCode::Unauthorized
        );
        // Unlock the game asset to allow use again
        fighter_asset.is_locked = false;
    }

    // if let Some(fighter_link_used_pubkey) = fight_card_link.fighter_link_used {
    //     require!(
    //         fighter_link_used_pubkey == fighter_link.to_account_info().key(),
    //         ErrorCode::Unauthorized
    //     );
    //     require!(fighter_link.is_free == false, ErrorCode::Unauthorized);
    //     require!(
    //         fighter_link.mintable_game_asset_nonce_tracker == fighter_asset.nonce,
    //         ErrorCode::Unauthorized
    //     );
    //     require!(
    //         fighter_link.mintable_game_asset_pubkey == fighter_asset.to_account_info().key(),
    //         ErrorCode::Unauthorized
    //     );
    //     //require!(fighter_link.is_free, ErrorCode::NotFreePDA);
    //     // fighter_link.mintable_game_asset_nonce_tracker = fighter_asset.nonce;
    //     // fighter_link.mintable_game_asset_pubkey = fighter_asset.to_account_info().key();
    //     // fighter_link.is_free = false;
    // }

    if let Some(points_booster_used_pubkey) = fight_card_link.points_booster_used {
        let points_booster_asset_unwrapped = points_booster_asset
            .as_mut()
            .ok_or(ErrorCode::Unauthorized)?;
        let points_booster_link_unwrapped = points_booster_link
            .as_mut()
            .ok_or(ErrorCode::Unauthorized)?;

        require!(
            points_booster_used_pubkey == points_booster_asset_unwrapped.to_account_info().key(),
            ErrorCode::Unauthorized
        );
        points_booster_asset_unwrapped.is_locked = false;
        points_booster_asset_unwrapped.is_burned = false;
        points_booster_asset_unwrapped.owner =
            Some(points_booster_link_unwrapped.to_account_info().key());
        process_game_asset(
            points_game_asset_link_nonce,
            &mut player_account.player_game_asset_link_nonce,
            points_booster_link_unwrapped.is_free,
            &mut points_booster_link_unwrapped.nonce,
        )?;
        points_booster_link_unwrapped.mintable_game_asset_nonce_tracker =
            points_booster_asset_unwrapped.nonce;
    }

    if let Some(shield_booster_used_pubkey) = fight_card_link.shield_booster_used {
        let shield_booster_asset_unwrapped = shield_booster_asset
            .as_mut()
            .ok_or(ErrorCode::Unauthorized)?;
        let shield_booster_link_unwrapped = shield_booster_link
            .as_mut()
            .ok_or(ErrorCode::Unauthorized)?;

        require!(
            shield_booster_used_pubkey == shield_booster_asset_unwrapped.to_account_info().key(),
            ErrorCode::Unauthorized
        );
        shield_booster_asset_unwrapped.is_locked = false;
        shield_booster_asset_unwrapped.is_burned = false;
        shield_booster_asset_unwrapped.owner =
            Some(shield_booster_link_unwrapped.to_account_info().key());
        process_game_asset(
            shield_game_asset_link_nonce,
            &mut player_account.player_game_asset_link_nonce,
            shield_booster_link_unwrapped.is_free,
            &mut shield_booster_link_unwrapped.nonce,
        )?;
        shield_booster_link_unwrapped.mintable_game_asset_nonce_tracker =
            shield_booster_asset_unwrapped.nonce;
    }

    Ok(())
}

pub fn create_mintable_game_asset(
    ctx: Context<CreateMintableGameAsset>,
    mintable_game_asset_link_nonce: u64, // used in instruction
    player_pubkey: Pubkey,               // used in instruction
    request: OpenRequest,
) -> Result<()> {
    let clock: Clock = Clock::get()?;
    let program = &mut ctx.accounts.program;
    let mystery_box = &mut ctx.accounts.mystery_box;
    let mintable_game_asset_link = &mut ctx.accounts.mintable_game_asset_link;
    let mintable_game_asset = &mut ctx.accounts.mintable_game_asset;
    let player_account = &mut ctx.accounts.player_account;

    require!(
        mystery_box.randomness_account.is_some(),
        ErrorCode::RandomnessIsNone
    );
    require!(
        mystery_box.randomness_account.unwrap().key() == ctx.accounts.randomness_account_data.key(),
        ErrorCode::RandomnessNotMatchingProvided
    );
    require!(
        mintable_game_asset_link_nonce <= player_account.player_game_asset_link_nonce,
        ErrorCode::WrongPlayerGameAssetLinkNonce
    );

    if mintable_game_asset_link_nonce < player_account.player_game_asset_link_nonce {
        require!(mintable_game_asset_link.is_free, ErrorCode::NotFreePDA);
    } else {
        // Save the nonce for seeds easier re-generation
        mintable_game_asset_link.nonce = player_account.player_game_asset_link_nonce;
        // increase the player game asset link nonce for the next game asset generation
        player_account.player_game_asset_link_nonce += 1;
    }
    // Used for testing in local-net without depending on external services
    let randomness = match program.env {
        Env::Prod => {
            let randomness_data =
                RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow())
                    .unwrap();
            // call the switchboard on-demand get_value function to get the revealed random value
            let randomness = randomness_data
                .get_value(&clock)
                .map_err(|_| ErrorCode::RandomnessNotResolved)?;
            randomness
        }
        Env::Dev => {
            let timestamp_bytes = clock.unix_timestamp.to_le_bytes();
            // Initialize a Sha256 hasher
            let mut hasher = Sha256::new();

            // Feed the timestamp bytes into the hasher
            hasher.update(&timestamp_bytes);

            // Optionally add more entropy here
            // hasher.update(&additional_entropy);

            // Finalize the hash to get a 32-byte output
            let hash_result = hasher.finalize();
            let randomness: [u8; 32] = hash_result.into();

            randomness
        }
    };

    msg!("Randomness {:?}", randomness);

    match request.nft_type {
        NftType::Booster => {
            require!(
                mystery_box.booster_mint_allowance >= 1,
                ErrorCode::NotEnoughAllowance
            );
            let rarity = &ctx
                .accounts
                .rarity
                .clone()
                .ok_or(ErrorCode::RarityAccountRequired)?;
            // call the switchboard on-demand parse function to get the randomness data

            // let randomness = mystery_box
            //     .randomness
            //     .clone()
            //     .ok_or(ErrorCode::RandomnessUnavailable)?;
            let public_key_bytes = player_pubkey.to_bytes();

            let combined_allowance = &mystery_box.booster_mint_allowance
                + &mystery_box.fighter_mint_allowance
                + &mystery_box.champions_pass_mint_allowance;
            let nonce_byte = (combined_allowance % 256) as u8;

            let rng_seed = create_rng_seed(&randomness, &public_key_bytes, &nonce_byte, None);
            let random_number = ((rng_seed % 100) + 1) as u8;
            msg!("Random number{}", random_number);

            let rng_seed_1 =
                create_rng_seed(&randomness, &public_key_bytes, &nonce_byte, Some(1_u8));
            let random_booster_type = (&rng_seed % 2) as usize;
            let booster_type = BoosterType::from_index(random_booster_type);
            let rarity_index = find_rarity(mystery_box.probability_tier.clone(), random_number);
            // Get the random booster type
            let mut rarity_booster_found: Option<&RarityBooster> = None;
            if let Some(booster) = booster_type.clone() {
                match booster {
                    BoosterType::Points => {
                        rarity_booster_found = rarity
                            .points_booster
                            .iter()
                            .find(|r| r.matches_index(rarity_index));
                    }
                    BoosterType::Shield => {
                        rarity_booster_found = rarity
                            .shield_booster
                            .iter()
                            .find(|r| r.matches_index(rarity_index));
                    } // BoosterType::Energy => {
                      //     rarity_booster_found = rarity
                      //         .energy_booster
                      //         .iter()
                      //         .find(|r| r.matches_index(rarity_index));
                      // }
                }
            }

            msg!("rarity index {:?}", rarity_index);
            msg!("rarity found {:?}", rarity_booster_found);

            if let Some(rarity_booster) = rarity_booster_found.clone() {
                let scaled_random_number = match rarity_booster {
                    RarityBooster::Common { value } => find_scaled_rarity(value, rng_seed_1),
                    RarityBooster::Uncommon { value } => find_scaled_rarity(value, rng_seed_1),
                    RarityBooster::Rare { value } => find_scaled_rarity(value, rng_seed_1),
                    RarityBooster::Epic { value } => find_scaled_rarity(value, rng_seed_1),
                    RarityBooster::Legendary { value } => find_scaled_rarity(value, rng_seed_1),
                };

                /*
                   TODO: Probably add `Uses` and `Used` to allows multiple use before burn?
                   TODO: Should we add the creation date time?
                */
                let attributes = vec![
                    Attribute {
                        trait_type: "Booster Type".to_string(),
                        value: booster_type.unwrap().to_string(),
                    },
                    Attribute {
                        trait_type: "Rarity".to_string(),
                        value: rarity_booster_found.unwrap().to_string(),
                    },
                    Attribute {
                        trait_type: "Value".to_string(),
                        value: scaled_random_number.to_string(),
                    },
                ];

                mintable_game_asset.metadata = create_nft_metadata(
                    "Booster".to_string(),
                    "test".to_string(),
                    None,
                    None,
                    Some(format!(
                        "{}/{}",
                        METADATA_OFF_CHAIN_URI,
                        mintable_game_asset.key().to_string()
                    )),
                    attributes,
                );

                msg!("{:?}", mintable_game_asset.metadata);
                msg!("Scaled random number: {}", scaled_random_number);
            } else {
                // Handle case where no matching rarity was found
                return Err(ErrorCode::NoMatchingRarityFound.into());
            }

            mystery_box.booster_mint_allowance = mystery_box
                .booster_mint_allowance
                .checked_sub(1)
                .unwrap_or(0);
        }
        NftType::Fighter => {
            require!(
                mystery_box.fighter_mint_allowance >= 1,
                ErrorCode::NotEnoughAllowance
            );
            let rarity = &ctx
                .accounts
                .rarity
                .clone()
                .ok_or(ErrorCode::RarityAccountRequired)?;

            let public_key_bytes = player_pubkey.to_bytes();
            let combined_allowance = &mystery_box.booster_mint_allowance
                + &mystery_box.fighter_mint_allowance
                + &mystery_box.champions_pass_mint_allowance;
            let nonce_byte = (combined_allowance % 256) as u8;

            let rng_seed = create_rng_seed(&randomness, &public_key_bytes, &nonce_byte, None);
            let random_number = ((rng_seed % 100) + 1) as u8;
            msg!("Random number{}", random_number);

            let rng_seed_1 =
                create_rng_seed(&randomness, &public_key_bytes, &nonce_byte, Some(1_u8));
            let rng_seed_2 =
                create_rng_seed(&randomness, &public_key_bytes, &nonce_byte, Some(2_u8));
            let rng_seed_3 =
                create_rng_seed(&randomness, &public_key_bytes, &nonce_byte, Some(3_u8));
            msg!("seed 1 {}", rng_seed_1);
            msg!("seed 2 {}", rng_seed_2);
            msg!("seed 3 {}", rng_seed_3);

            let random_fighter_type = (rng_seed.clone() % 8) as usize;
            let fighter_type = FighterType::from_index(random_fighter_type);
            let rarity_index = find_rarity(mystery_box.probability_tier.clone(), random_number);
            let rarity_fighter_found = rarity
                .fighter
                .iter()
                .find(|r| r.matches_index(rarity_index));

            if let Some(rarity_fighter) = rarity_fighter_found.clone() {
                let (
                    //scaled_random_number_energy,
                    scaled_random_number_power,
                    scaled_random_number_lifespan,
                ) = match rarity_fighter {
                    RarityFighter::Common {
                        //energy,
                        power,
                        lifespan,
                    } => {
                        //msg!("Common value min: {} and max: {}  ", value.min, value.max);
                        (
                            //find_scaled_rarity(energy, rng_seed_1),
                            find_scaled_rarity(power, rng_seed_2),
                            find_scaled_rarity(lifespan, rng_seed_3),
                        )
                    }
                    RarityFighter::Uncommon {
                        //energy,
                        power,
                        lifespan,
                    } => (
                        //find_scaled_rarity(energy, rng_seed_1),
                        find_scaled_rarity(power, rng_seed_2),
                        find_scaled_rarity(lifespan, rng_seed_3),
                    ),
                    RarityFighter::Rare {
                        //energy,
                        power,
                        lifespan,
                    } => (
                        //find_scaled_rarity(energy, rng_seed_1),
                        find_scaled_rarity(power, rng_seed_2),
                        find_scaled_rarity(lifespan, rng_seed_3),
                    ),
                    RarityFighter::Epic {
                        //energy,
                        power,
                        lifespan,
                    } => (
                        //find_scaled_rarity(energy, rng_seed_1),
                        find_scaled_rarity(power, rng_seed_2),
                        find_scaled_rarity(lifespan, rng_seed_3),
                    ),
                    RarityFighter::Legendary {
                        //energy,
                        power,
                        lifespan,
                    } => (
                        //find_scaled_rarity(energy, rng_seed_1),
                        find_scaled_rarity(power, rng_seed_2),
                        find_scaled_rarity(lifespan, rng_seed_3),
                    ),
                };

                /*
                   TODO: Probably add `Used` to track usage?
                   TODO: Should we add the creation date time?
                */
                let attributes = vec![
                    Attribute {
                        trait_type: "Fighter Type".to_string(),
                        value: fighter_type.unwrap().to_string(),
                    },
                    Attribute {
                        trait_type: "Rarity".to_string(),
                        value: rarity_fighter_found.unwrap().to_string(),
                    },
                    // Attribute {
                    //     trait_type: "Energy".to_string(),
                    //     value: scaled_random_number_energy.to_string(),
                    // },
                    Attribute {
                        trait_type: "Power".to_string(),
                        value: scaled_random_number_power.to_string(),
                    },
                    Attribute {
                        trait_type: "Maximum Lifespan".to_string(),
                        value: scaled_random_number_lifespan.to_string(),
                    },
                    Attribute {
                        trait_type: "Lifespan".to_string(),
                        value: scaled_random_number_lifespan.to_string(),
                    },
                ];

                mintable_game_asset.metadata = create_nft_metadata(
                    "Fighter".to_string(),
                    "test".to_string(),
                    None,
                    None,
                    Some(format!(
                        "{}/{}",
                        METADATA_OFF_CHAIN_URI,
                        mintable_game_asset.key().to_string()
                    )),
                    attributes,
                );
                mystery_box.fighter_mint_allowance = mystery_box
                    .fighter_mint_allowance
                    .checked_sub(1)
                    .unwrap_or(0);
                msg!("{:?}", mintable_game_asset.metadata);
            } else {
                // Handle case where no matching rarity was found
                return Err(ErrorCode::NoMatchingRarityFound.into());
            }
        }
        NftType::ChampionsPass => {
            require!(
                mystery_box.champions_pass_mint_allowance >= 1,
                ErrorCode::NotEnoughAllowance
            );
            /*
               TODO: Create a mintable asset for champion's pass
               TODO: Should we add the creation date time?
            */
            let attributes = vec![
                Attribute {
                    trait_type: "Uses".to_string(),
                    value: "1".to_string(),
                },
                Attribute {
                    trait_type: "Used".to_string(),
                    value: "0".to_string(),
                },
            ];

            mintable_game_asset.metadata = create_nft_metadata(
                "Champion's Pass".to_string(),
                "test".to_string(),
                None,
                None,
                Some(format!(
                    "{}/{}",
                    METADATA_OFF_CHAIN_URI,
                    mintable_game_asset.key().to_string()
                )),
                attributes,
            );
            mystery_box.champions_pass_mint_allowance = mystery_box
                .champions_pass_mint_allowance
                .checked_sub(1)
                .unwrap_or(0);
        }
    }

    // Establishes a linkage between the `player_game_asset_link` PDA
    // and the nonce of the `mintable_game_asset`,
    // facilitating indexed seed access.
    mintable_game_asset_link.mintable_game_asset_nonce_tracker =
        program.mintable_game_asset_nonce.clone();
    // Save the Public key of the `mintable_game_asset` PDA for direct linkage
    mintable_game_asset_link.mintable_game_asset_pubkey =
        mintable_game_asset.to_account_info().key();
    // The mintable asset is not free
    mintable_game_asset_link.is_free = false;

    // Save the nonce for seeds easier re-generation
    mintable_game_asset.nonce = program.mintable_game_asset_nonce;
    // Assigns the player_game_asset_link as the owner of the mintable asset,
    // ensuring ownership until the user decides to mint it.
    mintable_game_asset.owner = Some(mintable_game_asset_link.to_account_info().key());
    // Updates the global state to track the current amount of created `mintable_game_asset` instances.
    program.mintable_game_asset_nonce += 1;

    msg!(
        "Used mystery box account: {}",
        mystery_box.to_account_info().key()
    );

    Ok(())
}

pub fn join_fight_card(
    ctx: Context<JoinFightCard>,
    fighter_color_side: FighterColorSide,
) -> Result<()> {
    let clock = Clock::get()?;
    let current_blockchain_timestamp = clock.unix_timestamp;
    let event = &ctx.accounts.event;
    let fight_card = &ctx.accounts.fight_card;
    let fight_card_link = &mut ctx.accounts.fight_card_link;
    let event_link = &mut ctx.accounts.event_link;

    match event.tournament_type {
        TournamentType::MainCard => {
            require!(
                event_link.champions_pass_pubkey.is_some(),
                ErrorCode::MissingChampionsPassAsset
            )
        }
        _ => {}
    }

    require!(
        !fight_card_link.is_initialized,
        ErrorCode::AlreadyInitialized
    );

    // Make sure the event have not started before joining the fight
    require!(
        event.start_date > current_blockchain_timestamp,
        ErrorCode::EventAlreadyStarted
    );

    // Game assets
    process_game_asset_for_action(
        Some(&mut ctx.accounts.fighter_asset),
        Some(&mut ctx.accounts.fighter_link),
        false,
    )?;
    process_and_verify_game_asset_type(Some(&ctx.accounts.fighter_asset), fight_card_link)?;

    process_game_asset_for_action(
        ctx.accounts.shield_booster_asset.as_mut(),
        ctx.accounts.shield_booster_link.as_mut(),
        true,
    )?;
    process_and_verify_game_asset_type(
        ctx.accounts.shield_booster_asset.as_ref(),
        fight_card_link,
    )?;
    process_game_asset_for_action(
        ctx.accounts.points_booster_asset.as_mut(),
        ctx.accounts.points_booster_link.as_mut(),
        true,
    )?;
    process_and_verify_game_asset_type(
        ctx.accounts.points_booster_asset.as_ref(),
        fight_card_link,
    )?;

    require!(
        fight_card_link.fighter_used.is_some() && fight_card_link.fighter_nonce_tracker.is_some(),
        ErrorCode::FightCardLinkedToGameAsset
    );

    fight_card_link.fighter_link_used = Some(ctx.accounts.fighter_link.to_account_info().key());
    fight_card_link.fighter_link_used_nonce_tracker = Some(ctx.accounts.fighter_link.nonce);
    fight_card_link.fight_card_pubkey = fight_card.to_account_info().key();
    fight_card_link.fight_card_nonce_tracker = fight_card.nonce;
    fight_card_link.fighter_color_side = fighter_color_side;
    fight_card_link.is_consumed = false;
    fight_card_link.is_initialized = true;

    msg!("Join event: {}", event.to_account_info().key());
    msg!("Join fight card: {}", fight_card.to_account_info().key());
    msg!(
        "With fight card link: {}",
        fight_card_link.to_account_info().key()
    );

    Ok(())
}

pub fn collect_rewards(ctx: Context<CollectRewards>) -> Result<()> {
    let clock = Clock::get().unwrap();
    let current_blockchain_timestamp = clock.unix_timestamp;
    let program = &ctx.accounts.program;
    let rank = &mut ctx.accounts.rank;
    let event = &mut ctx.accounts.event;
    let player_account = &mut ctx.accounts.player_account;
    let mystery_box = &mut ctx.accounts.mystery_box;
    let rarity = &ctx.accounts.rarity;
    let bank = &mut ctx.accounts.bank;
    let feed_account = &ctx.accounts.price_feed;
    let signer = &ctx.accounts.signer;

    verify_equality(&rank.player_account.key(), &signer.to_account_info().key())?;
    require!(
        event.end_date < current_blockchain_timestamp,
        ErrorCode::EventStillRunning
    );
    require!(!rank.is_consumed, ErrorCode::ConsumedAlready);
    require!(rank.total_points.is_some(), ErrorCode::RankPointsIsNone);

    if let Some(player_rank) = rank.rank {
        let rank_rewards = event.rank_rewards.iter().find(|rank_reward| {
            rank_reward.start_rank <= player_rank
                && match rank_reward.end_rank {
                    Some(end_rank) => player_rank <= end_rank,
                    None => true, // If end_rank is None, any rank above start_rank qualifies
                }
        });
        if let Some(reward) = rank_rewards {
            // Found a rank reward that matches the player's rank
            // Do something with the reward
            match event.tournament_type {
                TournamentType::MainCard => {
                    if let Some(probability_tier) = rarity.get_probability_by_tier(TierType::Tier1)
                    {
                        mystery_box.probability_tier = probability_tier;
                    }
                }
                TournamentType::Prelims => {
                    if let Some(probability_tier) = rarity.get_probability_by_tier(TierType::Tier2)
                    {
                        mystery_box.probability_tier = probability_tier;
                    }
                }
                TournamentType::EarlyPrelims => {
                    if let Some(probability_tier) = rarity.get_probability_by_tier(TierType::Tier3)
                    {
                        mystery_box.probability_tier = probability_tier;
                    }
                }
            }
            mystery_box.randomness_account = None;
            mystery_box.booster_mint_allowance = reward.booster_amount as u64;
            mystery_box.fighter_mint_allowance = reward.fighter_amount as u64;
            mystery_box.champions_pass_mint_allowance = reward.champions_pass_amount as u64;
            mystery_box.nonce = player_account.order_nonce;

            // let feed = PullFeedAccountData::parse(feed_account)
            //     .map_err(|_| error!(ErrorCode::FeedUnreachable))?;
            // let price = feed.get_value(&Clock::get()?, STALENESS_THRESHOLD, 1, true)
            //     .map_err(|_| error!(ErrorCode::StaleFeed))?;
            // let val = price.to_f64().ok_or(ErrorCode::InvalidOperation)?;

            let feed_id: [u8; 32] = get_feed_id_from_hex(FEED_HEX)?;
            let price = feed_account.get_price_no_older_than(
                &Clock::get()?,
                STALENESS_THRESHOLD,
                &feed_id,
            )?;
            let val = (price.price as f64) * 10f64.powi(price.exponent);

            let sol_per_usd = 1.0 / val;
            let total_sol = reward.prize_amount * sol_per_usd;
            let total_lamports = (total_sol * LAMPORTS_PER_SOL as f64).round() as u64;

            let bank_balance = bank.lamports();
            msg!("bank balance before: {}", bank.lamports());
            if bank_balance < total_lamports {
                msg!(
                    "Insufficient funds: required {}, available {}.",
                    total_lamports,
                    bank_balance
                );
                return Err(ErrorCode::InsufficientFunds.into());
            }

            let transfer_instruction = system_instruction::transfer(
                &bank.key(),
                &signer.key(),
                total_lamports, // Amount in lamports to transfer
            );

            let bank_seeds = [MY_APP_PREFIX, BANK, &[program.bank_bump]];

            //Perform the transfer
            invoke_signed(
                &transfer_instruction,
                &[
                    bank.to_account_info(),
                    signer.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[&bank_seeds],
            )?;

            msg!("bank balance now: {}", bank.lamports());
            msg!("Collect prize SOL: {}", total_sol);
            msg!("Collect prize booster amount: {}", reward.booster_amount);
            msg!("Collect prize fighter amount: {}", reward.fighter_amount);
            msg!(
                "Collect prize champion's pass amount: {}",
                reward.champions_pass_amount
            );
            msg!(
                "Collect mystery box: {}",
                mystery_box.to_account_info().key()
            );
            msg!(
                "Collect for player account: {}",
                player_account.to_account_info().key()
            );
            msg!(
                "Collect for player account creator: {}",
                signer.to_account_info().key()
            );
            msg!(
                "Collect for event account: {}",
                event.to_account_info().key()
            );
            // Increase order_nonce
            player_account.order_nonce += 1;
        }
    } else {
        return Err(ErrorCode::RankIsNone.into());
    }

    rank.is_consumed = true;
    Ok(())
}

pub fn admin_update_rank(ctx: Context<UpdateRank>, ranking: u64) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let program = &ctx.accounts.program;
    let rank = &mut ctx.accounts.rank;

    verify_equality(&program.admin_pubkey, &signer.to_account_info().key())?;
    rank.rank = Some(ranking);
    msg!("new rank : {:?}", rank.rank);

    Ok(())
}

pub fn determine_ranking_points(
    ctx: Context<DetermineRankingPoints>,
    _fighter_type: FighterType, // Used in instruction
) -> Result<()> {
    let clock = Clock::get().unwrap();
    let current_blockchain_timestamp = clock.unix_timestamp;
    let event = &ctx.accounts.event;

    let fight_card = &ctx.accounts.fight_card;
    let fight_card_link = &mut ctx.accounts.fight_card_link;
    let rank = &mut ctx.accounts.rank;
    let fighter_mintable_game_asset = &mut ctx.accounts.fighter_asset;
    let fighter_mintable_game_asset_link = &mut ctx.accounts.fighter_asset_link;
    let points_mintable_game_asset = &mut ctx.accounts.points_booster_asset;
    let shield_mintable_game_asset = &mut ctx.accounts.shield_booster_asset;
    let fighter_base = &ctx.accounts.fighter_base;

    verify_equality(
        &fighter_mintable_game_asset.to_account_info().key(),
        &fighter_mintable_game_asset_link.mintable_game_asset_pubkey,
    )?;
    require!(
        !event.end_date < current_blockchain_timestamp,
        ErrorCode::EventStillRunning
    );
    require!(!fight_card_link.is_consumed, ErrorCode::ConsumedAlready);

    if let Some(attribute) = fighter_mintable_game_asset
        .metadata
        .attributes
        .iter()
        .find(|x| x.trait_type == "Fighter Type")
    {
        let fighter_type = FighterType::from_name(&attribute.value).unwrap();
        require!(
            fighter_base.fighter_type == fighter_type,
            ErrorCode::Unauthorized
        );
    }

    let mut points_multiplier = 1_u32;
    // Get the points metadata
    if let Some(points) = points_mintable_game_asset {
        points_multiplier = asset_metadata_value(&points.metadata, "value".to_string());
    }

    let power_multiplier =
        asset_metadata_value(&fighter_mintable_game_asset.metadata, "Power".to_string());
    let power_multiplier_float = power_multiplier as f32 / 100.0;
    msg!("power_multiplier: {:?}", power_multiplier);
    msg!("power_multiplier_float: {:?}", power_multiplier_float);
    let fighter_blue = fight_card.fighter_blue.clone().unwrap();
    let fighter_red = fight_card.fighter_red.clone().unwrap();

    let (points_value, damage_value) = match fight_card_link.fighter_color_side {
        /*
           TODO: Add Points in case the fight is finished before duration or the round end
               Extra points multiplier for big perf
               Add grappler bonus for finishing in submission or penality if finishing in striker
               Add it also for stricker
               Do a ratio attempted and landed to calculate the points ratio

        */
        FighterColorSide::FighterBlue => metrics_calculation(
            &fighter_blue,
            &fighter_red,
            &fighter_base.fight_metrics,
            power_multiplier_float,
        ),
        FighterColorSide::FighterRed => metrics_calculation(
            &fighter_red,
            &fighter_blue,
            &fighter_base.fight_metrics,
            power_multiplier_float,
        ),
    };

    let mut shield_multiplier = 1_u32;
    // Get the shield metadata
    if let Some(shield) = shield_mintable_game_asset {
        shield_multiplier = asset_metadata_value(&shield.metadata, "Value".to_string());
    }

    // reduce lifespan
    if let Some(lifespan_attribute) = fighter_mintable_game_asset
        .metadata
        .attributes
        .iter_mut()
        .find(|x| x.trait_type == "Lifespan")
    {
        if let Ok(life_span_value) = lifespan_attribute.value.parse::<u32>() {
            // Properly calculate lifespan value with shield multiplier
            let shield_effect = (shield_multiplier as f32 / 100.0) * life_span_value as f32;
            let life_span_value_plus_shield = life_span_value + shield_effect.round() as u32;

            msg!(
                "Lifespan: {}, Lifespan with shield: {}",
                life_span_value,
                life_span_value_plus_shield
            );

            let life_span_after_damage = life_span_value_plus_shield
                .checked_sub(damage_value)
                .unwrap_or(0);

            msg!("Lifespan after damage: {}", life_span_after_damage);

            lifespan_attribute.value = life_span_after_damage.to_string();

            if life_span_after_damage == 0 {
                fighter_mintable_game_asset.is_burned = true;
                fighter_mintable_game_asset.owner = None;
                fighter_mintable_game_asset_link.is_free = true;
            }
        } else {
            return Err(ErrorCode::FailedToParseValue.into());
        }
    };

    let new_points_value = if points_multiplier > 0 {
        ((points_multiplier as f32 / 100.0) * points_value as f32).round() as u32
    } else {
        points_value
    };

    // Set new point value
    rank.total_points = Some(new_points_value as u64);

    fighter_mintable_game_asset.is_locked = false;
    fight_card_link.is_consumed = true;

    msg!("Determine ranking points: {}", points_value);
    msg!("Determine damage value: {}", damage_value);
    msg!(
        "Determine ranking points with fighter game asset: {}",
        fighter_mintable_game_asset.to_account_info().key()
    );
    msg!(
        "Determine ranking points with fighter game asset link: {}",
        fighter_mintable_game_asset_link.to_account_info().key()
    );

    Ok(())
}
