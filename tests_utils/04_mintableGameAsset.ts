import * as anchor from '@coral-xyz/anchor';
import { BN, Program, web3 } from '@coral-xyz/anchor';
import { Battleboosters } from '../target/types/battleboosters';
import { assert, expect } from 'chai';
import airdropSol from './utils/airdropSol';
import {
    TOKEN_PROGRAM_ID,
    AccountLayout,
    MintLayout,
    ASSOCIATED_TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
const { SystemProgram, SYSVAR_RENT_PUBKEY } = anchor.web3;
import {
    mplTokenMetadata,
    getMplTokenMetadataProgramId,
} from '@metaplex-foundation/mpl-token-metadata';
import { MPL_TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import {
    Connection,
    LAMPORTS_PER_SOL,
    PublicKey,
    Transaction,
} from '@solana/web3.js';
import { before } from 'mocha';
import airdrop_sol from './utils/airdropSol';
import { sleep } from '@switchboard-xyz/common';
import {
    AggregatorAccount,
    SwitchboardProgram,
} from '@switchboard-xyz/solana.js';
import InitializePlayerAccount from './utils/initializePlayerAccount';
import * as buffer from 'buffer';
import account_init from './utils/initAccounts';
import createMintableGameAsset from './utils/createMintableGameAsset';
import purchaseMysteryBox from './utils/purchaseMysteryBox';

describe('Mintable Game Asset', () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);
    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;

    const {
        admin_account,
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
    const newRecipient = anchor.web3.Keypair.generate();

    before(async () => {
        try {
            await InitializePlayerAccount(
                provider,
                provider.wallet.publicKey,
                program
            );

            // Initialize the player account first
            await InitializePlayerAccount(
                provider,
                newRecipient.publicKey,
                //provider.wallet.publicKey,
                program
            );
            await airdrop_sol(provider, newRecipient.publicKey, 1);
            await purchaseMysteryBox(
                program,
                provider,
                program_pda,
                rarity_pda,
                bank_pda,
                new BN(100),
                provider.wallet,
                3,
                1
            );

            // const tx = await program.methods
            //     .adminAirdropCollectorPack(new BN(3), new BN(2), new BN(0))
            //     .accounts({
            //         signer: admin_account.publicKey,
            //         recipient: provider.wallet.publicKey,
            //         program: program_pda,
            //         playerAccount: player_account_pda,
            //         mysteryBox: mystery_box_pda,
            //         rarity: rarity_pda,
            //         systemProgram: anchor.web3.SystemProgram.programId,
            //     })
            //     .signers([admin_account])
            //     .rpc();
            //
            // const mystery_box_pda_data =
            //     await program.account.mysteryBoxData.fetch(mystery_box_pda);
            // assert.isTrue(
            //     mystery_box_pda_data.boosterMintAllowance.eq(new BN(3))
            // );
            // assert.isTrue(
            //     mystery_box_pda_data.fighterMintAllowance.eq(new BN(2))
            // );
        } catch (e) {
            console.log(e);
        }
    });
    it('Open a fighter from mystery box randomly', async () => {
        try {
            const program_pda_data_before =
                await program.account.programData.fetch(program_pda);
            assert.equal(
                program_pda_data_before.mintableGameAssetNonce.eq(new BN(0)),
                true
            );

            let {
                mystery_box_pda,
                mintable_game_asset_pda,
                player_game_asset_link_pda,
                player_account_pda,
            } = await createMintableGameAsset(
                program,
                provider,
                program_pda,
                {
                    nftType: { fighter: {} },
                },
                rarity_pda,
                null,
                '',
                0,
                provider.wallet.publicKey,
                null
            );

            const mystery_box_pda_data =
                await program.account.mysteryBoxData.fetch(mystery_box_pda);
            assert.isTrue(
                mystery_box_pda_data.boosterMintAllowance.eq(new BN(3))
            );
            assert.isTrue(
                mystery_box_pda_data.fighterMintAllowance.eq(new BN(4))
            );
            //const pre_mint_pda_data = await program.account.mintableGameAssetData.fetch(mintable_game_asset_pda);
            const mintable_game_asset_pda_data =
                await program.account.mintableGameAssetData.fetch(
                    mintable_game_asset_pda
                );
            const attribute = [
                {
                    traitType: 'Fighter Type',
                    value: 'Taekwondo',
                },
                {
                    traitType: 'Rarity',
                    value: 'Uncommon',
                },
                {
                    traitType: 'Power',
                    value: '194',
                },
                {
                    traitType: 'Maximum Lifespan',
                    value: '192',
                },
                {
                    traitType: 'Lifespan',
                    value: '192',
                },
            ];
            assert.isFalse(mintable_game_asset_pda_data.isLocked);
            assert.isFalse(mintable_game_asset_pda_data.isMinted);
            assert.equal(mintable_game_asset_pda_data.metadata.name, 'Fighter');
            assert.equal(
                mintable_game_asset_pda_data.metadata.description,
                'test'
            );
            assert.equal(
                mintable_game_asset_pda_data.metadata.externalUrl,
                `https://battleboosters.com/api/metadata/${mintable_game_asset_pda}`
            );
            assert.isNull(mintable_game_asset_pda_data.metadata.animationUrl);
            assert.isNull(mintable_game_asset_pda_data.metadata.image);
            assert.deepEqual(
                mintable_game_asset_pda_data.metadata.attributes,
                attribute
            );

            const player_game_asset_link_pda_data =
                await program.account.mintableGameAssetLinkData.fetch(
                    player_game_asset_link_pda
                );
            assert.isFalse(player_game_asset_link_pda_data.isFree);
            assert.equal(
                player_game_asset_link_pda_data.mintableGameAssetNonceTracker.eq(
                    new BN(0)
                ),
                true
            );
            assert.deepEqual(
                player_game_asset_link_pda_data.mintableGameAssetPubkey,
                mintable_game_asset_pda
            );

            const program_pda_data_after =
                await program.account.programData.fetch(program_pda);
            assert.equal(
                program_pda_data_after.mintableGameAssetNonce.eq(new BN(1)),
                true
            );

            const player_pda_data = await program.account.playerData.fetch(
                player_account_pda
            );
            assert.equal(
                player_pda_data.playerGameAssetLinkNonce.eq(new BN(1)),
                true
            );
        } catch (e) {
            console.log(e);
        }
    });

    it('Open a second fighter from mystery box randomly', async () => {
        try {
            let {
                mystery_box_pda,
                mintable_game_asset_pda,
                player_game_asset_link_pda,
                player_account_pda,
            } = await createMintableGameAsset(
                program,
                provider,
                program_pda,
                {
                    nftType: { fighter: {} },
                },
                rarity_pda,
                null,
                '',
                0,
                provider.wallet.publicKey,
                null
            );
            const mystery_box_pda_data =
                await program.account.mysteryBoxData.fetch(mystery_box_pda);
            assert.isTrue(
                mystery_box_pda_data.boosterMintAllowance.eq(new BN(3))
            );

            assert.isTrue(
                mystery_box_pda_data.fighterMintAllowance.eq(new BN(3))
            );

            const mintable_game_asset_pda_data =
                await program.account.mintableGameAssetData.fetch(
                    mintable_game_asset_pda
                );
            const attribute = [
                {
                    traitType: 'Fighter Type',
                    value: 'Wrestling',
                },
                {
                    traitType: 'Rarity',
                    value: 'Common',
                },
                {
                    traitType: 'Power',
                    value: '102',
                },
                {
                    traitType: 'Maximum Lifespan',
                    value: '111',
                },
                {
                    traitType: 'Lifespan',
                    value: '111',
                },
            ];
            assert.isFalse(mintable_game_asset_pda_data.isLocked);
            assert.isFalse(mintable_game_asset_pda_data.isMinted);
            assert.equal(mintable_game_asset_pda_data.metadata.name, 'Fighter');
            assert.equal(
                mintable_game_asset_pda_data.metadata.description,
                'test'
            );
            assert.isNull(mintable_game_asset_pda_data.metadata.image);
            assert.isNull(mintable_game_asset_pda_data.metadata.animationUrl);
            assert.equal(
                mintable_game_asset_pda_data.metadata.externalUrl,
                `https://battleboosters.com/api/metadata/${mintable_game_asset_pda}`
            );

            assert.deepEqual(
                mintable_game_asset_pda_data.metadata.attributes,
                attribute
            );

            const player_game_asset_link_pda_data =
                await program.account.mintableGameAssetLinkData.fetch(
                    player_game_asset_link_pda
                );
            assert.isFalse(player_game_asset_link_pda_data.isFree);
            assert.equal(
                player_game_asset_link_pda_data.mintableGameAssetNonceTracker.eq(
                    new BN(1)
                ),
                true
            );
            assert.deepEqual(
                player_game_asset_link_pda_data.mintableGameAssetPubkey,
                mintable_game_asset_pda
            );

            const program_pda_data_after =
                await program.account.programData.fetch(program_pda);
            assert.equal(
                program_pda_data_after.mintableGameAssetNonce.eq(new BN(2)),
                true
            );

            const player_pda_data = await program.account.playerData.fetch(
                player_account_pda
            );
            assert.equal(
                player_pda_data.playerGameAssetLinkNonce.eq(new BN(2)),
                true
            );
        } catch (e) {
            console.log(e);
        }
    });

    it('Open a booster from mystery box randomly', async () => {
        try {
            let {
                mystery_box_pda,
                mintable_game_asset_pda,
                player_game_asset_link_pda,
                player_account_pda,
            } = await createMintableGameAsset(
                program,
                provider,
                program_pda,
                {
                    nftType: { booster: {} },
                },
                rarity_pda,
                null,
                '',
                0,
                provider.wallet.publicKey,
                null
            );
            const mystery_box_pda_data =
                await program.account.mysteryBoxData.fetch(mystery_box_pda);
            assert.isTrue(
                mystery_box_pda_data.boosterMintAllowance.eq(new BN(2))
            );

            assert.isTrue(
                mystery_box_pda_data.fighterMintAllowance.eq(new BN(3))
            );

            const mintable_game_asset_pda_data =
                await program.account.mintableGameAssetData.fetch(
                    mintable_game_asset_pda
                );
            const attribute = [
                { traitType: 'Booster Type', value: 'Points' },
                { traitType: 'Rarity', value: 'Epic' },
                { traitType: 'Value', value: '274' },
            ];
            assert.isFalse(mintable_game_asset_pda_data.isLocked);
            assert.isFalse(mintable_game_asset_pda_data.isMinted);
            assert.equal(mintable_game_asset_pda_data.metadata.name, 'Booster');
            assert.equal(
                mintable_game_asset_pda_data.metadata.description,
                'test'
            );
            assert.equal(
                mintable_game_asset_pda_data.metadata.externalUrl,
                `https://battleboosters.com/api/metadata/${mintable_game_asset_pda}`
            );
            assert.isNull(mintable_game_asset_pda_data.metadata.animationUrl);
            assert.isNull(mintable_game_asset_pda_data.metadata.image);
            assert.deepEqual(
                mintable_game_asset_pda_data.metadata.attributes,
                attribute
            );

            const player_game_asset_link_pda_data =
                await program.account.mintableGameAssetLinkData.fetch(
                    player_game_asset_link_pda
                );
            assert.isFalse(player_game_asset_link_pda_data.isFree);
            assert.equal(
                player_game_asset_link_pda_data.mintableGameAssetNonceTracker.eq(
                    new BN(2)
                ),
                true
            );
            assert.deepEqual(
                player_game_asset_link_pda_data.mintableGameAssetPubkey,
                mintable_game_asset_pda
            );

            const program_pda_data_after =
                await program.account.programData.fetch(program_pda);
            assert.equal(
                program_pda_data_after.mintableGameAssetNonce.eq(new BN(3)),
                true
            );

            const player_pda_data = await program.account.playerData.fetch(
                player_account_pda
            );
            assert.equal(
                player_pda_data.playerGameAssetLinkNonce.eq(new BN(3)),
                true
            );
        } catch (e) {
            console.log(e);
        }
    });

    it('Fail minting allowance too low trying to mint fighter from mystery box', async () => {
        let transactionFailed = false;
        try {
            for (let i = 0; i < 4; i++) {
                await createMintableGameAsset(
                    program,
                    provider,
                    program_pda,
                    { nftType: { fighter: {} } },
                    rarity_pda,
                    null,
                    '',
                    0,
                    provider.wallet.publicKey,
                    null
                );
            }
        } catch (e) {
            transactionFailed = true;
            assert.include(
                e.message,
                'Not enough allowance to generate mintable game asset.'
            );
        }
        assert.isTrue(transactionFailed);
    });

    it('Fail trying to reuse a nonce which is not free', async () => {
        let transactionFailed = false;
        try {
            await createMintableGameAsset(
                program,
                provider,
                program_pda,
                {
                    nftType: { booster: {} },
                },
                rarity_pda,
                1,
                '',
                0,
                provider.wallet.publicKey,
                null
            );
        } catch (e) {
            transactionFailed = true;
            assert.include(
                e.message,
                'This player game asset pda is not free.'
            );
        }
        assert.isTrue(transactionFailed);
    });

    it('Fail trying to create a nonce greater than the player game asset link nonce', async () => {
        let transactionFailed = false;
        try {
            await createMintableGameAsset(
                program,
                provider,
                program_pda,
                {
                    nftType: { booster: {} },
                },
                rarity_pda,
                10,
                '',
                0,
                provider.wallet.publicKey,
                null
            );
        } catch (e) {
            transactionFailed = true;
            assert.include(
                e.message,
                "The nonce must not exceed the last known nonce in the player's state."
            );
        }
        assert.isTrue(transactionFailed);
    });
    /*
        TODO Test:
            - Try Override a PDA which have been free and check the nonce haven't moved.


     */
});
