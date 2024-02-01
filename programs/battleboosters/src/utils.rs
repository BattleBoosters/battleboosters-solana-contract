use crate::errors::ErrorCode;
use crate::state::fight_card::*;
use anchor_lang::prelude::*;

pub fn only_admin(creator: &Pubkey, admin: &Pubkey) -> Result<()> {
    require!(creator == admin, ErrorCode::Unauthorized);
    Ok(())
}
pub fn set_fight_card_properties(fight_card: &mut FightCardData, params: &FightCardData) {
    fight_card.id = params.id.clone();
    fight_card.event_pubkey = params.event_pubkey;
    fight_card.title_fight = params.title_fight.clone();
    fight_card.result = None;
    fight_card.winner = None;
    fight_card.tournament = params.tournament.clone();

    if let Some(fight_duration) = params.fight_duration.clone() {
        fight_card.fight_duration = Some(fight_duration);
    } else {
        fight_card.fight_duration = None
    }

    if let Some(fight_stats_fighter_1) = params.fight_stats_fighter_1.clone() {
        fight_card.fight_stats_fighter_1 = Some(fight_stats_fighter_1);
    } else {
        fight_card.fight_stats_fighter_1 = None
    }

    if let Some(fight_stats_fighter_2) = params.fight_stats_fighter_2.clone() {
        fight_card.fight_stats_fighter_2 = Some(fight_stats_fighter_2);
    } else {
        fight_card.fight_stats_fighter_2 = None
    }
}
