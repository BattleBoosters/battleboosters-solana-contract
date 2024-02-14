use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum CollectionType {
    Energy = 0,
    Shield = 1,
    Points = 2,
    Fighter = 3,
    ChampionsPass = 4,
}
