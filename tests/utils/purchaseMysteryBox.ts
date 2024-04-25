import * as anchor from '@coral-xyz/anchor';
import { BN } from '@coral-xyz/anchor';
import { sleep } from '@switchboard-xyz/common';
import { Battleboosters } from '../../target/types/battleboosters';
import { Transaction } from '@solana/web3.js';
import InitializePlayerAccount from './initializePlayerAccount';
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
const createMintableGameAsset = async function (
    program: anchor.Program<Battleboosters>,
    provider: anchor.AnchorProvider,
    program_pda,
    rarity_pda,
    bank_pda,
    lastPriceSolUsd,
    recipient,
    boosterQtyToNumber,
    fighterQtyToNumber
) {
    const [player_account_pda, player_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('player'),
                recipient.publicKey.toBuffer(),
                //provider.wallet.publicKey.toBuffer(),
            ],
            program.programId
        );

    const playerAccountData = await program.account.playerData.fetch(
        player_account_pda
    );

    const [mystery_box_pda, mystery_box_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mysteryBox'),
                //provider.wallet.publicKey.toBuffer(),
                new BN(playerAccountData.orderNonce).toBuffer('le', 8),
                recipient.publicKey.toBuffer(),
            ],
            program.programId
        );

    const priceFeedAccount = new anchor.web3.PublicKey(
        'GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR'
    );

    let program_data = await program.account.programData.fetch(program_pda);

    const boosterQty = new anchor.BN(boosterQtyToNumber);
    const fighterQty = new anchor.BN(fighterQtyToNumber);
    const boosterPrice = program_data.boosterPrice;
    const fighterPrice = program_data.fighterPackPrice;

    const total = boosterQty
        .mul(boosterPrice)
        .add(fighterQty.mul(fighterPrice));

    const safeAmount = total.toNumber() * (1 / lastPriceSolUsd.toNumber());

    const amountToSend = new anchor.BN(
        anchor.web3.LAMPORTS_PER_SOL * safeAmount
    ); // For example, 1 SOL

    // // Create a transaction to transfer SOL from the signer to the bank_escrow PDA
    // const transferTx = new anchor.web3.Transaction().add(
    //     anchor.web3.SystemProgram.transfer({
    //         fromPubkey: provider.wallet.publicKey,
    //         toPubkey: user_bank_pda,
    //         lamports: amountToSend.toNumber(),
    //     })
    // );

    // Sign and send the transaction
    //await provider.sendAndConfirm(transferTx, []);
    // const tx2 = await InstructionUtils.asV0Tx(sb_program, [ix]);
    // await provider.sendAndConfirm(tx2, [rngKp]);

    // tx2.sign([admin_account, rngKp]);
    // const sig1 = await provider.connection.sendTransaction(tx2);
    // await provider.connection.confirmTransaction(sig1);

    // wait for RPC
    //await sleep(10000);

    // const accountData = await provider.connection.getAccountInfo(
    //     user_bank_pda
    // );
    // const rentExemptionAmount =
    //     await provider.connection.getMinimumBalanceForRentExemption(
    //         accountData.data.length
    //     );

    //console.log(randomness);
    // Add this instruction to your coinFlip transaction and send it
    //const commitIx = await randomness.commitIx();

    await program.methods
        .purchaseMysteryBox([
            {
                nftType: { booster: {} }, // Use the variant name as key for enum
                quantity: boosterQty,
            },
            {
                nftType: { fighter: {} }, // Use the variant name as key for enum
                quantity: fighterQty,
            },
        ])
        .accounts({
            signer: provider.wallet.publicKey,
            recipient: recipient.publicKey, //provider.wallet.publicKey,
            program: program_pda,
            playerAccount: player_account_pda,
            mysteryBox: mystery_box_pda,
            bank: bank_pda,
            priceFeed: priceFeedAccount,
            randomnessAccountData: provider.wallet.publicKey,
            rarity: rarity_pda,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([])
        .rpc();

    return {
        mystery_box_pda,
        player_account_pda,
        amountToSend,
    };
};

export default createMintableGameAsset;
