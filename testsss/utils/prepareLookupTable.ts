import * as anchor from '@coral-xyz/anchor';
import { Battleboosters } from '../../target/types/battleboosters';
async function PrepareLookupTable(
    admin_account,
    provider: anchor.AnchorProvider,
    program: anchor.Program<Battleboosters>,
    addresses
) {
    // Check if a Lookup Table exists or needs to be updated
    // Create or update the Lookup Table here
    // This might involve program.rpc.createLookupTable({...}), etc.

    try {
        const [lookupTableInst, lookupTableAddress] =
            anchor.web3.AddressLookupTableProgram.createLookupTable({
                authority: admin_account.publicKey,
                payer: admin_account.publicKey,
                recentSlot: (await provider.connection.getSlot()) - 1,
            });
        console.log('lookup table address:', lookupTableAddress.toBase58());
        // Create a transaction and add the instruction
        const tx = new anchor.web3.Transaction().add(lookupTableInst);
        // Sign and send the transaction
        await provider.sendAndConfirm(tx, [admin_account]);

        console.log('Lookup table created:', lookupTableAddress.toString());

        const transaction = new anchor.web3.Transaction();
        const extendInstruction =
            anchor.web3.AddressLookupTableProgram.extendLookupTable({
                payer: admin_account.publicKey,
                authority: admin_account.publicKey,
                lookupTable: lookupTableAddress,
                addresses: addresses,
            });

        transaction.add(extendInstruction);

        // Send the transaction
        await provider.sendAndConfirm(transaction, [admin_account]);
        console.log(lookupTableInst);
        console.log(lookupTableAddress);

        return { lookupTableInst, lookupTableAddress };
    } catch (e) {
        console.log(e);
    }
}

export default PrepareLookupTable;
