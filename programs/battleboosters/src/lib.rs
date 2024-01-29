use anchor_lang::prelude::*;
mod errors;
mod state;
mod constants;

use errors::ErrorCode;
use crate::state::global::*;
use crate::state::event::*;



declare_id!("9DZTGocMWp5n7nH9dfN4VMxhDoZuN82AAsne4qcaWygJ");

#[program]
pub mod battleboosters {
    use super::*;

    pub fn initialize(ctx: Context<GlobalState>, admin_pubkey: Pubkey) -> Result<()> {
        let global_state = &mut ctx.accounts.new_account;
        global_state.event_counter = 0_u64;
        global_state.admin_pubkey = admin_pubkey;

        Ok(())
    }

    pub fn create_new_event(ctx: Context<CreateEvent>, start_date: i64, end_date: i64) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        require!(ctx.accounts.creator.key() == global_state.admin_pubkey, ErrorCode::Unauthorized);

        let create_event = &mut ctx.accounts.event_account;
        create_event.fight_card_id_counter += 1_u8;
        create_event.start_date = start_date;
        create_event.end_date = end_date;

        // Increment event counter
        global_state.event_counter += 1_u64;

        Ok(())
    }
}
