use anchor_lang::prelude::*;
use anchor_spl::token::{InitializeMint, MintTo};
mod constants;
mod errors;
mod events;
mod processor;
mod state;
mod utils;

use errors::ErrorCode;

use crate::events::*;
use crate::state::event::*;
use crate::state::fight_card::*;
use crate::state::program::*;
// use crate::state::spl::*;
use crate::utils::*;

declare_id!("H85sU4mupXtsMZmtHM4y1Cucjfb7SVh7Q3eFrbZPX6a1");

#[program]
pub mod battleboosters {
    use super::*;
    use crate::constants;
    use crate::constants::{MINT_AUTHORITY, MY_APP_PREFIX};
    use anchor_lang::solana_program::program::invoke;
    use mpl_token_metadata::accounts::Metadata;
    use mpl_token_metadata::instructions::{
        BurnCpiBuilder, CreateMetadataAccountV3, CreateV1, CreateV1Builder, CreateV1CpiBuilder,
        MintV1CpiBuilder, TransferV1Cpi, TransferV1CpiAccounts, TransferV1InstructionArgs,
    };
    use mpl_token_metadata::programs::MPL_TOKEN_METADATA_ID;
    use mpl_token_metadata::types::MetadataDelegateRole::Collection;
    use mpl_token_metadata::types::{DataV2, PrintSupply, TokenStandard};
    // use crate::state::spl::InitializeEnergyBooster;

    pub fn initialize(
        ctx: Context<InitializeProgram>,
        authority_bump: u8,
        admin_pubkey: Pubkey,
        nft_fighter_pack_price: u64,
        booster_energy_price: u64,
        booster_shield_price: u64,
        booster_points_price: u64,
        fighter_pack_amount: u8,
    ) -> Result<()> {
        let program = &mut ctx.accounts.program;
        require!(!program.is_initialized, ErrorCode::AlreadyInitialized);

        program.authority_bump = authority_bump;
        program.event_counter = 0_u64;
        program.admin_pubkey = admin_pubkey;
        program.fighter_pack_price = nft_fighter_pack_price;
        program.booster_energy_price = booster_energy_price;
        program.booster_shield_price = booster_shield_price;
        program.booster_points_price = booster_points_price;
        program.fighter_pack_amount = fighter_pack_amount;
        program.is_initialized = true;

        let metadata_program = ctx.accounts.metadata_program.to_account_info();
        let authority = ctx.accounts.mint_authority.to_account_info();
        let payer = ctx.accounts.creator.to_account_info();
        let sysvar = ctx.accounts.sysvar_instructions.to_account_info();
        let spl_token_program = ctx.accounts.token_program.to_account_info();
        let metadata = ctx.accounts.metadata_energy_booster.to_account_info();
        let energy_minter = ctx.accounts.energy_minter.to_account_info();

        let mut binding = CreateV1CpiBuilder::new(&metadata_program);
        // let pda = mpl_token_metadata::accounts::Metadata::create_pda(mint.key(), 1).unwrap();
        let create_cpi = binding
            .metadata(&metadata)
            .mint(&energy_minter, false)
            .authority(&authority)
            .payer(&payer)
            .update_authority(&authority, true)
            .master_edition(Some(&ctx.accounts.master_edition_account_energy_booster))
            .system_program(&ctx.accounts.system_program)
            .sysvar_instructions(&sysvar)
            .spl_token_program(Some(&spl_token_program))
            .token_standard(TokenStandard::ProgrammableNonFungible)
            .name(String::from("My NFT X"))
            .uri("https://test.com".to_string())
            .seller_fee_basis_points(500)
            .is_mutable(true)
            .print_supply(PrintSupply::Unlimited);

        let bump: u8 = 252;
        let authority_seeds = [MY_APP_PREFIX, MINT_AUTHORITY, &[bump]];
        create_cpi.invoke_signed(&[&authority_seeds])?;

        Ok(())
    }

    // pub fn initialize_energy_booster(_ctx: Context<InitializeEnergyBooster>) -> Result<()>{
    //     Ok(())
    // }

    pub fn create_new_event(
        ctx: Context<CreateEvent>,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        let program = &mut ctx.accounts.program;
        only_admin(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

        // Create event account and set data
        let create_event = &mut ctx.accounts.event;
        create_event.fight_card_id_counter = 0_u8;
        create_event.start_date = start_date;
        create_event.end_date = end_date;

        emit!(EventCreated {
            event_id: program.event_counter
        });

        // Increment event counter
        program.event_counter += 1_u64;

        Ok(())
    }

    pub fn update_event(
        ctx: Context<UpdateEvent>,
        event_id: u64,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        let program = &ctx.accounts.program;
        only_admin(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

        let update_event = &mut ctx.accounts.event;
        update_event.start_date = start_date;
        update_event.end_date = end_date;

        emit!(EventUpdated {
            event_id: program.event_counter
        });

        Ok(())
    }

    pub fn create_new_fight_card(
        ctx: Context<CreateFightCard>,
        params: FightCardData,
    ) -> Result<()> {
        let program = &ctx.accounts.program;
        only_admin(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

        let fight_card = &mut ctx.accounts.fight_card;
        set_fight_card_properties(fight_card, &params);

        let event = &mut ctx.accounts.event;
        event.fight_card_id_counter = event.fight_card_id_counter.checked_add(1_u8).unwrap();

        emit!(FightCardCreated {
            fight_card_id: fight_card.id
        });

        Ok(())
    }

    pub fn update_fight_card(
        ctx: Context<UpdateFightCard>,
        fight_card_id: u8,
        params: FightCardData,
    ) -> Result<()> {
        let program = &ctx.accounts.program;
        only_admin(&ctx.accounts.creator.key(), &program.admin_pubkey)?;

        let fight_card = &mut ctx.accounts.fight_card;
        set_fight_card_properties(fight_card, &params);

        emit!(FightCardUpdated {
            fight_card_id: fight_card.id
        });

        Ok(())
    }

    /*
       @params: Tournament id, Card type (Main card, prelims, early prelims),
       TODO: Register to tournament

    */

    /*
       TODO: Claim event reward
    */

    /*
        TODO: Purchase NFT
            - Integration with Pyth Oracle or Switchboard for price feeds Sol/Usd conversion
    */

    /*
       TODO: Deposit NFT to my collection
    */

    /*
       TODO: Withdraw NFT from my collection
    */
}
