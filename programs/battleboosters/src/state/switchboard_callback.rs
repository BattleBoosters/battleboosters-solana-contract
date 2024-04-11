use super::mystery_box::MysteryBoxData;
use crate::constants::*;
use crate::state::event::EventData;
use anchor_lang::prelude::*;
use switchboard_solana::prelude::*;

// Struct for managing player inventory
#[derive(Accounts)]
#[instruction(order_nonce: u64)]
pub struct ConsumeRandomness<'info> {
    /// CHECK:
    pub recipient: AccountInfo<'info>,
    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
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
    #[account(mut, seeds = [MY_APP_PREFIX, MYSTERY_BOX, recipient.key().as_ref(), order_nonce.to_le_bytes().as_ref()], bump)]
    pub mystery_box: Box<Account<'info, MysteryBoxData>>,
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

#[derive(Accounts)]
#[instruction(event_nonce: u64)]
pub struct ConsumeRandomnessEvent<'info> {
    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
    /// CHECK:
    #[account(mut, seeds = [MY_APP_PREFIX, EVENT, event_nonce.to_le_bytes().as_ref()], bump)]
    pub event: Box<Account<'info, EventData>>,
}
