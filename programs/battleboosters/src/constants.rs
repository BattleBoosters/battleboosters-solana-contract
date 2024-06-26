pub const MY_APP_PREFIX: &[u8] = b"BattleBoosters";
pub const PROGRAM_STATE: &[u8] = b"program";
pub const EVENT: &[u8] = b"event";
pub const FIGHT_CARD: &[u8] = b"fightCard";
pub const RARITY: &[u8] = b"rarity";
pub const FIGHTER_BASE: &[u8] = b"fighterBase";
pub const MINT_AUTHORITY: &[u8] = b"mintAuthority";
pub const BANK: &[u8] = b"bank";
pub const RANK: &[u8] = b"rank";
pub const MINT: &[u8] = b"mint";
//pub const INVENTORY: &[u8] = b"inventory";
pub const PLAYER: &[u8] = b"player";
pub const MYSTERY_BOX: &[u8] = b"mysteryBox";
pub const MINTABLE_GAME_ASSET: &[u8] = b"mintableGameAsset";
pub const METADATA_OFF_CHAIN_URI: &str = "https://battleboosters.com/api/metadata";
// pub const NFT_NAME: &str = "Fighter";
pub const PRICE_DECIMALS: u64 = 1_000_000;

/// Switchboard
pub const STALENESS_THRESHOLD: u64 = 60;
pub const _SOL_USD_FEED_MAINNET: &str = "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR";
/// PYTH
pub const FEED_HEX: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
// /// Seller BasePoint fees
// pub const SELLER_FEE: u16 = 500;
