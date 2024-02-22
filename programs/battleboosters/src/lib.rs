use anchor_lang::prelude::*;

use anchor_spl::token::{InitializeMint, MintTo};
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
    create_spl_nft::*, event::*, fight_card::*, player::*, program::*, transaction_escrow::*,
};

use crate::types::*;
use crate::utils::*;

use errors::ErrorCode;
use std::collections::HashSet;

use mpl_token_metadata::instructions::{
    BurnCpiBuilder, CreateMetadataAccountV3, CreateV1, CreateV1Builder, CreateV1CpiBuilder,
    MintV1CpiBuilder, TransferV1Cpi, TransferV1CpiAccounts, TransferV1InstructionArgs,
};

use mpl_token_metadata::types::{DataV2, PrintSupply, TokenStandard};
// use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed, Price};

declare_id!("H85sU4mupXtsMZmtHM4y1Cucjfb7SVh7Q3eFrbZPX6a1");

#[program]
pub mod battleboosters {
    use super::*;
    use crate::state::player::InitializePlayer;
    use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
    use anchor_lang::solana_program::program::invoke_signed;
    use anchor_lang::solana_program::system_instruction;

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
        program.event_counter = 0_u64;
        program.admin_pubkey = admin_pubkey;
        program.fighter_pack_price = nft_fighter_pack_price;
        program.booster_price = booster_price;
        program.fighter_pack_amount = fighter_pack_amount;
        program.is_initialized = true;

        Ok(())
    }

    pub fn initialize_player(ctx: Context<InitializePlayer>, player_pubkey: Pubkey) -> Result<()> {
        let player = &mut ctx.accounts.inventory;
        require!(!player.is_initialized, ErrorCode::AlreadyInitialized);

        player.fighter_mint_allowance = 0;
        player.booster_mint_allowance = 0;
        player.is_initialized = true;

        Ok(())
    }

    pub fn create_nft_collection(
        ctx: Context<CreateSplNft>,
        collection_id: CollectionType,
        collection_name: String,
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

        let mut binding = CreateV1CpiBuilder::new(&metadata_program);

        let create_cpi = binding
            .metadata(&metadata)
            .mint(&minter, false)
            .authority(&authority)
            .payer(&payer)
            .update_authority(&authority, true)
            .master_edition(Some(&ctx.accounts.master_edition))
            .system_program(&ctx.accounts.system_program)
            .sysvar_instructions(&sysvar)
            .spl_token_program(Some(&spl_token_program))
            .token_standard(TokenStandard::ProgrammableNonFungible)
            .name(collection_name)
            .uri(uri)
            .seller_fee_basis_points(fees)
            .is_mutable(true)
            .print_supply(PrintSupply::Unlimited);

        let authority_seeds = [
            MY_APP_PREFIX,
            MINT_AUTHORITY,
            &[program.authority_bump.clone()],
        ];
        create_cpi.invoke_signed(&[&authority_seeds])?;

        Ok(())
    }

    pub fn purchase_nfts(
        ctx: Context<TransactionEscrow>,
        bank_escrow_bump: u8,
        requests: Vec<PurchaseRequest>,
    ) -> Result<()> {
        let program = &ctx.accounts.program;
        let feed = &ctx.accounts.price_feed.load()?;
        let player_inventory = &mut ctx.accounts.player_inventory;

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
                    player_inventory.booster_mint_allowance += &request.quantity;

                    total_usd += request
                        .quantity
                        .checked_mul(program.booster_price.clone())
                        .unwrap();
                }
                NftType::FighterPack => {
                    // update the quantity of fighter mint allowance
                    player_inventory.fighter_mint_allowance += &request.quantity;

                    total_usd += request
                        .quantity
                        .checked_mul(program.fighter_pack_price.clone())
                        .unwrap();
                }
            }
        }

        require!(total_usd > 0, ErrorCode::InsufficientAmount);

        let bank = &ctx.accounts.bank;
        let bank_escrow = &ctx.accounts.bank_escrow;
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

        // Calculate the minimum balance required to remain rent-exempt
        let rent_exempt_balance = Rent::get()?.minimum_balance(bank_escrow.data_len());
        // Calculate the maximum amount that can be safely withdrawn while keeping the account rent-exempt
        let withdrawable_balance = bank_escrow_balance.saturating_sub(rent_exempt_balance);

        // Construct the transfer instruction
        let transfer_instruction = system_instruction::transfer(
            &bank_escrow.key(),
            &bank.key(),
            withdrawable_balance, // Amount in lamports to transfer
        );

        let signer = &ctx.accounts.signer.key();
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

    // pub fn open_nft() -> Result<()> {
    //
    //     Ok(())
    // }

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
            event_id: program.event_counter
        });

        // Increment event counter
        program.event_counter += 1_u64;

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
            event_id: program.event_counter
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
