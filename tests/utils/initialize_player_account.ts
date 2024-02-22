import * as anchor from "@coral-xyz/anchor";

const InitializePlayerAccount = async function(provider, publicKey, program, program_pda) {


    const [player_inventory_pda, player_inventory_bump]  = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("BattleBoosters"),
            Buffer.from("inventory"),
            publicKey.toBuffer()
        ], program.programId);
    // Initialize the player account first
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

export default InitializePlayerAccount
