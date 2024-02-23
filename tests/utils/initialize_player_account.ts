import * as anchor from "@coral-xyz/anchor";
import {assert} from "chai";

const InitializePlayerAccount = async function(provider, publicKey, program, program_pda) {


    const [player_inventory_pda, player_inventory_bump]  = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("BattleBoosters"),
            Buffer.from("inventory"),
            publicKey.toBuffer()
        ], program.programId);
    try {
        const playerInventoryAccountAfter = await program.account.inventoryData.fetch(player_inventory_pda);
        console.log(playerInventoryAccountAfter.isInitialized)
    }catch (e) {
        console.log(e)
        const initializePlayerTx = await program.methods.initializePlayer(
            publicKey
        )
            .accounts({
                creator: provider.wallet.publicKey,
                inventory: player_inventory_pda,
                program: program_pda,
            }).signers([]) // Include new_account as a signer
            .rpc();
    }


}

export default InitializePlayerAccount
