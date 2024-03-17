import * as anchor from '@coral-xyz/anchor';
import { BN, Program } from '@coral-xyz/anchor';
import { Battleboosters } from '../target/types/battleboosters';
import { assert } from 'chai';
import account_init from './utils/initAccounts';
import { joinFightCard } from './utils/joinFightCard';
import createMintableGameAsset from './utils/createMintableGameAsset';
import InitializePlayerAccount from './utils/initializePlayerAccount';
describe('Join fight card', () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);
    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;

    const {
        admin_account,
        unauthorized_account,
        metadata_pubkey,
        bank_pda,
        bank_bump,
        program_pda,
        program_bump,
        rarity_pda,
        rarity_bump,
        mint_authority_account,
        authority_bump,
    } = account_init(program);

    // before( async () => {
    //
    //         await InitializePlayerAccount(
    //             provider,
    //             admin_account.publicKey,
    //             program,
    //             program_pda
    //         );
    //         const [player_account_pda, player_account_bump] =
    //             anchor.web3.PublicKey.findProgramAddressSync(
    //                 [
    //                     Buffer.from('BattleBoosters'),
    //                     Buffer.from('player'),
    //                     admin_account.publicKey.toBuffer(),
    //                 ],
    //                 program.programId
    //             );
    //
    //         const player_account_pda_data = await program.account.playerData.fetch(
    //             player_account_pda
    //         );
    //         const [collector_pack_pda, collector_pack_bump] =
    //             anchor.web3.PublicKey.findProgramAddressSync(
    //                 [
    //                     Buffer.from('BattleBoosters'),
    //                     Buffer.from('collector'),
    //                     admin_account.publicKey.toBuffer(),
    //                     new BN(player_account_pda_data.orderNonce).toBuffer(
    //                         'le',
    //                         8
    //                     ),
    //                 ],
    //                 program.programId
    //             );
    //
    //         const tx = await program.methods
    //             .testGiftCollectorPack()
    //             .accounts({
    //                 signer: provider.wallet.publicKey,
    //                 recipient: admin_account.publicKey,
    //                 program: program_pda,
    //                 playerAccount: player_account_pda,
    //                 collectorPack: collector_pack_pda,
    //             })
    //             .signers([])
    //             .rpc();
    //         await createMintableGameAsset(program, provider, program_pda, {
    //             nftType: { fighter: {} },
    //         }, rarity_pda, null, admin_account)
    //
    // })

    it('Should join a new fight card', async () => {
        let {
            event_account,
            event_link_account,
            fight_card_account,
            fight_card_link_account,
            player_account_pda,
            fighter_mintable_game_asset_pda,
            fighter_mintable_game_asset_link_pda,
        } = await joinFightCard(
            provider,
            program,
            admin_account,
            program_pda,
            {
                fighterBlue: {},
            },
            0,
            0,
            0,
            0,
            0
        );

        const fighter_mintable_game_asset_pda_data =
            await program.account.mintableGameAssetData.fetch(
                fighter_mintable_game_asset_pda
            );
        assert.equal(fighter_mintable_game_asset_pda_data.isLocked, true);
        assert.equal(fighter_mintable_game_asset_pda_data.isBurned, false);
        assert.equal(fighter_mintable_game_asset_pda_data.isMinted, false);
        assert.deepEqual(
            fighter_mintable_game_asset_pda_data.owner,
            provider.wallet.publicKey
        );

        const fighter_mintable_game_asset_link_pda_data =
            await program.account.playerGameAssetLinkData.fetch(
                fighter_mintable_game_asset_link_pda
            );
        assert.equal(fighter_mintable_game_asset_link_pda_data.isFree, false);
        assert.deepEqual(
            fighter_mintable_game_asset_link_pda_data.mintableGameAssetPubkey,
            fighter_mintable_game_asset_pda
        );

        const fight_card_link_account_data =
            await program.account.fightCardLinkData.fetch(
                fight_card_link_account
            );
        assert.equal(fight_card_link_account_data.isConsumed, false);
        assert.equal(fight_card_link_account_data.isInitialized, true);
        assert.deepEqual(fight_card_link_account_data.fighterColorSide, {
            fighterBlue: {},
        });
        assert.deepEqual(
            fight_card_link_account_data.creator,
            provider.wallet.publicKey
        );
        assert.deepEqual(
            fight_card_link_account_data.fighterUsed,
            fighter_mintable_game_asset_pda
        );
        assert.deepEqual(
            fight_card_link_account_data.fighterNonceTracker.eq(new BN(0)),
            true
        );
        assert.deepEqual(
            fight_card_link_account_data.fightCardPubkey,
            fight_card_account
        );
        assert.equal(fight_card_link_account_data.fightCardNonceTracker, 0);
        assert.equal(fight_card_link_account_data.energyBoosterUsed, null);
        assert.equal(
            fight_card_link_account_data.energyBoosterNonceTracker,
            null
        );
        assert.equal(fight_card_link_account_data.shieldBoosterUsed, null);
        assert.equal(
            fight_card_link_account_data.shieldBoosterNonceTracker,
            null
        );
        assert.equal(fight_card_link_account_data.pointsBoosterUsed, null);
        assert.equal(
            fight_card_link_account_data.pointsBoosterNonceTracker,
            null
        );

        const player_account_pda_data = await program.account.playerData.fetch(
            player_account_pda
        );
        assert.equal(player_account_pda_data.isInitialized, true);
        assert.deepEqual(
            player_account_pda_data.orderNonce.eq(new BN(0)),
            true
        );
        assert.deepEqual(
            player_account_pda_data.playerGameAssetLinkNonce.eq(new BN(3)),
            true
        );

        const event_link_account_data =
            await program.account.eventLinkData.fetch(event_link_account);
        assert.deepEqual(
            event_link_account_data.creator,
            provider.wallet.publicKey
        );
        assert.equal(event_link_account_data.championsPassNonceTracker, null);
        assert.equal(event_link_account_data.championsPassPubkey, null);
        assert.deepEqual(event_link_account_data.eventPubkey, event_account);
        assert.deepEqual(
            event_link_account_data.eventNonceTracker.eq(new BN(0)),
            true
        );
    });
    it('Should fail joining the same fight card', async () => {
        try {
            let {
                event_account,
                event_link_account,
                fight_card_account,
                fight_card_link_account,
                player_account_pda,
                fighter_mintable_game_asset_pda,
                fighter_mintable_game_asset_link_pda,
            } = await joinFightCard(
                provider,
                program,
                admin_account,
                program_pda,
                {
                    fighterBlue: {},
                },
                0,
                1,
                1,
                1
            );

            // const fighter_mintable_game_asset_pda_data = await program.account.mintableGameAssetData.fetch(
            //     fighter_mintable_game_asset_pda
            // );
            // console.log(fighter_mintable_game_asset_pda_data.isLocked)
        } catch (e) {
            console.log(e);
        }
    });
});
