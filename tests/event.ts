import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import { Battleboosters } from "../target/types/battleboosters";
import {assert} from "chai";
describe("Create event", () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;
    const admin_account = anchor.web3.Keypair.generate();
    const program_account =  anchor.web3.Keypair.generate();
    const random_account = anchor.web3.Keypair.generate();


    before("Initialize", async () => {
        let rarity = {
            "common": {
                "powerMin": 10,
                "powerMax": 100,
                "lifespanMin": 10,
                "lifespanMax": 100,
                "energyMin": 10,
                "energyMax": 100,
            }
        }

        await program.methods.initialize(
            admin_account.publicKey,
            rarity,
            Buffer.from([1, 2, 3, 4 ,5]),
            new BN((100 * anchor.web3.LAMPORTS_PER_SOL)),
            new BN((1 * anchor.web3.LAMPORTS_PER_SOL))
        )
            .accounts({
                creator: provider.wallet.publicKey,
                program: program_account.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([program_account])
            .rpc();

        // Airdrop random_account
        const airdrop_random_account = await provider.connection.requestAirdrop(
            random_account.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL);
        const latestBlockHash = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: airdrop_random_account
        });
        // Airdrop random_account
        const airdrop_admin_account = await provider.connection.requestAirdrop(
            admin_account.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL);
        const latestBlockHash2 = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction({
            blockhash: latestBlockHash2.blockhash,
            lastValidBlockHeight: latestBlockHash2.lastValidBlockHeight,
            signature: airdrop_admin_account
        });
    })

    it("Should add a new event", async () => {

        let senderAccount = await program.account.programData.fetch(program_account.publicKey);
        assert.equal(senderAccount.eventCounter.eq(new BN(0)),  true);

        const [event_account_one, event_account_one_bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("event"),
                new BN(senderAccount.eventCounter).toBuffer("le", 8)
            ], program.programId);


        const tx = await program.methods.createNewEvent(new BN(1713045216), new BN(1711045216))
            .accounts({
                creator: admin_account.publicKey,
                program: program_account.publicKey,
                event: event_account_one,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin_account])
            .rpc();


        // Fetch the account details of the payment sender
        senderAccount = await program.account.programData.fetch(program_account.publicKey);

        const eventAccount = await program.account.eventData.fetch(event_account_one);

        assert.equal(eventAccount.fightCardIdCounter, 0);
        assert.equal(eventAccount.startDate.eq(new BN(1713045216)), true);
        assert.equal(eventAccount.endDate.eq(new BN(1711045216)), true);
        assert.equal(senderAccount.eventCounter.eq(new BN(1)),  true);
        //console.log("Transaction signature", tx);
    });

    it("Should add a second new event", async () => {

        let senderAccount = await program.account.programData.fetch(program_account.publicKey);
        assert.equal(senderAccount.eventCounter.eq(new BN(1)),  true);

        const [event_account_one, event_account_one_bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("event"),
                new BN(senderAccount.eventCounter).toBuffer("le", 8)
            ], program.programId);


        const tx = await program.methods.createNewEvent(new BN(1713045216), new BN(1711045216))
            .accounts({
                creator: admin_account.publicKey,
                program: program_account.publicKey,
                event: event_account_one,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin_account])
            .rpc();


        // Fetch the account details of the payment sender
        senderAccount = await program.account.programData.fetch(program_account.publicKey);

        const eventAccount = await program.account.eventData.fetch(event_account_one);


        assert.equal(eventAccount.fightCardIdCounter, 0);
        assert.equal(eventAccount.startDate.eq(new BN(1713045216)), true);
        assert.equal(eventAccount.endDate.eq(new BN(1711045216)), true);
        assert.equal(senderAccount.eventCounter.eq(new BN(2)),  true);
        //console.log("Transaction signature", tx);
    });

    it("Should fail adding a new event, unauthorized signer", async () => {

        let senderAccount = await program.account.programData.fetch(program_account.publicKey);
        const [event_account_one, event_account_one_bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("event"),
                new BN(senderAccount.eventCounter).toBuffer("le", 8)
            ], program.programId);

        try {
            await program.methods.createNewEvent(new BN(1713045216), new BN(1711045216))
                .accounts({
                    creator: random_account.publicKey,
                    program: program_account.publicKey,
                    event: event_account_one,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([random_account])
                .rpc();
        }catch (err) {
            assert.include(err.message, 'Unauthorized access attempt')
        }

    });

    it("Should update an event", async () => {

        const [event_account_one, event_account_one_bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("event"),
                new BN(0).toBuffer("le", 8)
            ], program.programId);


        const tx = await program.methods.updateEvent(new BN(0), new BN(1713045316), new BN(1711045516))
            .accounts({
                creator: admin_account.publicKey,
                program: program_account.publicKey,
                event: event_account_one,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin_account])
            .rpc();

        console.log("Transaction signature", tx);

        const eventAccount = await program.account.eventData.fetch(event_account_one);
        assert.equal(eventAccount.startDate.eq(new BN(1713045316)), true );
        assert.equal(eventAccount.endDate.eq(new BN(1711045516)), true );
        assert.equal(eventAccount.fightCardIdCounter, 0);

    })

    it("Should fail updating a new event, unauthorized signer", async () => {

        const [event_account_one, event_account_one_bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("event"),
                new BN(0).toBuffer("le", 8)
            ], program.programId);

        try {
            await program.methods.updateEvent(new BN(0), new BN(1713045316), new BN(1711045516))
                .accounts({
                    creator: random_account.publicKey,
                    program: program_account.publicKey,
                    event: event_account_one,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([random_account])
                .rpc();

        }catch (err) {
            assert.include(err.message, 'Unauthorized access attempt')
        }

    })

});
