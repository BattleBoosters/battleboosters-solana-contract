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

        const tx = await program.methods.initialize(
            provider.wallet.publicKey,
            rarity,
            Buffer.from([1, 2, 3, 4 ,5]),
            new BN((100 * anchor.web3.LAMPORTS_PER_SOL)),
            new BN((1 * anchor.web3.LAMPORTS_PER_SOL))
        )
            .accounts({
                signer: provider.wallet.publicKey,
                newAccount: new_account.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([new_account]) // Include new_account as a signer
            .rpc();

        //Fetch the account details of the payment sender
        const senderAccount = await program.account.globalStateData.fetch(new_account.publicKey);

        assert.equal(senderAccount.eventCounter.eq(new BN(0)),  true);
        assert.deepEqual(senderAccount.rarity.common, rarity.common);
        assert.deepEqual(senderAccount.adminPubkey, provider.wallet.publicKey);
        assert.deepEqual(Buffer.from(senderAccount.rarityProbabilities), Buffer.from([1, 2, 3, 4 , 5]))
        assert.equal(senderAccount.nftFighterPackPrice.eq(new BN((100 * anchor.web3.LAMPORTS_PER_SOL))), true)
        assert.equal(senderAccount.nftBoosterPackPrice.eq(new BN((1 * anchor.web3.LAMPORTS_PER_SOL))), true)
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
            provider.wallet.publicKey,
            rarity,
            Buffer.from([1, 2, 3, 4 ,5]),
            new BN((100 * anchor.web3.LAMPORTS_PER_SOL)),
            new BN((1 * anchor.web3.LAMPORTS_PER_SOL))
        )
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

        let senderAccount = await program.account.globalStateData.fetch(state_account.publicKey);
        assert.equal(senderAccount.eventCounter.eq(new BN(0)),  true);

        const [event_account_one, event_account_one_bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("event"),
                new BN(senderAccount.eventCounter).toBuffer("le", 8)
            ], program.programId);

        const tx = await program.methods.createNewEvent(new BN(1713045216), new BN(1711045216))
            .accounts({
                creator: provider.wallet.publicKey,
                globalState: state_account.publicKey,
                eventAccount: event_account_one,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([])
            .rpc();


        // Fetch the account details of the payment sender
        senderAccount = await program.account.globalStateData.fetch(state_account.publicKey);

        const eventAccount = await program.account.eventData.fetch(event_account_one);

        assert.equal(eventAccount.fightCardIdCounter, 0);
        assert.equal(eventAccount.startDate.eq(new BN(1713045216)), true);
        assert.equal(eventAccount.endDate.eq(new BN(1711045216)), true);
        assert.equal(senderAccount.eventCounter.eq(new BN(1)),  true);
        console.log("Your transaction signature", tx);
    });

    it("Should fail adding a new event, unauthorized signer", async () => {

        let senderAccount = await program.account.globalStateData.fetch(state_account.publicKey);
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


});
