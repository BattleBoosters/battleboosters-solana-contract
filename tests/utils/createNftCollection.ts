import * as anchor from '@coral-xyz/anchor';
import { BN } from '@coral-xyz/anchor';
import { sleep } from '@switchboard-xyz/common';
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import { MPL_TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';

const createNftCollection = async function (
    program,
    provider,
    program_pda,
    metadata_pubkey,
    mint_authority_account,
    admin_account
) {
    const [minter] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from('BattleBoosters'), Buffer.from('mint'), Buffer.from([0])],
        program.programId
    );

    const [metadata] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from('metadata'),
            metadata_pubkey.toBuffer(),
            minter.toBuffer(),
        ],
        metadata_pubkey
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

    let token_account = anchor.utils.token.associatedAddress({
        mint: minter,
        owner: mint_authority_account,
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

    // const update_cu_ix = anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({units: 1000000});
    //
    // const tx_ = new anchor.web3.Transaction().add(update_cu_ix)
    //    anchor.utils.

    const tx = await program.methods
        .createNftCollection(
            { energy: {} },
            'Energy Booster',
            'EB',
            'https://battleboosters.com/metadata',
            500 // 5% royalty
        )
        .accounts({
            creator: admin_account.publicKey,
            program: program_pda,
            mintAuthority: mint_authority_account,
            minter: minter,
            metadata: metadata,
            masterEdition: master_edition,
            tokenAccount: token_account,
            tokenRecord: token_record,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        })
        .preInstructions([
            anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
                units: 300000,
            }),
        ])
        .signers([admin_account]) // Include new_account as a signer
        .rpc();

    return {
        minter,
        metadata,
        master_edition,
        token_record,
    };
};

export default createNftCollection;
