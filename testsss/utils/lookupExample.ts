import * as anchor from '@coral-xyz/anchor';

import { Battleboosters } from '../../target/types/battleboosters';
import { sleep } from '@switchboard-xyz/common';
import { BN } from '@coral-xyz/anchor';
async function PrepareLookupTable(
    provider: anchor.AnchorProvider,
    program: anchor.Program<Battleboosters>,
    admin_account,
    program_pda
) {
    const program_data_before = await program.account.programData.fetch(
        program_pda
    );

    const [event_account, event_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('event'),
                new BN(0).toBuffer('le', 8),
            ],
            program.programId
        );
    const [event_link_account, event_link_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('event'),
                event_account.toBuffer(),
                provider.wallet.publicKey.toBuffer(),
                //admin_account.publicKey.toBuffer()
                // new BN(0).toBuffer('le', 8),
            ],
            program.programId
        );

    console.log(event_account);

    const [fight_card_account, fight_card_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('fightCard'),
                event_account.toBuffer(),
                new BN(0).toBuffer(),
            ],
            program.programId
        );
    const [fight_card_link_account, fight_card_link_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('fightCard'),
                event_account.toBuffer(),
                new BN(0).toBuffer(),
                provider.wallet.publicKey.toBuffer(),
                //admin_account.publicKey.toBuffer()
            ],
            program.programId
        );
    const [player_account_pda, player_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('player'),
                provider.wallet.publicKey.toBuffer(),
                //admin_account.publicKey.toBuffer(),
            ],
            program.programId
        );

    const player_account_pda_data = await program.account.playerData.fetch(
        player_account_pda
    );

    const [fighter_mintable_game_asset_pda, fighter_mintable_game_asset_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mintableGameAsset'),
                new BN(0).toBuffer('le', 8),
            ],
            program.programId
        );
    const [
        fighter_mintable_game_asset_link_pda,
        fighter_mintable_game_asset_link_bump,
    ] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from('BattleBoosters'),
            Buffer.from('mintableGameAsset'),
            new BN(0).toBuffer('le', 8),
            provider.wallet.publicKey.toBuffer(),
            //admin_account.publicKey.toBuffer(),
        ],
        program.programId
    );

    console.log(player_account_pda_data);

    const { lookupTableInst, lookupTableAddress } = await PrepareLookupTable(
        admin_account,
        provider,
        program,
        [
            admin_account.publicKey,
            program_pda,
            event_account,
            fighter_mintable_game_asset_pda,
            fighter_mintable_game_asset_link_pda,
            fight_card_account,
            fight_card_link_account,
            event_link_account,
            anchor.web3.SystemProgram.programId,
        ]
    );
    await sleep(2000);

    const data2 = program.coder.instruction.encode('joinFightCard', {
        owners: new BN(0),
        eventId: new BN(0),
        fightCardId: 0,
        fighterAssetId: new BN(0),
        energyBoosterAssetId: null,
        shieldBoosterAssetId: null,
        pointsBoosterAssetId: null,
        championsPassAssetId: null,
        fighterLinkId: new BN(0),
        energyBoosterLinkId: null,
        shieldBoosterLinkId: null,
        pointsBoosterLinkId: null,
        championsPassLinkId: null,
        fighterColorSide: { fighterBlue: {} },
    });

    const accounts = [
        // The list of accounts should match those required by the instruction
        // For example:
        { pubkey: admin_account.publicKey, isSigner: true, isWritable: true },
        { pubkey: program_pda, isSigner: false, isWritable: true },
        { pubkey: event_account, isSigner: false, isWritable: true },
        {
            pubkey: fighter_mintable_game_asset_pda,
            isSigner: false,
            isWritable: true,
        },
        {
            pubkey: fighter_mintable_game_asset_link_pda,
            isSigner: false,
            isWritable: true,
        },
        { pubkey: fight_card_account, isSigner: false, isWritable: true },
        { pubkey: fight_card_link_account, isSigner: false, isWritable: true },
        // { pubkey: null, isSigner: false, isWritable: false },
        // { pubkey: null, isSigner: false, isWritable: false },
        // { pubkey: null, isSigner: false, isWritable: false },
        // { pubkey: null, isSigner: false, isWritable: false },
        // { pubkey: null, isSigner: false, isWritable: false },
        // { pubkey: null, isSigner: false, isWritable: false },
        // { pubkey: null, isSigner: false, isWritable: false },
        // { pubkey: null, isSigner: false, isWritable: false },
        { pubkey: event_link_account, isSigner: false, isWritable: true },
        {
            pubkey: anchor.web3.SystemProgram.programId,
            isSigner: false,
            isWritable: false,
        },
        // Add other accounts as necessary...
    ];
    // console.log("data2")
    // console.log(data2)
    // console.log("acccounts")
    // console.log(accounts)
    // Creating the instruction
    const instruction = new anchor.web3.TransactionInstruction({
        keys: accounts,
        programId: program.programId,
        data: data2,
    });
    // Ensure you have the recent blockhash
    const recentBlockhash = await provider.connection.getLatestBlockhash();

    // get the table from the cluster
    const lookupTableAccount = (
        await provider.connection.getAddressLookupTable(lookupTableAddress)
    ).value;
    console.log('Table address from cluster:', lookupTableAccount);

    try {
        // Create a TransactionMessage for a v0 transaction
        const message = new anchor.web3.TransactionMessage({
            payerKey: admin_account.publicKey,
            recentBlockhash: recentBlockhash.blockhash,
            instructions: [instruction], // Assuming only one instruction for simplicity
        }).compileToV0Message([lookupTableAccount]);

        // Create the VersionedTransaction
        const transaction = new anchor.web3.VersionedTransaction(message);

        // Sign the transaction
        //const signer = provider.wallet.publicKey; // This should be the actual signer object
        transaction.sign([admin_account]);

        const signers = [admin_account].map((pubKey) => pubKey.toString());

        // Construct the simulation configuration
        const config: anchor.web3.SimulateTransactionConfig = {
            commitment: 'confirmed',
            sigVerify: false,
            replaceRecentBlockhash: false,
            accounts: {
                encoding: 'base64',
                addresses: signers,
            },
        };

        // Perform the simulation
        //const simulatedTransactionResponse = await provider.connection.simulateTransaction(transaction, config);

        //console.log(simulatedTransactionResponse.value)

        // Send and confirm the transaction
        const txid = await provider.connection.sendTransaction(transaction);
        console.log(`Transaction ID: ${txid}`);
    } catch (e) {
        console.log(e);
    }
}
