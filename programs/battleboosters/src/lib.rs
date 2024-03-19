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

declare_id!("A92RCbQNmTNcGBQS63gvPXz1pdaVcSQjo4LZBQuswzia");

#[program]
pub mod battleboosters {
    use super::*;
    use crate::state::player::InitializePlayer;
    use crate::state::rarity::InitializeRarity;
    use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
    use anchor_lang::solana_program::system_instruction;
    use mpl_token_metadata::types::{Collection, CollectionDetails, DataV2};
    use solana_program::address_lookup_table::state::ProgramState::LookupTable;
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
        processor::initialize(
            ctx,
            authority_bump,
            bank_bump,
            admin_pubkey,
            nft_fighter_pack_price,
            booster_price,
            fighter_pack_amount,
        )
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
        processor::initialize_rarity(
            ctx,
            fighter,
            energy_booster,
            shield_booster,
            points_booster,
            fighter_probabilities,
            booster_probabilities,
        )
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
                NftType::Fighter => {
                    // update the quantity of fighter mint allowance
                    collector_pack.booster_mint_allowance += request
                        .quantity
                        .checked_mul(program.fighter_pack_amount.clone() as u64)
                        .unwrap();

                    total_usd += request
                        .quantity
                        .checked_mul(program.fighter_pack_price.clone())
                        .unwrap();
                }
                _ => return Err(error!(ErrorCode::UnsupportedNftType)),
            }
        }

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
        ix_data.extend_from_slice(&player_account.order_nonce.to_le_bytes());
        ix_data.extend_from_slice(&ctx.accounts.recipient.to_account_info().key().as_ref());
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
                    AccountMeta::new_readonly(
                        ctx.accounts.randomness_state.to_account_info().key(),
                        true,
                    )
                    .into(),
                    AccountMeta::new_readonly(
                        ctx.accounts.randomness_request.to_account_info().key(),
                        false,
                    )
                    .into(),
                    //AccountMeta::new(ctx.accounts.program.to_account_info().key(), false).into(),
                    AccountMeta::new_readonly(
                        ctx.accounts.recipient.to_account_info().key(),
                        false,
                    )
                    .into(),
                    // AccountMeta::new(player_account.to_account_info().key(), false).into(),
                    //AccountMeta::new(ctx.accounts.collector_pack.to_account_info().key(), false).into(),
                    AccountMeta::new(collector_pack.to_account_info().key(), false).into(),
                ],
                ix_data, // TODO: hardcode this discriminator [190,217,49,162,99,26,73,234]
            },
            Some(TransactionOptions {
                compute_units: Some(1_000_000),
                compute_unit_price: Some(200),
            }),
        )?;

        // Set the collector pack to default `champion_s_pass_mint_allowance`
        collector_pack.champions_pass_mint_allowance = 0;
        // Increase order
        player_account.order_nonce += 1;

        Ok(())
    }

    pub fn consume_randomness(
        ctx: Context<ConsumeRandomness>,
        order_nonce: u64,
        recipient_pubkey: Pubkey,
        bank_escrow_bump: u8,
        total_lamports: u64,
        result: Vec<u8>,
    ) -> Result<()> {
        msg!("Randomness received: {:?}", result);
        msg!("order_nonce: {:?}", order_nonce);
        msg!("pubkjey recipient: {:?}", recipient_pubkey);
        msg!(
            "mint allowance:  {:?}",
            ctx.accounts.collector_pack.fighter_mint_allowance
        );
        let collector_pack = &mut ctx.accounts.collector_pack;
        collector_pack.randomness = Some(result);
        // let x = &mut ctx.accounts.collector_pack;
        // x.randomness = Some(result);

        // // 1. PDA Derivation
        // let (collector_pack_pda, bump_seed) = Pubkey::find_program_address(
        //     &[MY_APP_PREFIX, COLLECTOR, ctx.accounts.signer.key().as_ref(), order_nonce.to_le_bytes().as_ref()],
        //     ctx.program_id);
        // RpcClient::new(collector_pack_pda);

        // let signer = &ctx.accounts.signer.key();
        // let collector_pack = &mut ctx.accounts.collector_pack;
        // let bank = &ctx.accounts.bank;
        // let bank_escrow = &ctx.accounts.bank_escrow;
        //
        // collector_pack.randomness = Some(result);
        //
        // let bank_escrow_balance = bank_escrow.lamports();
        // if bank_escrow_balance < total_lamports {
        //     msg!(
        //         "Insufficient funds: required {}, available {}.",
        //         total_lamports,
        //         bank_escrow_balance
        //     );
        //     return Err(ErrorCode::InsufficientFunds.into());
        // }
        //
        // //Calculate the minimum balance required to remain rent-exempt
        // // let rent_exempt_balance = Rent::get()?.minimum_balance(bank_escrow.data_len());
        // // // Calculate the maximum amount that can be safely withdrawn while keeping the account rent-exempt
        // // let withdrawable_balance = bank_escrow_balance.saturating_sub(rent_exempt_balance);
        //
        // // Construct the transfer instruction
        // let transfer_instruction = system_instruction::transfer(
        //     &bank_escrow.key(),
        //     &bank.key(),
        //     // Withdraw the full balance
        //     total_lamports, // Amount in lamports to transfer
        // );
        //
        // let bank_escrow_seeds = [MY_APP_PREFIX, BANK, signer.as_ref(), &[bank_escrow_bump]];
        //
        // // Perform the transfer
        // invoke_signed(
        //     &transfer_instruction,
        //     &[
        //         bank_escrow.to_account_info(),
        //         bank.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        //     &[&bank_escrow_seeds],
        // )?;

        Ok(())
    }

    // TODO: REMOVE BEFORE MAINNET LAUNCH
    /// ONLY FOR TEST PURPOSE
    pub fn admin_airdrop_collector_pack(
        ctx: Context<TransactionTest>,
        booster_mint_alowance: u64,
        fighter_mint_allowance: u64,
        champions_pass_mint_allowance: u64,
    ) -> Result<()> {
        verify_equality(
            &ctx.accounts.signer.key(),
            &ctx.accounts.program.admin_pubkey,
        )?;
        let collector_pack = &mut ctx.accounts.collector_pack;

        collector_pack.randomness = Some(vec![12, 23, 34, 34, 54, 74, 94, 23]);
        collector_pack.booster_mint_allowance = booster_mint_alowance;
        collector_pack.fighter_mint_allowance = fighter_mint_allowance;
        collector_pack.champions_pass_mint_allowance = champions_pass_mint_allowance;
        Ok(())
    }

    pub fn generate_mintable_game_asset(
        ctx: Context<GenerateNftPreMint>,
        player_game_asset_link_nonce: u64, // used on instruction
        request: OpenRequest,
    ) -> Result<()> {
        let program = &mut ctx.accounts.program;
        let collector_pack = &mut ctx.accounts.collector_pack;
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

        match request.nft_type {
            NftType::Booster => {
                require!(
                    collector_pack.booster_mint_allowance >= 1,
                    ErrorCode::NotEnoughAllowance
                );
                let rarity = &ctx
                    .accounts
                    .rarity
                    .clone()
                    .ok_or(ErrorCode::RarityAccountRequired)?;
                let randomness = collector_pack
                    .randomness
                    .clone()
                    .ok_or(ErrorCode::RandomnessUnavailable)?;
                let public_key_bytes = signer.key().to_bytes();

                let combined_allowance = &collector_pack.booster_mint_allowance
                    + &collector_pack.fighter_mint_allowance
                    + &collector_pack.champions_pass_mint_allowance;
                let nonce_byte = (combined_allowance % 256) as u8;

                let rng_seed = create_rng_seed(&randomness, &public_key_bytes, &nonce_byte, None);
                let random_number = ((rng_seed % 100) + 1) as u8;
                msg!("Random number{}", random_number);

                let rng_seed_1 =
                    create_rng_seed(&randomness, &public_key_bytes, &nonce_byte, Some(1_u8));
                let random_booster_type = (&rng_seed % 3) as usize;
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
                } else {
                    // Handle case where no matching rarity was found
                    return Err(ErrorCode::NoMatchingRarityFound.into());
                }

                collector_pack.booster_mint_allowance = collector_pack
                    .booster_mint_allowance
                    .checked_sub(1)
                    .unwrap();
                msg!("GOOD");
            }
            NftType::Fighter => {
                require!(
                    collector_pack.fighter_mint_allowance >= 1,
                    ErrorCode::NotEnoughAllowance
                );
                let rarity = &ctx
                    .accounts
                    .rarity
                    .clone()
                    .ok_or(ErrorCode::RarityAccountRequired)?;
                let randomness = collector_pack
                    .randomness
                    .clone()
                    .ok_or(ErrorCode::RandomnessUnavailable)?;
                let public_key_bytes = signer.key().to_bytes();

                let combined_allowance = &collector_pack.booster_mint_allowance
                    + &collector_pack.fighter_mint_allowance
                    + &collector_pack.champions_pass_mint_allowance;
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
                let rarity_index = find_rarity(rarity.fighter_probabilities.clone(), random_number);
                let rarity_fighter_found = rarity
                    .fighter
                    .iter()
                    .find(|r| r.matches_index(rarity_index));

                if let Some(rarity_fighter) = rarity_fighter_found.clone() {
                    let (
                        scaled_random_number_energy,
                        scaled_random_number_power,
                        scaled_random_number_lifespan,
                    ) = match rarity_fighter {
                        RarityFighter::Common {
                            energy,
                            power,
                            lifespan,
                        } => {
                            //msg!("Common value min: {} and max: {}  ", value.min, value.max);
                            (
                                find_scaled_rarity(energy, rng_seed_1),
                                find_scaled_rarity(power, rng_seed_2),
                                find_scaled_rarity(lifespan, rng_seed_3),
                            )
                        }
                        RarityFighter::Uncommon {
                            energy,
                            power,
                            lifespan,
                        } => (
                            find_scaled_rarity(energy, rng_seed_1),
                            find_scaled_rarity(power, rng_seed_2),
                            find_scaled_rarity(lifespan, rng_seed_3),
                        ),
                        RarityFighter::Rare {
                            energy,
                            power,
                            lifespan,
                        } => (
                            find_scaled_rarity(energy, rng_seed_1),
                            find_scaled_rarity(power, rng_seed_2),
                            find_scaled_rarity(lifespan, rng_seed_3),
                        ),
                        RarityFighter::Epic {
                            energy,
                            power,
                            lifespan,
                        } => (
                            find_scaled_rarity(energy, rng_seed_1),
                            find_scaled_rarity(power, rng_seed_2),
                            find_scaled_rarity(lifespan, rng_seed_3),
                        ),
                        RarityFighter::Legendary {
                            energy,
                            power,
                            lifespan,
                        } => (
                            find_scaled_rarity(energy, rng_seed_1),
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
                        Attribute {
                            trait_type: "Energy".to_string(),
                            value: scaled_random_number_energy.to_string(),
                        },
                        Attribute {
                            trait_type: "Power".to_string(),
                            value: scaled_random_number_power.to_string(),
                        },
                        Attribute {
                            trait_type: "Lifespan".to_string(),
                            value: scaled_random_number_lifespan.to_string(),
                        },
                    ];

                    mintable_game_asset.metadata = create_nft_metadata(
                        "Fighter".to_string(),
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
                    collector_pack.fighter_mint_allowance = collector_pack
                        .fighter_mint_allowance
                        .checked_sub(1)
                        .unwrap();

                    msg!("{:?}", mintable_game_asset.metadata);
                } else {
                    // Handle case where no matching rarity was found
                    return Err(ErrorCode::NoMatchingRarityFound.into());
                }

                msg!("GOOD");
            }
            NftType::ChampionsPass => {
                require!(
                    collector_pack.champions_pass_mint_allowance >= 1,
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
                    format!(
                        "{}/{}",
                        METADATA_OFF_CHAIN_URI,
                        mintable_game_asset.key().to_string()
                    ),
                    None,
                    None,
                    attributes,
                );
                collector_pack.champions_pass_mint_allowance = collector_pack
                    .champions_pass_mint_allowance
                    .checked_sub(1)
                    .unwrap();
            }
        }

        // Establishes a linkage between the `player_game_asset_link` PDA
        // and the nonce of the `mintable_game_asset`,
        // facilitating indexed seed access.
        player_game_asset_link.mintable_game_asset_nonce_tracker =
            program.mintable_game_asset_nonce.clone();
        // Save the Public key of the `mintable_game_asset` PDA for direct linkage
        player_game_asset_link.mintable_game_asset_pubkey =
            mintable_game_asset.to_account_info().key();
        // Updates the global state to track the current amount of created `mintable_game_asset` instances.
        program.mintable_game_asset_nonce += 1;
        // Assigns the player_game_asset_link as the owner of the mintable asset,
        // ensuring ownership until the user decides to mint it.
        mintable_game_asset.owner = Some(player_game_asset_link.to_account_info().key());

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
        tournament_type: TournamentType,
    ) -> Result<()> {
        processor::create_new_event(ctx, start_date, end_date, tournament_type)
    }

    pub fn update_event(
        ctx: Context<UpdateEvent>,
        event_nonce: u64,
        start_date: i64,
        end_date: i64,
        tournament_type: TournamentType,
    ) -> Result<()> {
        processor::update_event(ctx, event_nonce, start_date, end_date, tournament_type)
    }

    pub fn create_new_fight_card(
        ctx: Context<CreateFightCard>,
        event_nonce: u64,
        params: FightCardData,
    ) -> Result<()> {
        processor::create_new_fight_card(ctx, event_nonce, params)
    }

    pub fn update_fight_card(
        ctx: Context<UpdateFightCard>,
        event_nonce: u64,
        fight_card_id: u8,
        params: FightCardData,
    ) -> Result<()> {
        processor::update_fight_card(ctx, event_nonce, fight_card_id, params)
    }

    pub fn initialize_event_link(
        ctx: Context<InitializeEventLink>,
        event_nonce: u64,
    ) -> Result<()> {
        let event_link = &mut ctx.accounts.event_link;
        let event = &ctx.accounts.event;
        require!(!event_link.is_initialized, ErrorCode::AlreadyInitialized);

        event_link.event_pubkey = event.to_account_info().key();
        event_link.event_nonce_tracker = event_nonce;
        event_link.champions_pass_pubkey = None;
        event_link.champions_pass_nonce_tracker = None;
        event_link.is_initialized = true;

        Ok(())
    }

    pub fn join_fight_card(
        ctx: Context<JoinFightCard>,
        event_nonce: u64,                        // Used in instruction
        fight_card_nonce: u8,                    // Used in instruction
        fighter_asset_nonce: u64,                // Used in instruction
        energy_booster_asset_nonce: Option<u64>, // Used in instruction
        shield_booster_asset_nonce: Option<u64>, // Used in instruction
        points_booster_asset_nonce: Option<u64>, // Used in instruction
        champions_pass_asset_nonce: Option<u64>, // Used in instruction
        fighter_link_nonce: u64,                 // Used in instruction
        energy_booster_link_nonce: Option<u64>,  // Used in instruction
        shield_booster_link_nonce: Option<u64>,  // Used in instruction
        points_booster_link_nonce: Option<u64>,  // Used in instruction
        champions_pass_link_nonce: Option<u64>,  // Used in instruction
        fighter_color_side: FighterColorSide,
    ) -> Result<()> {
        let clock = Clock::get().unwrap();
        let current_blockchain_timestamp = clock.unix_timestamp;

        let signer = &ctx.accounts.signer.to_account_info();
        let event = &ctx.accounts.event;
        let fight_card = &ctx.accounts.fight_card;
        let fight_card_link = &mut ctx.accounts.fight_card_link;
        let event_link = &mut ctx.accounts.event_link;
        let champions_pass_asset = &ctx.accounts.champions_pass_asset;

        require!(
            !fight_card_link.is_initialized,
            ErrorCode::AlreadyInitialized
        );

        // Make sure the event have not started before joining the fight
        require!(
            event.start_date > current_blockchain_timestamp,
            ErrorCode::EventAlreadyStarted
        );

        /*
           TODO: Check the game assets are what they pretend to be
            (Fighter, Energy, Shield, Points and Champion's pass)

            TODO: - Move it to utils `process_game_asset_for_action` method and check save it and check it is None before saving.

        */
        // Game assets

        process_game_asset_for_action(
            Some(&mut ctx.accounts.fighter_asset),
            Some(&mut ctx.accounts.fighter_link),
            &signer.key(),
            false,
        )?;
        process_and_verify_game_asset_type(
            Some(&ctx.accounts.fighter_asset),
            fight_card_link,
            event_link,
            None,
            Some(fighter_asset_nonce),
        )?;

        process_game_asset_for_action(
            ctx.accounts.energy_booster_asset.as_mut(),
            ctx.accounts.energy_booster_link.as_mut(),
            &signer.key(),
            true,
        )?;
        process_and_verify_game_asset_type(
            ctx.accounts.energy_booster_asset.as_ref(),
            fight_card_link,
            event_link,
            None,
            energy_booster_asset_nonce,
        )?;
        process_game_asset_for_action(
            ctx.accounts.shield_booster_asset.as_mut(),
            ctx.accounts.shield_booster_link.as_mut(),
            &signer.key(),
            true,
        )?;
        process_and_verify_game_asset_type(
            ctx.accounts.shield_booster_asset.as_ref(),
            fight_card_link,
            event_link,
            None,
            shield_booster_asset_nonce,
        )?;
        process_game_asset_for_action(
            ctx.accounts.points_booster_asset.as_mut(),
            ctx.accounts.points_booster_link.as_mut(),
            &signer.key(),
            true,
        )?;
        process_and_verify_game_asset_type(
            ctx.accounts.points_booster_asset.as_ref(),
            fight_card_link,
            event_link,
            None,
            points_booster_asset_nonce,
        )?;

        process_game_asset_for_action(
            champions_pass_asset.clone().as_mut(),
            ctx.accounts.champions_pass_link.as_mut(),
            &signer.key(),
            true,
        )?;

        process_and_verify_game_asset_type(
            champions_pass_asset.as_ref(),
            fight_card_link,
            event_link,
            Some(&event.tournament_type),
            champions_pass_asset_nonce.clone(),
        )?;

        require!(
            fight_card_link.fighter_used.is_some()
                && fight_card_link.fighter_nonce_tracker.is_some(),
            ErrorCode::FightCardLinkedToGameAsset
        );
        match event.tournament_type {
            TournamentType::MainCard => {
                require!(
                    event_link.champions_pass_pubkey.is_some()
                        && event_link.champions_pass_nonce_tracker.is_some(),
                    ErrorCode::EventLinkedToGameAsset
                );
            }
            _ => {}
        }

        fight_card_link.fight_card_pubkey = fight_card.to_account_info().key();
        fight_card_link.fight_card_nonce_tracker = fight_card_nonce;
        fight_card_link.fighter_color_side = fighter_color_side;
        fight_card_link.is_consumed = false;
        fight_card_link.is_initialized = true;

        Ok(())
    }

    /*
       TODO: Create a ranking rewards system for events
    */

    /*
       TODO: Admin resolve ranking
    */

    /*
       TODO: Claim event reward
    */

    /*
       TODO: Deposit NFT to my collection
    */

    /*
       TODO: Withdraw NFT from my collection
    */
}
