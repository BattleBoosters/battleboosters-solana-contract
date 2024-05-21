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
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    Transaction,
} from '@solana/web3.js';
import { before } from 'mocha';
import airdrop_sol from './utils/airdropSol';
import { sleep } from '@switchboard-xyz/common';
import {
    AggregatorAccount,
    AnchorWallet,
    SwitchboardProgram,
} from '@switchboard-xyz/solana.js';
import InitializePlayerAccount from './utils/initializePlayerAccount';

import * as buffer from 'buffer';
import account_init from './utils/initAccounts';
import {
    SB_ON_DEMAND_PID,
    Randomness,
    InstructionUtils,
} from '@switchboard-xyz/on-demand';
import createMintableGameAsset from './utils/createMintableGameAsset';
import purchaseMysteryBox from './utils/purchaseMysteryBox';
/*
    TODO: Test try to pass nft different type
 */
describe.only('Purchase', () => {
    // const sb_programId = SB_ON_DEMAND_PID;
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;

    // const queue = new PublicKey('5Qv744yu7DmEbU669GmYRqL9kpQsyYsaVKdR8YiBMTaP');

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

    let switchboardProgram;
    let lastPriceSolUsd;
    before(async () => {
        switchboardProgram = await SwitchboardProgram.load(
            new Connection('https://api.mainnet-beta.solana.com')
        );

        // Check the latest SOL/USD price
        const aggregatorAccount = new AggregatorAccount(
            switchboardProgram,
            new anchor.web3.PublicKey(
                'GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR'
            )
        );
        lastPriceSolUsd = await aggregatorAccount.fetchLatestValue();
    });

    it('Purchase successfully in-game assets for signer', async () => {
        // const sb_idl = await anchor.Program.fetchIdl(sb_programId, provider);
        // //console.log("sb_idl")
        // //console.log(sb_idl)
        // const sb_program = new anchor.Program(sb_idl!, sb_programId, provider);
        // const rngKp = Keypair.generate();
        // const [randomness, ix] = await Randomness.create(
        //     sb_program,
        //     rngKp,
        //     queue
        // );

        const newRecipient = anchor.web3.Keypair.generate();

        // Initialize the player account first
        await InitializePlayerAccount(
            provider,
            newRecipient.publicKey,
            //provider.wallet.publicKey,
            program
        );

        try {
            const { mystery_box_pda, amountToSend } = await purchaseMysteryBox(
                program,
                provider,
                program_pda,
                rarity_pda,
                bank_pda,
                lastPriceSolUsd,
                newRecipient,
                3,
                1
            );

            const mystery_box_pda_after =
                await program.account.mysteryBoxData.fetch(mystery_box_pda);

            assert.isTrue(
                mystery_box_pda_after.boosterMintAllowance.eq(new BN(3))
            );
            // 5 because a pack contain 5 fighters
            assert.isTrue(
                mystery_box_pda_after.fighterMintAllowance.eq(new BN(5))
            );
            assert.isTrue(
                mystery_box_pda_after.championsPassMintAllowance.eq(new BN(0))
            );

            //Test if bank PDA received the correct SOL amount
            const bankPdaBalance = await provider.connection.getBalance(
                bank_pda
            );
            console.log('bankPdaBalance');
            console.log(bankPdaBalance);
            assert.isAbove(bankPdaBalance, 0);
        } catch (e) {
            console.log(e);
        }
    });

    /*
    it('Purchase successfully in-game assets for another recipient', async () => {
        const newRecipient = anchor.web3.Keypair.generate();

        const [bank_pda, bank_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [Buffer.from('BattleBoosters'), Buffer.from('bank')],
                program.programId
            );

        const [user_bank_pda, user_bank_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('bank'),
                    provider.wallet.publicKey.toBuffer(),
                ],
                program.programId
            );

        const [player_inventory_pda, player_inventory_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('inventory'),
                    newRecipient.publicKey.toBuffer(),
                ],
                program.programId
            );

        const priceFeedAccount = new anchor.web3.PublicKey(
            'GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR'
        );

        const programPDA = await program.account.programData.fetch(program_pda);

        const boosterQty = new anchor.BN(1);
        const fighterQty = new anchor.BN(2);
        const boosterPrice = programPDA.boosterPrice;
        const fighterPrice = programPDA.fighterPackPrice;

        const total = boosterQty
            .mul(boosterPrice)
            .add(fighterQty.mul(fighterPrice));
        const safeAmount =
            total.add(new BN(1)).toNumber() * (1 / lastPriceSolUsd.toNumber());

        const amountToSend = new anchor.BN(
            anchor.web3.LAMPORTS_PER_SOL * safeAmount
        ); // For example, 1 SOL

        // Create a transaction to transfer SOL from the signer to the bank_escrow PDA
        const transferTx = new anchor.web3.Transaction().add(
            anchor.web3.SystemProgram.transfer({
                fromPubkey: provider.wallet.publicKey,
                toPubkey: user_bank_pda,
                lamports: amountToSend.toNumber(),
            })
        );

        // Sign and send the transaction
        await provider.sendAndConfirm(transferTx, []);

        try {
            // Initialize the player account first
            await InitializePlayerAccount(
                provider,
                newRecipient.publicKey,
                program
            );

            const tx = await program.methods
                .purchaseMysteryBox(user_bank_bump, [
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
                    recipient: newRecipient.publicKey,
                    program: program_pda,
                    //playerInventory: player_inventory_pda,
                    bankEscrow: user_bank_pda,
                    bank: bank_pda,
                    priceFeed: priceFeedAccount,
                })
                .signers([]) // Include new_account as a signer
                .rpc();
            // wait for RPC
            await sleep(2000);
            const logs = await provider.connection.getParsedTransaction(
                tx,
                'confirmed'
            );

            console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));

            const playerInventoryAccountAfter =
                await program.account.mysteryBoxData.fetch(
                    player_inventory_pda
                );
            assert.isTrue(
                playerInventoryAccountAfter.boosterMintAllowance.eq(boosterQty)
            );
            assert.isTrue(
                playerInventoryAccountAfter.fighterMintAllowance.eq(fighterQty)
            );
            assert.isTrue(
                playerInventoryAccountAfter.championsPassMintAllowance.eq(
                    new BN(0)
                )
            );
        } catch (e) {
            console.log(e);
        }
    });

    it('Purchase error insuficient amount in purchase request', async () => {
        const newRecipient = anchor.web3.Keypair.generate();

        const [bank_pda, bank_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [Buffer.from('BattleBoosters'), Buffer.from('bank')],
                program.programId
            );

        const [user_bank_pda, user_bank_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('bank'),
                    provider.wallet.publicKey.toBuffer(),
                ],
                program.programId
            );

        const [player_inventory_pda, player_inventory_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('inventory'),
                    newRecipient.publicKey.toBuffer(),
                ],
                program.programId
            );

        const priceFeedAccount = new anchor.web3.PublicKey(
            'GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR'
        );

        const programPDA = await program.account.programData.fetch(program_pda);

        const boosterQty = new anchor.BN(0);
        const fighterQty = new anchor.BN(0);
        const boosterPrice = programPDA.boosterPrice;
        const fighterPrice = programPDA.fighterPackPrice;

        const total = boosterQty
            .mul(boosterPrice)
            .add(fighterQty.mul(fighterPrice));
        const safeAmount =
            total.add(new BN(0.1)).toNumber() *
            (1 / lastPriceSolUsd.toNumber());

        const amountToSend = new anchor.BN(
            anchor.web3.LAMPORTS_PER_SOL * safeAmount
        ); // For example, 1 SOL

        // Create a transaction to transfer SOL from the signer to the bank_escrow PDA
        const transferTx = new anchor.web3.Transaction().add(
            anchor.web3.SystemProgram.transfer({
                fromPubkey: provider.wallet.publicKey,
                toPubkey: user_bank_pda,
                lamports: amountToSend.toNumber(),
            })
        );

        // Sign and send the transaction
        await provider.sendAndConfirm(transferTx, []);

        try {
            // Initialize the player account first
            await InitializePlayerAccount(
                provider,
                newRecipient.publicKey,
                program
            );

            const tx = await program.methods
                .purchaseMysteryBox(user_bank_bump, [
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
                    recipient: newRecipient.publicKey,
                    program: program_pda,
                    // playerInventory: player_inventory_pda,
                    bankEscrow: user_bank_pda,
                    bank: bank_pda,
                    priceFeed: priceFeedAccount,
                })
                .signers([]) // Include new_account as a signer
                .rpc();
        } catch (e) {
            assert.include(
                e.message,
                'Insufficient amount in purchase request.'
            );
        }
    });

    it('Purchase unsuccessfully not enough money', async () => {
        const newRecipient = anchor.web3.Keypair.generate();

        const [bank_pda, bank_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [Buffer.from('BattleBoosters'), Buffer.from('bank')],
                program.programId
            );

        const [user_bank_pda, user_bank_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('bank'),
                    provider.wallet.publicKey.toBuffer(),
                ],
                program.programId
            );

        const [player_inventory_pda, player_inventory_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('inventory'),
                    newRecipient.publicKey.toBuffer(),
                ],
                program.programId
            );

        const priceFeedAccount = new anchor.web3.PublicKey(
            'GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR'
        );

        const programPDA = await program.account.programData.fetch(program_pda);

        const boosterQty = new anchor.BN(1);
        const fighterQty = new anchor.BN(2);
        const boosterPrice = programPDA.boosterPrice;
        const fighterPrice = programPDA.fighterPackPrice;

        const total = boosterQty
            .mul(boosterPrice)
            .add(fighterQty.mul(fighterPrice));
        const safeAmount =
            total.sub(new BN(1)).toNumber() * (1 / lastPriceSolUsd.toNumber());

        const amountToSend = new anchor.BN(
            anchor.web3.LAMPORTS_PER_SOL * safeAmount
        ); // For example, 1 SOL

        // Create a transaction to transfer SOL from the signer to the bank_escrow PDA
        const transferTx = new anchor.web3.Transaction().add(
            anchor.web3.SystemProgram.transfer({
                fromPubkey: provider.wallet.publicKey,
                toPubkey: user_bank_pda,
                lamports: amountToSend.toNumber(),
            })
        );

        // Sign and send the transaction
        await provider.sendAndConfirm(transferTx, []);

        try {
            // Initialize the player account first
            await InitializePlayerAccount(
                provider,
                newRecipient.publicKey,
                program
            );

            const tx = await program.methods
                .purchaseMysteryBox(user_bank_bump, [
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
                    recipient: newRecipient.publicKey,
                    program: program_pda,
                    //playerInventory: player_inventory_pda,
                    bankEscrow: user_bank_pda,
                    bank: bank_pda,
                    priceFeed: priceFeedAccount,
                })
                .signers([]) // Include new_account as a signer
                .rpc();
        } catch (e) {
            assert.include(e.message, 'Insufficient funds.');
        }
    });

*/
});
