use anchor_lang::prelude::*;
mod constants;
mod errors;
mod events;
mod processor;
mod state;
mod types;
mod utils;
use crate::state::{
    collect_rewards::*, create_spl_nft::*, determine_ranking_points::*, event::*, fight_card::*,
    fighter_base::*, join_fight_card::*, mintable_game_asset::*, mystery_box::*, player::*,
    program::*, rank::*, rarity::*, transaction_escrow::*,
};
use crate::types::*;

declare_id!("87NrgFw8UwRoP79qaMpTN7mipE9MAn5LjAZytxNiFh5g");

#[program]
pub mod battleboosters {
    use super::*;
    
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
        processor::initialize(
            ctx,
            authority_bump,
            bank_bump,
            admin_pubkey,
            nft_fighter_price,
            booster_price,
            //fighter_pack_amount,
            env,
        )
    }

    pub fn update_program(ctx: Context<UpdateProgram>) -> Result<()> {
        processor::update_program(ctx)
    }

    pub fn initialize_rarity(
        ctx: Context<InitializeRarity>,
        fighter: Vec<RarityFighter>,
        //energy_booster: Vec<RarityBooster>,
        shield_booster: Vec<RarityBooster>,
        points_booster: Vec<RarityBooster>,
        probability_tiers: Vec<TierProbabilities>,
    ) -> Result<()> {
        processor::initialize_rarity(
            ctx,
            fighter,
            //energy_booster,
            shield_booster,
            points_booster,
            probability_tiers,
        )
    }

    pub fn initialize_player(
        ctx: Context<InitializePlayer>,
        player_pubkey: Pubkey, /* Used in initialization */
    ) -> Result<()> {
        processor::initialize_player(ctx, player_pubkey)
    }

    pub fn initialize_event_link(ctx: Context<InitializeEventLink>) -> Result<()> {
        processor::initialize_event_link(ctx)
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
    pub fn create_fighter(
        ctx: Context<CreateFighterBase>,
        fighter_type: FighterType,
        fight_metrics: FightMetrics,
    ) -> Result<()> {
        processor::create_fighter(ctx, fighter_type, fight_metrics)
    }

    pub fn update_fighter(
        ctx: Context<CreateFighterBase>,
        fighter_type: FighterType,
        fight_metrics: FightMetrics,
    ) -> Result<()> {
        processor::update_fighter(ctx, fighter_type, fight_metrics)
    }

    pub fn update_randomness_mystery_box(
        ctx: Context<UpdateMysteryBox>,
        mystery_box_nonce: u64,
        player_pubkey: Pubkey,
    ) -> Result<()> {
        processor::update_randomness_mystery_box(ctx, mystery_box_nonce, player_pubkey)
    }

    pub fn purchase_mystery_box(
        ctx: Context<TransactionEscrow>,
        requests: Vec<PurchaseRequest>,
    ) -> Result<()> {
        processor::purchase_mystery_box(ctx, requests)
    }

    // pub fn consume_randomness(
    //     ctx: Context<ConsumeRandomness>,
    //     order_nonce: u64,
    //     // bank_escrow_bump: u8,
    //     // total_lamports: u64,
    //     result: Vec<u8>,
    // ) -> Result<()> {
    //     processor::consume_randomness(ctx, order_nonce, result)
    // }

    // pub fn refund_mintable_game_asset(
    //     ctx: Context<RefundMintableGameAsset>,
    //     mintable_game_asset_link_nonce: u64,
    // ) -> Result<()> {
    //     processor::refund_mintable_game_asset(ctx, mintable_game_asset_link_nonce)
    // }

    pub fn create_mintable_game_asset(
        ctx: Context<CreateMintableGameAsset>,
        mintable_game_asset_link_nonce: u64, // used in instruction
        player_pubkey: Pubkey,
        request: OpenRequest,
    ) -> Result<()> {
        processor::create_mintable_game_asset(
            ctx,
            mintable_game_asset_link_nonce,
            player_pubkey,
            request,
        )
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
        start_date: i64,
        end_date: i64,
        tournament_type: TournamentType,
        rank_reward: Vec<RankReward>,
    ) -> Result<()> {
        processor::update_event(ctx, start_date, end_date, tournament_type, rank_reward)
    }

    pub fn create_new_fight_card(
        ctx: Context<CreateFightCard>,
        params: FightCardData,
    ) -> Result<()> {
        processor::create_new_fight_card(ctx, params)
    }

    pub fn update_fight_card(ctx: Context<UpdateFightCard>, params: FightCardData) -> Result<()> {
        processor::update_fight_card(ctx, params)
    }

    /*
       TODO: Performs more unit test
    */
    pub fn join_fight_card(
        ctx: Context<JoinFightCard>,
        fighter_color_side: FighterColorSide,
    ) -> Result<()> {
        processor::join_fight_card(ctx, fighter_color_side)
    }

    /*
       TODO: Admin resolve ranking, Calculate points
    */

    pub fn collect_rewards(ctx: Context<CollectRewards>) -> Result<()> {
        processor::collect_rewards(ctx)
    }

    // pub fn event_request_randomness(
    //     ctx: Context<EventRequestRandomness>,
    //     event_nonce: u64,
    // ) -> Result<()> {
    //     processor::event_request_randomness(ctx, event_nonce)
    // }
    // pub fn consume_randomness_event(
    //     ctx: Context<ConsumeRandomnessEvent>,
    //     event_nonce: u64,
    //     result: Vec<u8>,
    // ) -> Result<()> {
    //     processor::consume_randomness_event(ctx, event_nonce, result)
    // }

    pub fn admin_update_rank(ctx: Context<UpdateRank>, ranking: u64) -> Result<()> {
        processor::admin_update_rank(ctx, ranking)
    }

    pub fn determine_ranking_points(
        ctx: Context<DetermineRankingPoints>,
        fighter_type: FighterType,
    ) -> Result<()> {
        processor::determine_ranking_points(ctx, fighter_type)
    }

    /*
       TODO: Deposit NFT to my collection
    */

    /*
        TODO: This method will come after the release of the game
        TODO: Withdraw NFT from my collection
    */
    // pub fn mint_nft_from_game_asset(
    //     ctx: Context<MintNftFromGameAsset>,
    //     //requests: Vec<PurchaseRequest>,
    // ) -> Result<()> {
    //     processor::mint_nft_from_game_asset(ctx)
    // }
}
