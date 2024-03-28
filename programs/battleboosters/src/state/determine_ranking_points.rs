/*
   TODO: Create determine ranking points accounts
*/
use crate::constants::*;
use anchor_lang::prelude::*;
use switchboard_solana::prelude::*;
#[derive(Accounts)]
#[instruction(event_nonce: u64)]
pub struct DetermineRankingPoints<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}
