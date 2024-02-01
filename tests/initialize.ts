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
        const program_account = anchor.web3.Keypair.generate();

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
                program: program_account.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([program_account]) // Include new_account as a signer
            .rpc();

        //Fetch the account details of the payment sender
        const senderAccount = await program.account.programData.fetch(program_account.publicKey);

        assert.equal(senderAccount.eventCounter.eq(new BN(0)),  true);
        assert.deepEqual(senderAccount.rarity.common, rarity.common);
        assert.deepEqual(senderAccount.adminPubkey, provider.wallet.publicKey);
        assert.deepEqual(Buffer.from(senderAccount.rarityProbabilities), Buffer.from([1, 2, 3, 4 , 5]))
        assert.equal(senderAccount.nftFighterPackPrice.eq(new BN((100 * anchor.web3.LAMPORTS_PER_SOL))), true)
        assert.equal(senderAccount.nftBoosterPackPrice.eq(new BN((1 * anchor.web3.LAMPORTS_PER_SOL))), true)
        console.log("Transaction signature", tx);
    });
});



