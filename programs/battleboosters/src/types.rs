use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum CollectionType {
    Energy = 0,
    Shield = 1,
    Points = 2,
    Fighter = 3,
    ChampionsPass = 4,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct PurchaseRequest {
    pub nft_type: NftType,
    pub quantity: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub enum NftType {
    Booster,
    FighterPack,
}
