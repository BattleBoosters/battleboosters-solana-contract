import * as anchor from '@coral-xyz/anchor';
import { BN } from '@coral-xyz/anchor';
import { sleep } from '@switchboard-xyz/common';
import { Battleboosters } from '../../target/types/battleboosters';
import {Transaction} from "@solana/web3.js";
const createMintableGameAsset = async function (
    program: anchor.Program<Battleboosters>,
    provider: anchor.AnchorProvider,
    program_pda,
    variant,
    rarity_pda,
    custom_player_game_asset_link_nonce,
    signer,
    mystery_box_nonce_nonce,
    randomness_pda,
    revealIx: anchor.web3.TransactionInstruction
) {
    let signer_ = signer
        ? signer.publicKey.toBuffer()
        : provider.wallet.publicKey.toBuffer();
    const program_pda_data = await program.account.programData.fetch(
        program_pda
    );
    const [mintable_game_asset_pda, mintable_game_asset_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mintableGameAsset'),
                new BN(program_pda_data.mintableGameAssetNonce).toBuffer(
                    'le',
                    8
                ),
            ],
            program.programId
        );

    const [player_account_pda, player_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from('BattleBoosters'), Buffer.from('player'), signer_],
            program.programId
        );

    const player_account_pda_data = await program.account.playerData.fetch(
        player_account_pda
    );
    let player_game_asset_link_nonce = !custom_player_game_asset_link_nonce
        ? player_account_pda_data.playerGameAssetLinkNonce
        : custom_player_game_asset_link_nonce;

    const [mintable_game_asset_link_pda, mintable_game_asset_link_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mintableGameAsset'),
                new BN(player_game_asset_link_nonce).toBuffer('le', 8),
                signer_,
            ],
            program.programId
        );

    const [mystery_box_pda, mystery_box_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mysteryBox'),
                signer_,
                new BN(mystery_box_nonce_nonce).toBuffer('le', 8),
            ],
            program.programId
        );

    let signers = signer ? [signer] : [];


    const tx = await program.methods
        .createMintableGameAsset(
            new BN(player_game_asset_link_nonce),
            new BN(mystery_box_nonce_nonce),
            variant
        )
        .accounts({
            signer: signer ? signer.publicKey : provider.wallet.publicKey,
            program: program_pda,
            playerAccount: player_account_pda,
            mysteryBox: mystery_box_pda,
            rarity: rarity_pda,
            mintableGameAssetLink: mintable_game_asset_link_pda,
            mintableGameAsset: mintable_game_asset_pda,
            randomnessAccountData: randomness_pda,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).instruction()

    const transaction = new Transaction();

    if (revealIx){
        transaction.add(revealIx)
    }
    transaction.add(tx)
    await provider.sendAndConfirm(transaction, signers);

    // console.log(tx)
    // await sleep(2000);
    // const logs = await provider.connection.getParsedTransaction(
    //     tx,
    //     "confirmed"
    // );
    //
    // console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));

    return {

        mintable_game_asset_pda,
        player_game_asset_link_pda: mintable_game_asset_link_pda,
        player_account_pda,
    };
};

export default createMintableGameAsset;
