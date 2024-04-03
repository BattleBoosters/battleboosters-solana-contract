import * as anchor from '@coral-xyz/anchor';
import { BN, Program } from '@coral-xyz/anchor';
import account_init from './utils/initAccounts';
import { Battleboosters } from '../target/types/battleboosters';
import { updateFightCard } from './utils/createUpdateFightCard';
import { updateEvent } from './utils/createUpdateEvent';
import { sleep } from '@switchboard-xyz/common';

describe('Request Randomness', () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);
    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;

    const {
        admin_account,
        unauthorized_account,
        metadata_pubkey,
        bank_pda,
        bank_bump,
        program_pda,
        program_bump,
        rarity_pda,
        rarity_bump,
        mint_authority_account,
        authority_bump,
    } = account_init(program);

    it('Set randomness admin', async () => {
        // Test code here

        const [event_account, event_account_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('event'),
                    new BN(0).toBuffer('le', 8),
                ],
                program.programId
            );

        try {
            let tx = await program.methods
                .adminSetRandomness(new BN(0))
                .accounts({
                    signer: admin_account.publicKey,
                    event: event_account,
                    program: program_pda,
                })
                .signers([admin_account])
                .rpc();

            await sleep(2000);
            const logs = await provider.connection.getParsedTransaction(
                tx,
                'confirmed'
            );

            console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));

            let event_data = await program.account.eventData.fetch(
                event_account
            );
            console.log(event_data.randomness);
        } catch (e) {
            console.log('issue');
            console.log(e);
        }
    });
});
