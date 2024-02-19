// src/errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message for an invalid operation.")]
    InvalidOperation,
    #[msg("Unauthorized access attempt")]
    Unauthorized, // ... other errors ...
    #[msg("Already initialized")]
    AlreadyInitialized,
    #[msg("Duplicate NFT type in purchase request")]
    InvalidArgumentInPurchaseRequest,
    #[msg("Invalid Price Feed")]
    InvalidPriceFeed,
    #[msg("Switchboard feed has not been updated in 5 minutes")]
    StaleFeed,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
