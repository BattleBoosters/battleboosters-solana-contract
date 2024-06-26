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
import createNftCollection from './utils/createNftCollection';

describe.skip('Collector pack', () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);

    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;
    let switchboardProgram;
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

    it('Mint an nft', async () => {
        //const player = anchor.web3.Keypair.generate();
        //console.log(player.secretKey)
        //await airdrop_sol(provider, player.publicKey, 1);

        let programPDA = await program.account.programData.fetch(program_pda);

        const [minter] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mint'),
                new BN(programPDA.mintableGameAssetNonce).toBuffer('le', 8),
            ],
            program.programId
        );
        const [master_edition] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('metadata'),
                metadata_pubkey.toBuffer(),
                minter.toBuffer(),
                Buffer.from('edition'),
            ],
            metadata_pubkey
        );

        const [metadata] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('metadata'),
                metadata_pubkey.toBuffer(),
                minter.toBuffer(),
            ],
            metadata_pubkey
        );

        const [energy_minter] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mint'),
                Buffer.from([0]),
            ],
            program.programId
        );

        const [energy_metadata] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('metadata'),
                metadata_pubkey.toBuffer(),
                energy_minter.toBuffer(),
            ],
            metadata_pubkey
        );

        const [energy_master_edition] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('metadata'),
                    metadata_pubkey.toBuffer(),
                    energy_minter.toBuffer(),
                    Buffer.from('edition'),
                ],
                metadata_pubkey
            );

        const [shield_minter] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mint'),
                Buffer.from([1]),
            ],
            program.programId
        );

        const [shield_metadata] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('metadata'),
                metadata_pubkey.toBuffer(),
                shield_minter.toBuffer(),
            ],
            metadata_pubkey
        );

        const [shield_master_edition] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('metadata'),
                    metadata_pubkey.toBuffer(),
                    shield_minter.toBuffer(),
                    Buffer.from('edition'),
                ],
                metadata_pubkey
            );
        const [points_minter] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mint'),
                Buffer.from([2]),
            ],
            program.programId
        );

        const [points_metadata] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('metadata'),
                metadata_pubkey.toBuffer(),
                points_minter.toBuffer(),
            ],
            metadata_pubkey
        );

        const [points_master_edition] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('metadata'),
                    metadata_pubkey.toBuffer(),
                    points_minter.toBuffer(),
                    Buffer.from('edition'),
                ],
                metadata_pubkey
            );

        const [fighter_minter] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mint'),
                Buffer.from([3]),
            ],
            program.programId
        );

        const [fighter_metadata] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('metadata'),
                metadata_pubkey.toBuffer(),
                fighter_minter.toBuffer(),
            ],
            metadata_pubkey
        );

        const [fighter_master_edition] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('metadata'),
                    metadata_pubkey.toBuffer(),
                    fighter_minter.toBuffer(),
                    Buffer.from('edition'),
                ],
                metadata_pubkey
            );

        let token_account = anchor.utils.token.associatedAddress({
            mint: minter,
            owner: admin_account.publicKey,
        });

        const [token_record] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('metadata'),
                metadata_pubkey.toBuffer(),
                minter.toBuffer(),
                Buffer.from('token_record'),
                token_account.toBuffer(),
            ],
            metadata_pubkey
        );

        try {
            const tx = await program.methods
                .mintCollectorPack()
                .accounts({
                    creator: admin_account.publicKey,
                    program: program_pda,
                    mintAuthority: mint_authority_account,
                    energyMinter: energy_minter,
                    energyMetadata: energy_metadata,
                    energyMasterEdition: energy_master_edition,
                    minter: minter,
                    tokenAccount: token_account,
                    tokenRecord: token_record,
                    masterEdition: master_edition,
                    metadata: metadata,
                    sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                })
                .preInstructions([
                    anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
                        units: 1000000,
                    }),
                ])
                .signers([admin_account]) // Include new_account as a signer
                .rpc();

            const program_pda_data = await program.account.programData.fetch(
                program_pda
            );
            assert.equal(
                program_pda_data.mintableGameAssetNonce.eq(new BN(0)),
                true
            );
        } catch (e) {
            console.log(e);
        }
    });
});
