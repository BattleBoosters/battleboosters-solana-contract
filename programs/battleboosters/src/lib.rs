use anchor_lang::prelude::*;
mod constants;
mod errors;
mod events;
mod state;
mod utils;

use errors::ErrorCode;

use crate::events::*;
use crate::state::event::*;
use crate::state::fight_card::*;
use crate::state::global_state::*;
use crate::utils::*;

declare_id!("9DZTGocMWp5n7nH9dfN4VMxhDoZuN82AAsne4qcaWygJ");

#[program]
pub mod battleboosters {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeGlobalState>,
        admin_pubkey: Pubkey,
        rarity: Rarity,
        rarity_probabilities: Vec<u8>,
        nft_fighter_pack_price: u64,
        nft_booster_pack_price: u64,
    ) -> Result<()> {
        // Create global state
        let global_state = &mut ctx.accounts.new_account;
        global_state.event_counter = 0_u64;
        global_state.admin_pubkey = admin_pubkey;
        global_state.rarity = rarity;
        global_state.rarity_probabilities = rarity_probabilities;
        global_state.nft_fighter_pack_price = nft_fighter_pack_price;
        global_state.nft_booster_pack_price = nft_booster_pack_price;

        Ok(())
    }

    pub fn create_new_event(
        ctx: Context<CreateEvent>,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        only_admin(&ctx.accounts.creator.key(), &global_state.admin_pubkey)?;

        // Increment event counter
        global_state.event_counter += 1_u64;



        // Create event account and set data
        let create_event = &mut ctx.accounts.event_account;
        create_event.fight_card_id_counter = 0_u8;
        create_event.start_date = start_date;
        create_event.end_date = end_date;

        emit!(EventCreated {
            event_id: global_state.event_counter
        });

        Ok(())
    }

    pub fn create_new_fight_card(
        ctx: Context<CreateFightCard>,
        params: FightCardData,
    ) -> Result<()> {
        let global_state = &ctx.accounts.global_state;
        only_admin(&ctx.accounts.creator.key(), &global_state.admin_pubkey)?;

        let event = &mut ctx.accounts.event;
        event.fight_card_id_counter = event.fight_card_id_counter.checked_add(1_u8).unwrap();

        let fight_card = &mut ctx.accounts.fight_card_account;
        fight_card.id = params.id;
        fight_card.event_pubkey = params.event_pubkey;
        fight_card.title_fight = params.title_fight;
        fight_card.result = None;
        fight_card.winner = None;

        if let Some(fight_duration) = params.fight_duration {
            fight_card.fight_duration = Some(fight_duration);
        } else {
            fight_card.fight_duration = None
        }

        if let Some(fight_stats_fighter_1) = params.fight_stats_fighter_1 {
            fight_card.fight_stats_fighter_1 = Some(fight_stats_fighter_1);
        } else {
            fight_card.fight_stats_fighter_1 = None
        }

        if let Some(fight_stats_fighter_2) = params.fight_stats_fighter_2 {
            fight_card.fight_stats_fighter_2 = Some(fight_stats_fighter_2);
        } else {
            fight_card.fight_stats_fighter_2 = None
        }

        if let Some(tournament_type) = params.tournament {
            fight_card.tournament = Some(tournament_type);
        } else {
            fight_card.tournament = None
        }

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
