// src/errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message for an invalid operation")]
    InvalidOperation,
    #[msg("Unauthorized access attempt")]
    Unauthorized,
    #[msg("The mintable game asset link is not properly linked to the specified mintable game asset PDA")]
    GameAssetLinkNotLinkedToAssetPDA,
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
    #[msg("The event is still in progress. Please try again after it concludes on approximately")]
    EventStillRunning,
    #[msg("Attach rarity account to this transaction")]
    RarityAccountRequired,
    #[msg("Fight card link already has a game asset, or game asset nonce is missing")]
    FightCardLinkedToGameAsset,
    #[msg("Event card link already has a game asset, or game asset nonce is missing")]
    EventLinkedToGameAsset,
    #[msg("Booster type not found")]
    BoosterTypeNotFound,
    #[msg("Champion's pass not required for non-main card events")]
    NonMainCardEvent,
    #[msg("This mintable game asset has no owner")]
    MintableAssetHasNoOwner,
    #[msg("This mintable game asset is burnt")]
    MintableAssetBurned,
    #[msg("This mintable game asset is locked")]
    MintableAssetLocked,
    #[msg("This mintable game asset has been minted as an NFT and is no longer available for in-game use")]
    MintableAssetMintedAndUnavailable,
    #[msg("This mintable game asset link is missing")]
    MintableAssetLinkRequired,
    #[msg("The probability tier was not found")]
    ProbabilityTierNotFound,
    #[msg("Rank point is required")]
    RankPointsIsNone,
    #[msg("Rank is required")]
    RankIsNone,
    #[msg("The requested operation has already been consumed")]
    ConsumedAlready,
    #[msg(
        "Randomness is required to collect your reward. Please request randomness and try again."
    )]
    RandomnessIsNone,
    #[msg("Failed to parse value")]
    FailedToParseValue,
    #[msg("Champion's pass asset is missing")]
    MissingChampionsPassAsset,
    #[msg("Champion's pass link is missing")]
    MissingChampionsPassLink,
    #[msg("Randomness already revealed")]
    RandomnessAlreadyRevealed,
}
