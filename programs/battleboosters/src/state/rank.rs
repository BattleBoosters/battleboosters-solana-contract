use crate::constants::*;
use crate::state::event::EventData;
use crate::state::program::ProgramData;
use anchor_lang::prelude::*;
/*
   TODO: Create rank tier reward account
*/
// #[derive(Accounts)]
// #[instruction(event_nonce: u64)]
// pub struct InitializeRank<'info> {
//     #[account(mut)]
//     pub creator: Signer<'info>,
//     #[account(
//     mut,
//     seeds = [MY_APP_PREFIX, EVENT, event_nonce.to_le_bytes().as_ref()],
//     bump
//     )]
//     pub event: Box<Account<'info, EventData>>,
//
//     /*
//         TODO: remove `creator.key()` and add a counter,
//          we need to ensure the player cannot create more than one of this per events.
//          One way to ensure the player cannot initialize multiple time this is to create it directly on
//          event `InitializeEventLink` so we initialize also the rank PDA :q
//     */
//     #[account(
//     init,
//     payer = creator,
//     seeds = [MY_APP_PREFIX, RANK, event.key().as_ref(), creator.key().as_ref()],
//     bump,
//     space = 8 + 8 + 8 + 1
//     )]
//     pub rank: Account<'info, RankData>,
//     pub system_program: Program<'info, System>,
// }

#[derive(Accounts)]
pub struct UpdateRank<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump
    )]
    pub program: Box<Account<'info, ProgramData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, EVENT, event.nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Box<Account<'info, EventData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, RANK, event.key().as_ref(), rank.nonce.to_le_bytes().as_ref()],
    bump,
    )]
    pub rank: Box<Account<'info, RankData>>,
}

#[account]
pub struct RankData {
    pub player_account: Pubkey,
    pub rank: Option<u64>,
    pub total_points: Option<u64>,
    pub is_consumed: bool,
    pub nonce: u64,
}
