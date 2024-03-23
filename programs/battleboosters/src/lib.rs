use anchor_lang::prelude::*;
mod constants;
mod errors;
mod events;
mod processor;
mod state;
mod types;
mod utils;

use crate::state::{
    create_spl_nft::*, event::*, fight_card::*, join_fight_card::*, mint_nft_from_game_asset::*,
    player::*, program::*, rarity::*, switchboard_callback::*, transaction_escrow::*,
};

use crate::types::*;
use crate::utils::*;

use errors::ErrorCode;

declare_id!("5GW3wfyowgfKsKCeC2VKg6ucM4wKYX5ebZNAqvBcvTSd");

#[program]
pub mod battleboosters {
    use super::*;
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
        processor::initialize_player(ctx, player_pubkey)
    }

    pub fn initialize_event_link(
        ctx: Context<InitializeEventLink>,
        event_nonce: u64,
    ) -> Result<()> {
        processor::initialize_event_link(ctx, event_nonce)
    }

    pub fn create_nft_collection(
        ctx: Context<CreateSplNft>,
        collection_id: CollectionType, /* Used in initialization */
        collection_name: String,
        symbol: String,
        uri: String,
        fees: u16,
    ) -> Result<()> {
        processor::create_nft_collection(ctx, collection_id, collection_name, symbol, uri, fees)
    }

    pub fn purchase_mystery_box(
        ctx: Context<TransactionEscrow>,
        bank_escrow_bump: u8,
        requests: Vec<PurchaseRequest>,
    ) -> Result<()> {
        processor::purchase_mystery_box(ctx, bank_escrow_bump, requests)
    }

    pub fn consume_randomness(
        ctx: Context<ConsumeRandomness>,
        order_nonce: u64,
        // bank_escrow_bump: u8,
        // total_lamports: u64,
        result: Vec<u8>,
    ) -> Result<()> {
        processor::consume_randomness(ctx, order_nonce, result)
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
        let mystery_box = &mut ctx.accounts.mystery_box;

        mystery_box.randomness = Some(vec![12, 23, 34, 34, 54, 74, 94, 23]);
        mystery_box.booster_mint_allowance = booster_mint_alowance;
        mystery_box.fighter_mint_allowance = fighter_mint_allowance;
        mystery_box.champions_pass_mint_allowance = champions_pass_mint_allowance;
        Ok(())
    }

    pub fn generate_mintable_game_asset(
        ctx: Context<GenerateNftPreMint>,
        mintable_game_asset_link_nonce: u64, // used on instruction
        request: OpenRequest,
    ) -> Result<()> {
        processor::generate_mintable_game_asset(ctx, mintable_game_asset_link_nonce, request)
    }

    /*
        TODO: This method will come after the release of the game
        TODO: Withdraw NFT from my collection
    */
    pub fn mint_nft_from_game_asset(
        ctx: Context<MintNftFromGameAsset>,
        //requests: Vec<PurchaseRequest>,
    ) -> Result<()> {
        processor::mint_nft_from_game_asset(ctx)
    }

    pub fn create_new_event(
        ctx: Context<CreateEvent>,
        start_date: i64,
        end_date: i64,
        tournament_type: TournamentType,
        rank_reward: Vec<RankReward>,
    ) -> Result<()> {
        processor::create_new_event(ctx, start_date, end_date, tournament_type, rank_reward)
    }

    pub fn update_event(
        ctx: Context<UpdateEvent>,
        event_nonce: u64,
        start_date: i64,
        end_date: i64,
        tournament_type: TournamentType,
        rank_reward: Vec<RankReward>,
    ) -> Result<()> {
        processor::update_event(
            ctx,
            event_nonce,
            start_date,
            end_date,
            tournament_type,
            rank_reward,
        )
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

    /*
       TODO: Performs more unit test
    */
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
        processor::join_fight_card(
            ctx,
            event_nonce,
            fight_card_nonce,
            fighter_asset_nonce,
            energy_booster_asset_nonce,
            shield_booster_asset_nonce,
            points_booster_asset_nonce,
            champions_pass_asset_nonce,
            fighter_link_nonce,
            energy_booster_link_nonce,
            shield_booster_link_nonce,
            points_booster_link_nonce,
            champions_pass_link_nonce,
            fighter_color_side,
        )
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
}
