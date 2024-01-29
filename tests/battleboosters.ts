import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import { Battleboosters } from "../target/types/battleboosters";
import {assert} from "chai";
import {beforeEach} from "mocha";

describe("battleboosters", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;

    it("Is initialized!", async () => {
        const new_account = anchor.web3.Keypair.generate();

        const tx = await program.methods.initialize(provider.wallet.publicKey)
            .accounts({
                signer: provider.wallet.publicKey,
                newAccount: new_account.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([new_account]) // Include new_account as a signer
            .rpc();


        //Fetch the account details of the payment sender
        const senderAccount = await program.account.globalData.fetch(new_account.publicKey);

        assert.equal(senderAccount.eventCounter.eq(new BN(0)),  true);
        console.log("Your transaction signature", tx);
    });
});


describe("Create event", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;
    const admin_account = anchor.web3.Keypair.generate();
    const state_account =  anchor.web3.Keypair.generate();
    const event_account_one = anchor.web3.Keypair.generate();
    const random_account = anchor.web3.Keypair.generate();

    before("Initialize", async () => {
        await program.methods.initialize(provider.wallet.publicKey)
            .accounts({
                signer: provider.wallet.publicKey,
                newAccount: state_account.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([state_account])
            .rpc();

        // Airdrop random_account
        const airdrop_random_account = await provider.connection.requestAirdrop(
            random_account.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
        // await provider.connection.confirmTransaction(airdrop_random_account);

    })

    it("Should add a new event", async () => {

        let senderAccount = await program.account.globalData.fetch(state_account.publicKey);
        assert.equal(senderAccount.eventCounter.eq(new BN(0)),  true);

        const [event_account_one, event_account_one_bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("event"),
                new BN(senderAccount.eventCounter).toBuffer("le", 8)
            ], program.programId);

        const tx = await program.methods.createNewEvent(new BN(1713045216), new BN(1713045216))
            .accounts({
                creator: provider.wallet.publicKey,
                globalState: state_account.publicKey,
                eventAccount: event_account_one,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([])
            .rpc();


        // Fetch the account details of the payment sender
        senderAccount = await program.account.globalData.fetch(state_account.publicKey);

        /*
            TODO:
                - Fetch event account
                - Assert fight_card_id == 0
                - Assert start_date == 1713045216
                - Assert end_date == 1713045216
         */

        assert.equal(senderAccount.eventCounter.eq(new BN(1)),  true);
        console.log("Your transaction signature", tx);
    });

    it("Should fail adding a new event, unauthorized signer", async () => {

        let senderAccount = await program.account.globalData.fetch(state_account.publicKey);
        const [event_account_one, event_account_one_bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("event"),
                new BN(senderAccount.eventCounter).toBuffer("le", 8)
            ], program.programId);

        try {
            await program.methods.createNewEvent(new BN(1713045216), new BN(1713045216))
                .accounts({
                    creator: random_account.publicKey,
                    globalState: state_account.publicKey,
                    eventAccount: event_account_one,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([random_account])
                .rpc();
        }catch (err) {
            assert.include(err.message, 'Unauthorized access attempt')
        }

    });

    /*
        TODO:
            - It should fail in case the end_date <= to start_date
            - It should fail in case the start_date <= now

     */


});
