// src/errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message for an invalid operation.")]
    InvalidOperation,
    #[msg("Unauthorized access attempt")]
    Unauthorized, // ... other errors ...
}
