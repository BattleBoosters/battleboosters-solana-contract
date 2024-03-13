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
import { RandomnessService } from '@switchboard-xyz/solana-randomness-service';
import * as buffer from 'buffer';
import account_init from './utils/initAccounts';
import createNftCollection from './utils/createNftCollection';
const { Metadata } = require('@metaplex-foundation/mpl-token-metadata');

describe('Create NFT Collection', () => {
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

    it('Admin successfully create Energy NFT Collection', async () => {
        try {
            let { minter, metadata, master_edition, token_record } =
                await createNftCollection(
                    program,
                    provider,
                    program_pda,
                    metadata_pubkey,
                    mint_authority_account,
                    admin_account
                );

            // Fetch the account info
            const metadataAccountInfo =
                await provider.connection.getAccountInfo(metadata);
            if (!metadataAccountInfo) {
                throw new Error('Failed to find metadata account');
            }

            // // Decode the metadata
            // const metadata_data = Metadata.fromAccountInfo(metadataAccountInfo);
            // console.log("Metadata URI:", metadata_data.data.uri);
            // console.log("Metadata Name:", metadata_data.data.name);
            // console.log("Metadata Symbol:", metadata_data.data.symbol);

            // const program_pda_data = await program.account.m.fetch(
            //     metadata
            // );
        } catch (e) {
            console.log(e);
        }
    });
});
