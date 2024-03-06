use anchor_lang::prelude::*;
use anchor_spl::token::MintTo;
mod constants;
mod errors;
mod events;
mod processor;
mod state;
mod types;
mod utils;

use crate::constants::*;
use crate::events::*;
use crate::state::{
    collector_pack::*, create_spl_nft::*, event::*, fight_card::*, player::*, program::*,
    rarity::*, switchboard_callback::*, transaction_escrow::*,
};

use crate::types::*;
use crate::utils::*;

use errors::ErrorCode;

use mpl_token_metadata::instructions::{
    BurnCpiBuilder, CreateMetadataAccountV3, CreateV1, CreateV1Builder, CreateV1CpiBuilder,
    MintV1CpiBuilder, TransferV1Cpi, TransferV1CpiAccounts, TransferV1InstructionArgs,
    VerifyCollectionV1CpiBuilder,
};

use mpl_token_metadata::types::{PrintSupply, TokenStandard};

use anchor_lang::solana_program::program::{invoke, invoke_signed};
use solana_randomness_service::ID as SolanaRandomnessServiceID;
use switchboard_solana::utils::get_ixn_discriminator;

declare_id!("32pkjHX1E7VKhY79YgAynriPojk8Sf4RC5d4pGWDLpsj");

#[program]
pub mod battleboosters {
    use super::*;
    use crate::state::player::InitializePlayer;
    use crate::state::rarity::InitializeRarity;
    use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
    use anchor_lang::solana_program::system_instruction;
    use mpl_token_metadata::types::{Collection, CollectionDetails, DataV2};
    use solana_randomness_service::TransactionOptions;
    use std::ops::Add;

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

    pub fn initialize_player(
        ctx: Context<InitializePlayer>,
        player_pubkey: Pubkey, /* Used in initialization */
    ) -> Result<()> {
        //let player_inventory = &mut ctx.accounts.inventory;
        let player_account = &mut ctx.accounts.player_account;
        require!(
            !player_account.is_initialized,
            ErrorCode::AlreadyInitialized
        );
        //
        // player_inventory.fighter_mint_allowance = 0;
        // player_inventory.booster_mint_allowance = 0;
        // player_inventory.is_initialized = true;

        player_account.order_nonce = 0;
        player_account.player_game_asset_link_nonce = 0;
        player_account.is_initialized = true;

        msg!("Player Initialized");

        Ok(())
    }

    pub fn create_nft_collection(
        ctx: Context<CreateSplNft>,
        collection_id: CollectionType, /* Used in initialization */
        collection_name: String,
        symbol: String,
        uri: String,
        fees: u16,
    ) -> Result<()> {
        let program = &ctx.accounts.program;
        only_admin(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

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

    pub fn purchase_nfts(
        ctx: Context<TransactionEscrow>,
        bank_escrow_bump: u8,
        requests: Vec<PurchaseRequest>,
    ) -> Result<()> {
        let program = &ctx.accounts.program;
        let feed = &ctx.accounts.price_feed.load()?;
        let collector_pack = &mut ctx.accounts.collector_pack;
        let player_account = &mut ctx.accounts.player_account;
        let bank = &ctx.accounts.bank;
        let bank_escrow = &ctx.accounts.bank_escrow;

        // get result
        let val: f64 = feed.get_result()?.try_into()?;
        // check whether the feed has been updated in the last 300 seconds
        feed.check_staleness(Clock::get()?.unix_timestamp, STALENESS_THRESHOLD)
            .map_err(|_| error!(ErrorCode::StaleFeed))?;

        let sol_per_usd = 1.0 / val;
        let mut total_usd = 0;
        for request in &requests {
            match request.nft_type {
                NftType::Booster => {
                    // update the quantity of fighter mint allowance
                    collector_pack.fighter_mint_allowance += request.quantity.clone();
                    total_usd += request
                        .quantity
                        .checked_mul(program.booster_price.clone())
                        .unwrap();
                }
                NftType::FighterPack => {
                    // update the quantity of fighter mint allowance
                    collector_pack.booster_mint_allowance += request.quantity.clone();
                    total_usd += request
                        .quantity
                        .checked_mul(program.fighter_pack_price.clone())
                        .unwrap();
                }
            }
        }
        // Increase order
        player_account.order_nonce += 1;

        require!(total_usd > 0, ErrorCode::InsufficientAmount);

        let total_sol = total_usd as f64 * sol_per_usd;
        let total_lamports = (total_sol * LAMPORTS_PER_SOL as f64).round() as u64;
        let bank_escrow_balance = bank_escrow.lamports();

        if bank_escrow_balance < total_lamports {
            msg!(
                "Insufficient funds: required {}, available {}.",
                total_lamports,
                bank_escrow_balance
            );
            return Err(ErrorCode::InsufficientFunds.into());
        }

        let mut ix_data = get_ixn_discriminator("consume_randomness").to_vec();
        ix_data.extend_from_slice(&[bank_escrow_bump.clone()]);
        ix_data.extend_from_slice(&total_lamports.to_le_bytes());

        solana_randomness_service::cpi::simple_randomness_v1(
            CpiContext::new(
                ctx.accounts.randomness_service.to_account_info(),
                solana_randomness_service::cpi::accounts::SimpleRandomnessV1Request {
                    request: ctx.accounts.randomness_request.to_account_info(),
                    escrow: ctx.accounts.randomness_escrow.to_account_info(),
                    state: ctx.accounts.randomness_state.to_account_info(),
                    mint: ctx.accounts.randomness_mint.to_account_info(),
                    payer: ctx.accounts.signer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    associated_token_program: ctx
                        .accounts
                        .associated_token_program
                        .to_account_info(),
                },
            ),
            8, // Request 8 bytes of randomness
            solana_randomness_service::Callback {
                program_id: ID,
                accounts: vec![
                    AccountMeta::new_readonly(ctx.accounts.randomness_state.key(), true).into(),
                    AccountMeta::new_readonly(ctx.accounts.randomness_request.key(), false).into(),
                ],
                ix_data, // TODO: hardcode this discriminator [190,217,49,162,99,26,73,234]
            },
            Some(TransactionOptions {
                compute_units: Some(1_000_000),
                compute_unit_price: Some(200),
            }),
        )?;

        Ok(())
    }

    pub fn consume_randomness(
        ctx: Context<ConsumeRandomness>,
        bank_escrow_bump: u8,
        total_lamports: u64,
        result: Vec<u8>,
    ) -> Result<()> {
        msg!("Randomness received: {:?}", result);
        let signer = &ctx.accounts.signer.key();
        let collector_pack = &mut ctx.accounts.collector_pack;
        let bank = &ctx.accounts.bank;
        let bank_escrow = &ctx.accounts.bank_escrow;

        collector_pack.randomness = Some(result);

        let bank_escrow_balance = bank_escrow.lamports();
        if bank_escrow_balance < total_lamports {
            msg!(
                "Insufficient funds: required {}, available {}.",
                total_lamports,
                bank_escrow_balance
            );
            return Err(ErrorCode::InsufficientFunds.into());
        }

        //Calculate the minimum balance required to remain rent-exempt
        // let rent_exempt_balance = Rent::get()?.minimum_balance(bank_escrow.data_len());
        // // Calculate the maximum amount that can be safely withdrawn while keeping the account rent-exempt
        // let withdrawable_balance = bank_escrow_balance.saturating_sub(rent_exempt_balance);

        // Construct the transfer instruction
        let transfer_instruction = system_instruction::transfer(
            &bank_escrow.key(),
            &bank.key(),
            // Withdraw the full balance
            total_lamports, // Amount in lamports to transfer
        );

        let bank_escrow_seeds = [MY_APP_PREFIX, BANK, signer.as_ref(), &[bank_escrow_bump]];

        // Perform the transfer
        invoke_signed(
            &transfer_instruction,
            &[
                bank_escrow.to_account_info(),
                bank.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&bank_escrow_seeds],
        )?;

        Ok(())
    }

    // TODO: REMOVE BEFORE MAINNET LAUNCH
    /// ONLY FOR TEST PURPOSE
    pub fn test_gift_collector_pack(ctx: Context<TransactionTest>) -> Result<()> {
        let collector_pack = &mut ctx.accounts.collector_pack;

        collector_pack.randomness = Some(vec![12, 23, 34, 34, 54, 34, 34, 23]);
        collector_pack.booster_mint_allowance = 3;
        collector_pack.fighter_mint_allowance = 1;
        Ok(())
    }

    pub fn generate_random_mintable_game_asset(
        ctx: Context<GenerateRandomNftPreMint>,
        player_game_asset_link_nonce: u64, // used on instruction
        request: OpenRequest,
    ) -> Result<()> {
        let program = &mut ctx.accounts.program;
        let collector_pack = &mut ctx.accounts.collector_pack;
        let rarity = &ctx.accounts.rarity;
        let player_game_asset_link = &mut ctx.accounts.player_game_asset_link;
        let mintable_game_asset = &mut ctx.accounts.mintable_game_asset;
        let player_account = &mut ctx.accounts.player_account;

        let signer = &ctx.accounts.signer;

        require!(
            player_game_asset_link_nonce <= player_account.player_game_asset_link_nonce,
            ErrorCode::WrongPlayerGameAssetLinkNonce
        );

        if player_game_asset_link_nonce < player_account.player_game_asset_link_nonce {
            require!(player_game_asset_link.is_free, ErrorCode::NotFreePDA);
        } else {
            // increase the player game asset link nonce for the next game asset generation
            player_account.player_game_asset_link_nonce += 1;
        }

        let randomness = collector_pack
            .randomness
            .clone()
            .ok_or(ErrorCode::RandomnessUnavailable)?;
        let public_key_bytes = signer.key().to_bytes();
        let nonce_byte = (collector_pack.booster_mint_allowance.clone() & 0xFF) as u8;

        let rng_seed = u64::from_le_bytes([
            randomness[0].clone(),
            randomness[1].clone(),
            randomness[2].clone(),
            randomness[3].clone(),
            public_key_bytes[0].clone(),
            public_key_bytes[1].clone(),
            public_key_bytes[2].clone(),
            nonce_byte,
        ]);
        let random_number = ((xorshift64(rng_seed.clone()) % 100) + 1) as u8;

        msg!("Random number{}", random_number);

        match request.nft_type {
            NftType::Booster => {
                require!(
                    collector_pack.booster_mint_allowance >= 1,
                    ErrorCode::Unauthorized
                );

                let random_booster_type = (xorshift64(rng_seed.clone()) % 3) as usize;
                let booster_type = BoosterType::from_index(random_booster_type);
                let rarity_index = find_rarity(rarity.booster_probabilities.clone(), random_number);
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
                        }
                        BoosterType::Energy => {
                            rarity_booster_found = rarity
                                .energy_booster
                                .iter()
                                .find(|r| r.matches_index(rarity_index));
                        }
                    }
                }

                msg!(" rarity index {:?}", rarity_index);
                msg!(" rarity found {:?}", rarity_booster_found);

                if let Some(rarity_booster) = rarity_booster_found.clone() {
                    match rarity_booster {
                        RarityBooster::Common { value } => {
                            msg!(" Common value min: {} and max: {}  ", value.min, value.max);
                        }
                        RarityBooster::Uncommon { value } => {
                            msg!(
                                " Uncommon value min: {} and max: {}  ",
                                value.min,
                                value.max
                            );
                            let scaled_random_number = find_scaled_rarity(value, rng_seed);

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
                                format!(
                                    "{}/{}",
                                    METADATA_OFF_CHAIN_URI,
                                    mintable_game_asset.key().to_string()
                                ),
                                None,
                                None,
                                attributes,
                            );

                            msg!("{:?}", mintable_game_asset.metadata);
                            msg!("Scaled random number: {}", scaled_random_number);
                        }
                        RarityBooster::Rare { value } => {}
                        RarityBooster::Epic { value } => {}
                        RarityBooster::Legendary { value } => {
                            println!("Min: {}, Max: {}", value.min, value.max);
                            // Use value.min and value.max as needed
                        }
                    }
                } else {
                    // Handle case where no matching rarity was found
                    return Err(ErrorCode::NoMatchingRarityFound.into());
                }

                collector_pack.booster_mint_allowance = collector_pack
                    .booster_mint_allowance
                    .checked_sub(1)
                    .unwrap();

                // Update global state for mintable game asset initialization
                program.mintable_game_asset_nonce =
                    program.mintable_game_asset_nonce.checked_add(1).unwrap();

                msg!("GOOD");
            }
            NftType::FighterPack => {
                require!(
                    collector_pack.fighter_mint_allowance >= 1,
                    ErrorCode::Unauthorized
                );
                msg!("GOOD");
            }
        }

        // let mut x = 0;
        // while x < collector_pack.booster_mint_allowance {
        //     // Your loop body here
        //
        //     x += 1;
        // }

        Ok(())
    }

    pub fn mint_collector_pack(
        ctx: Context<MintCollectorPack>,
        //requests: Vec<PurchaseRequest>,
    ) -> Result<()> {
        let program = &mut ctx.accounts.program;

        let metadata_program = &ctx.accounts.metadata_program.to_account_info();
        let metadata = &ctx.accounts.metadata.to_account_info();
        let token_record = &ctx.accounts.token_record.to_account_info();
        let minter = &ctx.accounts.minter.to_account_info();
        let token_owner = &ctx.accounts.creator.to_account_info();
        let master_edition = &ctx.accounts.master_edition.to_account_info();
        let mint_authority = &ctx.accounts.mint_authority.to_account_info();
        let token_account = &ctx.accounts.token_account.to_account_info();

        let sysvar = &ctx.accounts.sysvar_instructions.to_account_info();
        let spl_token_program = &ctx.accounts.token_program.to_account_info();

        // Energy
        let energy_metadata = &ctx.accounts.energy_metadata.to_account_info();
        let energy_master_edition = &ctx.accounts.energy_master_edition.to_account_info();
        let energy_minter = &ctx.accounts.energy_minter.to_account_info();

        let mut binding_create = CreateV1CpiBuilder::new(&metadata_program);

        let create_cpi = binding_create
            .metadata(&metadata)
            .mint(&minter, false)
            .authority(&mint_authority)
            .payer(&token_owner)
            .update_authority(&mint_authority, true)
            .master_edition(Some(&ctx.accounts.master_edition))
            .collection(Collection {
                key: energy_minter.key(),
                verified: false,
            })
            .system_program(&ctx.accounts.system_program)
            .sysvar_instructions(&sysvar)
            .spl_token_program(Some(&spl_token_program))
            .token_standard(TokenStandard::ProgrammableNonFungible)
            .name("Energy booster".to_string())
            .uri("https://battleboosters.com".to_string())
            .seller_fee_basis_points(500)
            .is_mutable(true)
            .print_supply(PrintSupply::Zero);

        let mut binding = MintV1CpiBuilder::new(metadata_program);
        let mint_cpi = binding
            .token(token_account)
            .token_owner(Some(token_owner))
            .metadata(metadata)
            .master_edition(Some(master_edition))
            .token_record(Some(token_record))
            .mint(minter)
            .payer(token_owner)
            .authority(mint_authority)
            .system_program(&ctx.accounts.system_program)
            .sysvar_instructions(&ctx.accounts.sysvar_instructions)
            .spl_token_program(&ctx.accounts.token_program)
            .spl_ata_program(&ctx.accounts.associated_token_program)
            .amount(1);

        let mut binding_verify = VerifyCollectionV1CpiBuilder::new(&metadata_program);
        let create_cpi_verify = binding_verify
            .collection_mint(&energy_minter)
            .authority(&mint_authority)
            .metadata(&metadata)
            .collection_metadata(Some(&energy_metadata))
            .collection_master_edition(Some(&energy_master_edition))
            .sysvar_instructions(&ctx.accounts.sysvar_instructions)
            .system_program(&ctx.accounts.system_program);

        let authority_seeds = [
            MY_APP_PREFIX,
            MINT_AUTHORITY,
            &[program.authority_bump.clone()],
        ];

        create_cpi.invoke_signed(&[&authority_seeds])?;
        mint_cpi.invoke_signed(&[&authority_seeds])?;
        create_cpi_verify.invoke_signed(&[&authority_seeds])?;

        // for request in &requests {
        //     match request.nft_type {
        //         NftType::Booster => {
        //             // update the quantity of booster mint allowance
        //             collector_pack.booster_mint_allowance = collector_pack.booster_mint_allowance.checked_sub(1).unwrap();
        //
        //
        //             let metadata_program = &ctx.accounts.metadata_program.to_account_info();
        //             let energy_metadata = &ctx.accounts.energy_metadata.to_account_info();
        //             let minter = &ctx.accounts.energy_minter.to_account_info();
        //             let token_owner = &ctx.accounts.creator.to_account_info();
        //             let master_edition = &ctx.accounts.energy_master_edition.to_account_info();
        //             let mint_authority = &ctx.accounts.mint_authority.to_account_info();
        //
        //             let energy_token_account = &ctx.accounts.energy_token_account.to_account_info();
        //
        //             let mut binding = MintV1CpiBuilder::new(metadata_program);
        //             let mint_cpi = binding
        //                 .token(energy_token_account)
        //                 .token_owner(Some(token_owner))
        //                 .metadata(energy_metadata)
        //                 .master_edition(Some(master_edition))
        //                 .mint(minter)
        //                 .payer(token_owner)
        //                 .authority(mint_authority)
        //                 .system_program(&ctx.accounts.system_program)
        //                 .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        //                 .spl_token_program(&ctx.accounts.token_program)
        //                 .spl_ata_program(&ctx.accounts.associated_token_program)
        //                 .amount(1);
        //
        //             let authority_seeds = [
        //                 MY_APP_PREFIX,
        //                 MINT_AUTHORITY,
        //                 &[program.authority_bump.clone()],
        //             ];
        //
        //             mint_cpi.invoke_signed(&[&authority_seeds])?;
        //
        //         }
        //         NftType::FighterPack => {
        //             // update the quantity of fighter mint allowance
        //             collector_pack.fighter_mint_allowance = collector_pack.fighter_mint_allowance.checked_sub(1).unwrap();
        //         }
        //     }
        // }

        program.mintable_game_asset_nonce =
            program.mintable_game_asset_nonce.checked_add(1).unwrap();
        Ok(())
    }

    pub fn create_new_event(
        ctx: Context<CreateEvent>,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        let program = &mut ctx.accounts.program;
        only_admin(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

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
        event_id: u64,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        let program = &ctx.accounts.program;
        only_admin(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

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
        params: FightCardData,
    ) -> Result<()> {
        let program = &ctx.accounts.program;
        only_admin(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

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
        fight_card_id: u8,
        params: FightCardData,
    ) -> Result<()> {
        let program = &ctx.accounts.program;
        only_admin(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

        let fight_card = &mut ctx.accounts.fight_card;
        set_fight_card_properties(fight_card, &params);

        emit!(FightCardUpdated {
            fight_card_id: fight_card.id
        });

        Ok(())
    }

    /*
       @params: Tournament id, Card type (Main card, prelims, early prelims),
       TODO: Register to tournament

    */

    /*
       TODO: Claim event reward
    */

    /*
        TODO: Purchase NFT
            - Integration with Pyth Oracle or Switchboard for price feeds Sol/Usd conversion
    */

    /*
       TODO: Deposit NFT to my collection
    */

    /*
       TODO: Withdraw NFT from my collection
    */
}
