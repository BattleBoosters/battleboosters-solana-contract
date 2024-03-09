// src/errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message for an invalid operation")]
    InvalidOperation,
    #[msg("Unauthorized access attempt")]
    Unauthorized, // ... other errors ...
    #[msg("Already initialized")]
    AlreadyInitialized,
    #[msg("The provided NFT type is not supported for this operation")]
    UnsupportedNftType,
    #[msg("Invalid Price Feed")]
    InvalidPriceFeed,
    #[msg("Feed has not been updated in 5 minutes")]
    StaleFeed,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Insufficient amount in purchase request")]
    InsufficientAmount,
    #[msg("Randomness unavailable for now")]
    RandomnessUnavailable,
    #[msg("No matching rarity found")]
    NoMatchingRarityFound,
    #[msg("The nonce must not exceed the last known nonce in the player's state")]
    WrongPlayerGameAssetLinkNonce,
    #[msg("This player game asset pda is not free")]
    NotFreePDA,
    #[msg("Not enough allowance to generate mintable game asset")]
    NotEnoughAllowance,
    #[msg("The event has already started")]
    EventAlreadyStarted,
}
