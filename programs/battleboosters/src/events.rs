use anchor_lang::prelude::*;

/// Emit an event with the created event id
#[event]
pub struct EventCreated {
    pub event_id: u64,
}
