import * as anchor from '@coral-xyz/anchor';
import { assert } from 'chai';

const InitializePlayerAccount = async function (
    provider,
    publicKey,
    program,
    program_pda
) {
    // const [player_inventory_pda, player_inventory_bump] =
    //     anchor.web3.PublicKey.findProgramAddressSync(
    //         [
    //             Buffer.from('BattleBoosters'),
    //             Buffer.from('inventory'),
    //             publicKey.toBuffer(),
    //         ],
    //         program.programId
    //     );

    const [player_account_pda, player_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('player'),
                publicKey.toBuffer(),
            ],
            program.programId
        );

    try {
        await program.account.playerData.fetch(player_account_pda);
    } catch (e) {
        const initializePlayerTx = await program.methods
            .initializePlayer(publicKey)
            .accounts({
                creator: provider.wallet.publicKey,
                playerAccount: player_account_pda,
                program: program_pda,
            })
            .signers([]) // Include new_account as a signer
            .rpc();
    }
    return { player_account_pda };
};

export default InitializePlayerAccount;
