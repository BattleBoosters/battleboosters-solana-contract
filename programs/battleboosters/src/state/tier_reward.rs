/*
    TODO: Initialize tier reward account 
 */
// #[derive(Accounts)]
// pub struct InitializeTierReward<'info> {
//     
// }

/*
      TODO: User tier as reward structure
*/
// Define a struct to hold the rewards for each tier.

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
// pub struct TierRewards {
//     prize_amount: u64,          // Currency or token reward
//     fighter_amount: i16,        // Quantities of fighter in-game assets awarded
//     booster_amount: i16,        // Quantities of booster in-game assets awarded
//     champions_pass_amount: i16, // Quantities of champions pass in-game assets awarded
// }
// 
// // Enum to represent the different tiers.
// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
// pub enum Tier {
//     Tier1,
//     Tier2,
//     Tier3,
//     Tier4,
//     Tier5,
// }
// 
// // Struct to link tiers with their corresponding rewards.
// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
// pub struct TierReward {
//     tier: Tier,
//     rewards: TierRewards,
// }
