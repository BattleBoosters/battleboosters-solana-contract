use anchor_lang::prelude::*;
use crate::constants::*;
use crate::ErrorCode;
use std::str::FromStr;
use switchboard_solana::{AggregatorAccountData};

#[derive(Accounts)]
pub struct FetchSolUsdPrice<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Switchboard network price feed id
    #[account(address = Pubkey::from_str(SOL_USD_FEED_DEVNET).unwrap() @ ErrorCode::InvalidPriceFeed)]
    pub price_feed: AccountLoader<'info, AggregatorAccountData>,
}
