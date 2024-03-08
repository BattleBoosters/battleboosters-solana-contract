import * as anchor from '@coral-xyz/anchor';
import { MPL_TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';

const initAccounts = function (program) {
    const metadata_pubkey = new anchor.web3.PublicKey(
        MPL_TOKEN_METADATA_PROGRAM_ID
    );

    const admin_account = anchor.web3.Keypair.fromSecretKey(
        new Uint8Array([
            223, 59, 101, 153, 143, 21, 27, 11, 169, 175, 70, 197, 18, 124, 44,
            79, 218, 51, 196, 199, 144, 211, 97, 87, 75, 138, 62, 180, 106, 250,
            127, 172, 6, 144, 226, 141, 181, 189, 96, 98, 164, 204, 232, 161,
            130, 182, 19, 162, 30, 200, 230, 194, 32, 45, 49, 175, 101, 113, 85,
            206, 140, 5, 206, 107,
        ])
    );
    const [bank_pda, bank_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from('BattleBoosters'), Buffer.from('bank')],
        program.programId
    );

    const [program_pda, program_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from('BattleBoosters'), Buffer.from('program')],
            program.programId
        );

    const [rarity_pda, rarity_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from('BattleBoosters'), Buffer.from('rarity')],
            program.programId
        );

    const [mint_authority_account, authority_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from('BattleBoosters'), Buffer.from('mintAuthority')],
            program.programId
        );

    return {
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
    };
};

export default initAccounts;
