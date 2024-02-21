use super::player::InventoryData;
use super::program::ProgramData;
use crate::constants::*;
use crate::ErrorCode;
use anchor_lang::prelude::*;
use std::str::FromStr;
use switchboard_solana::AggregatorAccountData;

#[derive(Accounts)]
#[instruction(bank_escrow_bump: u8)]
pub struct TransactionEscrow<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Receiver of the pack we use this account only for crediting fighter packs and boosters
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    #[account(mut, seeds = [MY_APP_PREFIX, PROGRAM_STATE], bump)]
    pub program: Account<'info, ProgramData>,
    #[account(mut, seeds = [MY_APP_PREFIX, INVENTORY, recipient.key().as_ref()], bump)]
    pub player_inventory: Account<'info, InventoryData>,
    /// CHECK: This is a PDA used as the bank
    #[account(mut, seeds = [MY_APP_PREFIX, BANK], bump = program.bank_bump)]
    pub bank: AccountInfo<'info>,
    /// CHECK: This is a PDA used as the bank
    #[account(mut, seeds = [MY_APP_PREFIX, BANK, signer.key().as_ref()], bump = bank_escrow_bump)]
    pub bank_escrow: AccountInfo<'info>,
    /// CHECK: Switchboard network price feed id
    #[account(address = Pubkey::from_str(SOL_USD_FEED_MAINNET).unwrap() @ ErrorCode::InvalidPriceFeed)]
    pub price_feed: AccountLoader<'info, AggregatorAccountData>,
    pub system_program: Program<'info, System>,
}
