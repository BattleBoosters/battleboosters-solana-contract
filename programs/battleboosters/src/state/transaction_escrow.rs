use super::mystery_box::MysteryBoxData;
use super::player::PlayerData;
use super::program::ProgramData;
use crate::constants::*;
use crate::state::rarity::RarityData;
use crate::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use std::str::FromStr;
// use switchboard_solana::prelude::{AggregatorAccountData, AccountLoader};

#[derive(Accounts)]
pub struct TransactionEscrow<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Receiver of the pack we use this account only for crediting fighter packs and boosters
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    #[account(mut, seeds = [MY_APP_PREFIX, PROGRAM_STATE], bump)]
    pub program: Box<Account<'info, ProgramData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PLAYER, recipient.key().as_ref()],
    bump,
    )]
    pub player_account: Box<Account<'info, PlayerData>>,
    #[account(
    init,
    payer = signer,
    seeds = [MY_APP_PREFIX, MYSTERY_BOX, player_account.order_nonce.to_le_bytes().as_ref(), recipient.key().as_ref()],
    bump,
    space = 128 + 32 + 8
    )]
    pub mystery_box: Box<Account<'info, MysteryBoxData>>,
    /// CHECK: This is a PDA used as the bank
    #[account(mut, seeds = [MY_APP_PREFIX, BANK], bump)]
    pub bank: AccountInfo<'info>,

    // /// CHECK: Switchboard network price feed id
    // #[account(address = Pubkey::from_str(SOL_USD_FEED_MAINNET).unwrap() @ ErrorCode::InvalidPriceFeed)]
    // pub price_feed: AccountLoader<'info, AggregatorAccountData>,

    /// Rarity PDA
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, RARITY],
    bump,
    )]
    pub rarity: Box<Account<'info, RarityData>>,

    // /// CHECK: The account's data is validated manually within the handler.
    // pub randomness_account_data: AccountInfo<'info>,
    //
    /// The Solana System program. Used to allocate space on-chain for the randomness_request account.
    pub system_program: Program<'info, System>,
    // /// The Solana Token program. Used to transfer funds to the randomness escrow.
    // pub token_program: Program<'info, Token>,
    //
    // /// The Solana Associated Token program. Used to create the TokenAccount for the randomness escrow.
    // pub associated_token_program: Program<'info, AssociatedToken>,
}

// #[derive(Accounts)]
// #[instruction(bank_escrow_bump: u8)]
// pub struct TransactionEscrow<'info> {
//     #[account(mut)]
//     pub signer: Signer<'info>,
//     /// CHECK: Receiver of the pack we use this account only for crediting fighter packs and boosters
//     #[account(mut)]
//     pub recipient: AccountInfo<'info>,
//     #[account(mut, seeds = [MY_APP_PREFIX, PROGRAM_STATE], bump)]
//     pub program: Account<'info, ProgramData>,
//     #[account(
//         mut,
//         seeds = [MY_APP_PREFIX, PLAYER, recipient.key().as_ref()],
//         bump,
//     )]
//     pub player_account: Account<'info, PlayerData>,
//     #[account(
//         init,
//         payer = signer,
//         seeds = [MY_APP_PREFIX, MYSTERY_BOX, recipient.key().as_ref(), player_account.order_nonce.to_le_bytes().as_ref()],
//         bump,
//         space = 128
//     )]
//     pub mystery_box: Account<'info, MysteryBoxData>,
//     /// CHECK: This is a PDA used as the bank
//     #[account(mut, seeds = [MY_APP_PREFIX, BANK], bump)]
//     pub bank: AccountInfo<'info>,
//     /// CHECK: This is a PDA used as the bank
//     #[account(mut, seeds = [MY_APP_PREFIX, BANK, signer.key().as_ref()], bump)]
//     pub bank_escrow: AccountInfo<'info>,
//
//     /// CHECK: Switchboard network price feed id
//     #[account(address = Pubkey::from_str(SOL_USD_FEED_MAINNET).unwrap() @ ErrorCode::InvalidPriceFeed)]
//     pub price_feed: AccountLoader<'info, AggregatorAccountData>,
//     /// The Solana Randomness Service program.
//     pub randomness_service: Program<'info, SolanaRandomnessService>,
//     /// The account that will be created on-chain to hold the randomness request.
//     /// Used by the off-chain oracle to pickup the request and fulfill it.
//     /// CHECK: todo
//     #[account(
//     mut,
//     signer,
//     owner = system_program.key(),
//     constraint = randomness_request.data_len() == 0 && randomness_request.lamports() == 0,
//     )]
//     pub randomness_request: AccountInfo<'info>,
//
//     /// The TokenAccount that will store the funds for the randomness request.
//     /// CHECK: todo
//     #[account(
//     mut,
//     owner = system_program.key(),
//     constraint = randomness_escrow.data_len() == 0 && randomness_escrow.lamports() == 0,
//     )]
//     pub randomness_escrow: AccountInfo<'info>,
//     /// The randomness service's state account. Responsible for storing the
//     /// reward escrow and the cost per random byte.
//     #[account(
//     seeds = [b"STATE"],
//     bump = randomness_state.bump,
//     seeds::program = randomness_service.key(),
//     )]
//     pub randomness_state: Box<Account<'info, solana_randomness_service::State>>,
//
//     /// The token mint to use for paying for randomness requests.
//     #[account(address = NativeMint::ID)]
//     pub randomness_mint: Account<'info, Mint>,
//
//     /// The Solana System program. Used to allocate space on-chain for the randomness_request account.
//     pub system_program: Program<'info, System>,
//
//     /// The Solana Token program. Used to transfer funds to the randomness escrow.
//     pub token_program: Program<'info, Token>,
//
//     /// The Solana Associated Token program. Used to create the TokenAccount for the randomness escrow.
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     /// Rarity PDA
//     #[account(
//     mut,
//     seeds = [MY_APP_PREFIX, RARITY],
//     bump,
//     )]
//     pub rarity: Account<'info, RarityData>,
// }
