use super::program::ProgramData;
use crate::constants::*;
use crate::ErrorCode;
use anchor_lang::prelude::*;
use std::str::FromStr;
use switchboard_solana::AggregatorAccountData;

#[derive(Accounts)]
#[instruction(bank_bump: u8)]
pub struct TransactionEscrow<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub program: Account<'info, ProgramData>,
    /// CHECK: This is a PDA used as the bank
    #[account(mut, seeds = [MY_APP_PREFIX, BANK, signer.key().as_ref()], bump = bank_bump)]
    pub bank: AccountInfo<'info>,
    /// CHECK: Switchboard network price feed id
    #[account(address = Pubkey::from_str(SOL_USD_FEED_MAINNET).unwrap() @ ErrorCode::InvalidPriceFeed)]
    pub price_feed: AccountLoader<'info, AggregatorAccountData>,
}
