import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Battleboosters } from '../target/types/battleboosters';

describe.only("test", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Test as Program<Battleboosters>;

    it("Is initialized!", async () => {
        // Add your test here.
        //const tx = await program.methods.initialize().rpc();
        console.log("Your transaction signature");
    });
});