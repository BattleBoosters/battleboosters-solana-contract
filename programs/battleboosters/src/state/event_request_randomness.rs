/*
   TODO: request randomness for an event at the end of the events only allowed.
*/
use crate::constants::*;
use crate::state::event::EventData;
use crate::state::program::ProgramData;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};
use solana_randomness_service::program::SolanaRandomnessService;
use switchboard_solana::prelude::*;
#[derive(Accounts)]
#[instruction(event_nonce: u64)]
pub struct EventRequestRandomness<'info> {
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
    seeds = [MY_APP_PREFIX, EVENT, event_nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub event: Box<Account<'info, EventData>>,
    /// The Solana Randomness Service program.
    pub randomness_service: Program<'info, SolanaRandomnessService>,
    /// The account that will be created on-chain to hold the randomness request.
    /// Used by the off-chain oracle to pickup the request and fulfill it.
    /// CHECK: todo
    #[account(
    mut,
    signer,
    owner = system_program.key(),
    constraint = randomness_request.data_len() == 0 && randomness_request.lamports() == 0,
    )]
    pub randomness_request: AccountInfo<'info>,

    /// The TokenAccount that will store the funds for the randomness request.
    /// CHECK: todo
    #[account(
    mut,
    owner = system_program.key(),
    constraint = randomness_escrow.data_len() == 0 && randomness_escrow.lamports() == 0,
    )]
    pub randomness_escrow: AccountInfo<'info>,
    /// The randomness service's state account. Responsible for storing the
    /// reward escrow and the cost per random byte.
    #[account(
    seeds = [b"STATE"],
    bump = randomness_state.bump,
    seeds::program = randomness_service.key(),
    )]
    pub randomness_state: Box<Account<'info, solana_randomness_service::State>>,

    /// The token mint to use for paying for randomness requests.
    #[account(address = NativeMint::ID)]
    pub randomness_mint: Account<'info, Mint>,

    /// The Solana System program. Used to allocate space on-chain for the randomness_request account.
    pub system_program: Program<'info, System>,

    /// The Solana Token program. Used to transfer funds to the randomness escrow.
    pub token_program: Program<'info, Token>,

    /// The Solana Associated Token program. Used to create the TokenAccount for the randomness escrow.
    pub associated_token_program: Program<'info, AssociatedToken>,
}
