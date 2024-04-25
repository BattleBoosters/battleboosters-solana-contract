import * as anchor from '@coral-xyz/anchor';
import { BN, Program, web3 } from '@coral-xyz/anchor';
import { Battleboosters } from '../target/types/battleboosters';
import { assert } from 'chai';
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

describe.skip('Mintable Game Asset', () => {
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

    before(async () => {
        try {
            await InitializePlayerAccount(
                provider,
                provider.wallet.publicKey,
                program
            );
            const [player_account_pda, player_account_bump] =
                anchor.web3.PublicKey.findProgramAddressSync(
                    [
                        Buffer.from('BattleBoosters'),
                        Buffer.from('player'),
                        provider.wallet.publicKey.toBuffer(),
                    ],
                    program.programId
                );

            const player_account_pda_data =
                await program.account.playerData.fetch(player_account_pda);
            console.log('player_account_pda_data.orderNonce');
            console.log(player_account_pda_data.orderNonce);
            const [mystery_box_pda, mystery_box_bump] =
                anchor.web3.PublicKey.findProgramAddressSync(
                    [
                        Buffer.from('BattleBoosters'),
                        Buffer.from('mysteryBox'),
                        provider.wallet.publicKey.toBuffer(),
                        new BN(player_account_pda_data.orderNonce).toBuffer(
                            'le',
                            8
                        ),
                    ],
                    program.programId
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
                mystery_box_pda_data.fighterMintAllowance.eq(new BN(1))
            );
            //const pre_mint_pda_data = await program.account.mintableGameAssetData.fetch(mintable_game_asset_pda);
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
                    value: 'Epic',
                },
                {
                    traitType: 'Energy',
                    value: '269',
                },
                {
                    traitType: 'Power',
                    value: '256',
                },
                {
                    traitType: 'Lifespan',
                    value: '288',
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
                mintable_game_asset_pda_data.metadata.image,
                `https://battleboosters.com/metadata/${mintable_game_asset_pda}`
            );
            assert.isNull(mintable_game_asset_pda_data.metadata.animationUrl);
            assert.isNull(mintable_game_asset_pda_data.metadata.externalUrl);
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
        const mystery_box_pda_data = await program.account.mysteryBoxData.fetch(
            mystery_box_pda
        );
        assert.isTrue(mystery_box_pda_data.boosterMintAllowance.eq(new BN(3)));

        assert.isTrue(mystery_box_pda_data.fighterMintAllowance.eq(new BN(0)));

        const mintable_game_asset_pda_data =
            await program.account.mintableGameAssetData.fetch(
                mintable_game_asset_pda
            );
        const attribute = [
            {
                traitType: 'Fighter Type',
                value: 'MuayThai',
            },
            {
                traitType: 'Rarity',
                value: 'Uncommon',
            },
            {
                traitType: 'Energy',
                value: '171',
            },
            {
                traitType: 'Power',
                value: '181',
            },
            {
                traitType: 'Lifespan',
                value: '195',
            },
        ];
        assert.isFalse(mintable_game_asset_pda_data.isLocked);
        assert.isFalse(mintable_game_asset_pda_data.isMinted);
        assert.equal(mintable_game_asset_pda_data.metadata.name, 'Fighter');
        assert.equal(mintable_game_asset_pda_data.metadata.description, 'test');
        assert.isNull(mintable_game_asset_pda_data.metadata.image);
        assert.isNull(mintable_game_asset_pda_data.metadata.animationUrl);
        assert.equal(
            mintable_game_asset_pda_data.metadata.externalUrl,
            `https://battleboosters.com/api/metadata/${mintable_game_asset_pda}`
        );
        console.log(mintable_game_asset_pda_data.metadata.attributes);
        // assert.deepEqual(
        //     mintable_game_asset_pda_data.metadata.attributes,
        //     attribute
        // );

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

        const program_pda_data_after = await program.account.programData.fetch(
            program_pda
        );
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
    });

    it('Fail minting allowance too low trying to mint fighter from mystery box', async () => {
        try {
            await createMintableGameAsset(
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
        } catch (e) {
            //console.log(e);
            assert.include(
                e.message,
                'Not enough allowance to generate mintable game asset'
            );
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
                mystery_box_pda_data.fighterMintAllowance.eq(new BN(0))
            );

            const mintable_game_asset_pda_data =
                await program.account.mintableGameAssetData.fetch(
                    mintable_game_asset_pda
                );
            const attribute = [
                { traitType: 'Booster Type', value: 'Energy' },
                { traitType: 'Rarity', value: 'Uncommon' },
                { traitType: 'Value', value: '175' },
            ];
            assert.isFalse(mintable_game_asset_pda_data.isLocked);
            assert.isFalse(mintable_game_asset_pda_data.isMinted);
            assert.equal(mintable_game_asset_pda_data.metadata.name, 'Booster');
            assert.equal(
                mintable_game_asset_pda_data.metadata.description,
                'test'
            );
            assert.equal(
                mintable_game_asset_pda_data.metadata.image,
                `https://battleboosters.com/metadata/${mintable_game_asset_pda}`
            );
            assert.isNull(mintable_game_asset_pda_data.metadata.animationUrl);
            assert.isNull(mintable_game_asset_pda_data.metadata.externalUrl);
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

    it('Fail trying to reuse a nonce which is not free', async () => {
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
            assert.include(
                e.message,
                'This player game asset pda is not free.'
            );
        }
    });

    it('Fail trying to create a nonce greater than the player game asset link nonce', async () => {
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
            assert.include(
                e.message,
                "The nonce must not exceed the last known nonce in the player's state."
            );
        }
    });
    /*
        TODO Test:
            - Try Override a PDA which have been free and check the nonce haven't moved.


     */
});
