// src/errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // 0
    #[msg("Custom error message for an invalid operation")]
    InvalidOperation,
    // 1
    #[msg("Unauthorized access attempt")]
    Unauthorized,
    // 2
    #[msg("The mintable game asset link is not properly linked to the specified mintable game asset PDA")]
    GameAssetLinkNotLinkedToAssetPDA,
    // 3
    #[msg("Already initialized")]
    AlreadyInitialized,
    // 4
    #[msg("The provided NFT type is not supported for this operation")]
    UnsupportedNftType,
    // 5
    #[msg("Invalid Price Feed")]
    InvalidPriceFeed,
    // 6
    #[msg("Feed has not been updated in 5 minutes")]
    StaleFeed,
    // 7
    #[msg("Insufficient funds")]
    InsufficientFunds,
    // 8
    #[msg("Insufficient amount in purchase request")]
    InsufficientAmount,
    // 9
    #[msg("Randomness unavailable for now")]
    RandomnessUnavailable,
    // 10
    #[msg("No matching rarity found")]
    NoMatchingRarityFound,
    // 11
    #[msg("The nonce must not exceed the last known nonce in the player's state")]
    WrongPlayerGameAssetLinkNonce,
    // 12
    #[msg("This player game asset pda is not free")]
    NotFreePDA,
    // 13
    #[msg("Not enough allowance to generate mintable game asset")]
    NotEnoughAllowance,
    // 14
    #[msg("The event has already started")]
    EventAlreadyStarted,
    // 15
    #[msg("The event is still in progress. Please try again after it concludes on approximately")]
    EventStillRunning,
    // 16
    #[msg("Attach rarity account to this transaction")]
    RarityAccountRequired,
    // 17
    #[msg("Fight card link already has a game asset, or game asset nonce is missing")]
    FightCardLinkedToGameAsset,
    // 18
    #[msg("Event card link already has a game asset, or game asset nonce is missing")]
    EventLinkedToGameAsset,
    // 19
    #[msg("Booster type not found")]
    BoosterTypeNotFound,
    // 20
    #[msg("Champion's pass not required for non-main card events")]
    NonMainCardEvent,
    // 21
    #[msg("This mintable game asset has no owner")]
    MintableAssetHasNoOwner,
    // 22
    #[msg("This mintable game asset is burnt")]
    MintableAssetBurned,
    // 23
    #[msg("This mintable game asset is locked")]
    MintableAssetLocked,
    // 24
    #[msg("This mintable game asset has been minted as an NFT and is no longer available for in-game use")]
    MintableAssetMintedAndUnavailable,
    // 25
    #[msg("This mintable game asset link is missing")]
    MintableAssetLinkRequired,
    // 26
    #[msg("The probability tier was not found")]
    ProbabilityTierNotFound,
    // 27
    #[msg("Rank point is required")]
    RankPointsIsNone,
    // 28
    #[msg("Rank is required")]
    RankIsNone,
    // 29
    #[msg("The requested operation has already been consumed")]
    ConsumedAlready,
    // 30
    #[msg(
        "Randomness is required to collect your reward. Please request randomness and try again."
    )]
    RandomnessIsNone,
    // 31
    #[msg("Failed to parse value")]
    FailedToParseValue,
    // 32
    #[msg("Champion's pass asset is missing")]
    MissingChampionsPassAsset,
    // 33
    #[msg("Champion's pass link is missing")]
    MissingChampionsPassLink,
    // 34
    #[msg("Randomness already revealed")]
    RandomnessAlreadyRevealed,
    // 35
    #[msg("Randomness is not yet resolved")]
    RandomnessNotResolved,
}
