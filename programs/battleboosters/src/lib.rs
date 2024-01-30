use anchor_lang::prelude::*;
mod constants;
mod errors;
mod events;
mod state;

use errors::ErrorCode;

use crate::events::*;
use crate::state::event::*;
// use crate::state::fight_card::*;
use crate::state::global::*;

declare_id!("9DZTGocMWp5n7nH9dfN4VMxhDoZuN82AAsne4qcaWygJ");

#[program]
pub mod battleboosters {

    use super::*;

    pub fn initialize(
        ctx: Context<GlobalState>,
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

        require!(
            ctx.accounts.creator.key() == global_state.admin_pubkey,
            ErrorCode::Unauthorized
        );

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

    // pub fn create_new_fight_card(ctx: Context<FightCard>) -> Result<()> {
    //     Ok(())
    // }

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
