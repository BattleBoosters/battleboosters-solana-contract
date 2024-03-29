/*
   TODO: Create determine ranking points accounts
*/
use crate::constants::*;
use anchor_lang::prelude::*;
use switchboard_solana::prelude::*;
#[derive(Accounts)]
#[instruction(rank_nonce: u64)]
pub struct DetermineRankingPoints<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /*
       TODO:
           - Add player rank to modify the points. Rank will be determined off-chain
           - Store the sequence per rounds and Determine ranks per rounds
    */
}
