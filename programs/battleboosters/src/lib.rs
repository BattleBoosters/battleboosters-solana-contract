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
use crate::state::program::*;
use crate::utils::*;

declare_id!("9DZTGocMWp5n7nH9dfN4VMxhDoZuN82AAsne4qcaWygJ");

#[program]
pub mod battleboosters {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeProgram>,
        admin_pubkey: Pubkey,
        rarity: Rarity,
        rarity_probabilities: Vec<u8>,
        nft_fighter_pack_price: u64,
        nft_booster_pack_price: u64,
    ) -> Result<()> {
        // Create program account
        let program = &mut ctx.accounts.program;
        program.event_counter = 0_u64;
        program.admin_pubkey = admin_pubkey;
        program.rarity = rarity;
        program.rarity_probabilities = rarity_probabilities;
        program.nft_fighter_pack_price = nft_fighter_pack_price;
        program.nft_booster_pack_price = nft_booster_pack_price;

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
