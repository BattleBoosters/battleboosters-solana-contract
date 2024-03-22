use super::collector_pack::CollectorPackData;
use super::player::PlayerData;
use super::program::ProgramData;
use crate::constants::*;
use anchor_lang::prelude::*;

use crate::state::rarity::RarityData;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::sysvar;
use solana_randomness_service::SimpleRandomnessV1Account;
use solana_randomness_service::ID as SolanaRandomnessServiceID;
use switchboard_solana::prelude::*;
// Struct for managing player inventory
#[derive(Accounts)]
#[instruction(order_nonce: u64)]
pub struct ConsumeRandomness<'info> {
    /// We need to make sure the randomness service signed this requests so it can only be invoked by a PDA and not a user.
    #[account(
    signer,
    seeds = [b"STATE"],
    seeds::program = SolanaRandomnessServiceID,
    bump = randomness_state.bump,
    )]
    pub randomness_state: Box<Account<'info, solana_randomness_service::State>>,
    pub request: Box<Account<'info, SimpleRandomnessV1Account>>,
    /// CHECK:
    pub recipient: AccountInfo<'info>,
    // /// CHECK: Only used to verify
    // pub signer: AccountInfo<'info>,
    // /// CHECK:
    // pub player_account: Box<Account<'info, PlayerData>>,
    // #[account(
    // mut,
    // seeds = [MY_APP_PREFIX, PLAYER, recipient.key().as_ref()],
    // bump,
    // )]
    // pub player_account: Box<Account<'info, PlayerData>>,
    /// CHECK:
    #[account(mut, seeds = [MY_APP_PREFIX, COLLECTOR, recipient.key().as_ref(), order_nonce.to_le_bytes().as_ref()], bump)]
    pub collector_pack: Box<Account<'info, CollectorPackData>>,
    // /// CHECK: This is a PDA used as the bank
    // #[account(mut, seeds = [MY_APP_PREFIX, BANK], bump = program.bank_bump)]
    // /// CHECK:
    // pub bank: AccountInfo<'info>,
    // // /// CHECK: This is a PDA used as the bank
    // // #[account(mut, seeds = [MY_APP_PREFIX, BANK, signer.key().as_ref()], bump)]
    // /// CHECK:
    // pub bank_escrow: AccountInfo<'info>,
    // /// CHECK: This is a PDA used as the bank
    // #[account(mut, seeds = [MY_APP_PREFIX, BANK], bump)]
    // pub bank: AccountInfo<'info>,
    // /// CHECK: This is a PDA used as the bank
    // #[account(mut, seeds = [MY_APP_PREFIX, BANK, signer.key().as_ref()], bump)]
    // pub bank_escrow: AccountInfo<'info>,
    // /// CHECK:
    // pub system_program: AccountInfo<'info>,
}
