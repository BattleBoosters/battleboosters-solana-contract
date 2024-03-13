use super::collector_pack::CollectorPack;
use super::player::PlayerData;
use super::program::ProgramData;
use super::rarity::RarityData;
use crate::constants::*;
use crate::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};
use std::str::FromStr;
use switchboard_solana::AggregatorAccountData;

use solana_randomness_service::program::SolanaRandomnessService;
use solana_randomness_service::SimpleRandomnessV1Account;
use switchboard_solana::prelude::*;

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
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PLAYER, recipient.key().as_ref()],
    bump,
    )]
    pub player_account: Account<'info, PlayerData>,
    #[account(
    init,
    payer = signer,
    seeds = [MY_APP_PREFIX, COLLECTOR, recipient.key().as_ref(), player_account.order_nonce.to_le_bytes().as_ref()],
    bump,
    space = 8 + 8 + 8 + 1 + 8
    )]
    pub collector_pack: Account<'info, CollectorPack>,
    /// CHECK: This is a PDA used as the bank
    #[account(mut, seeds = [MY_APP_PREFIX, BANK], bump = program.bank_bump)]
    pub bank: AccountInfo<'info>,
    /// CHECK: This is a PDA used as the bank
    #[account(mut, seeds = [MY_APP_PREFIX, BANK, signer.key().as_ref()], bump)]
    pub bank_escrow: AccountInfo<'info>,

    #[account(mut)]
    pub randomness: Account<'info, CollectorPack>,

    /// The Solana System program. Used to allocate space on-chain for the randomness_request account.
    pub system_program: Program<'info, System>,
}
