use anchor_lang::prelude::*;

#[event]
pub struct EventCreated {
    pub event_id: u64,
}

#[event]
pub struct EventUpdated {
    pub event_id: u64,
}
