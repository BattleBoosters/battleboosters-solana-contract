import * as anchor from '@coral-xyz/anchor';
import { BN } from '@coral-xyz/anchor';
import { sleep } from '@switchboard-xyz/common';
import { assert } from 'chai';
import { Battleboosters } from '../../target/types/battleboosters';
const createEvent = async function (
    provider: anchor.AnchorProvider,
    program: anchor.Program<Battleboosters>,
    admin_account,
    program_pda,
    time_start,
    time_end,
    tournament_type
) {
    const program_data_before = await program.account.programData.fetch(
        program_pda
    );
    const [event_account_one, event_account_one_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('event'),
                new BN(program_data_before.eventNonce).toBuffer('le', 8),
            ],
            program.programId
        );

    const tx = await program.methods
        .createNewEvent(new BN(time_start), new BN(time_end), tournament_type)
        .accounts({
            creator: admin_account.publicKey,
            program: program_pda,
            event: event_account_one,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([admin_account])
        .rpc();

    const eventAccount = await program.account.eventData.fetch(
        event_account_one
    );

    const program_data_after = await program.account.programData.fetch(
        program_pda
    );

    // console.log(tx)
    // await sleep(2000);
    // const logs = await provider.connection.getParsedTransaction(
    //     tx,
    //     "confirmed"
    // );
    //
    // console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));

    return {
        program_data_before,
        eventAccount,
        program_data_after,
    };
};

const updateEvent = async function (
    provider: anchor.AnchorProvider,
    program: anchor.Program<Battleboosters>,
    admin_account,
    program_pda,
    event_nonce,
    time_start,
    time_end,
    tournament_type
) {
    const [event_account_one, event_account_one_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('event'),
                new BN(event_nonce).toBuffer('le', 8),
            ],
            program.programId
        );

    try {
        const tx = await program.methods
            .updateEvent(
                new BN(event_nonce),
                new BN(time_start),
                new BN(time_end),
                tournament_type
            )
            .accounts({
                creator: admin_account.publicKey,
                program: program_pda,
                event: event_account_one,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin_account])
            .rpc();
    } catch (e) {
        assert.include(e.message, 'Unauthorized access attempt');
    }

    const eventAccount = await program.account.eventData.fetch(
        event_account_one
    );
    // console.log(tx)
    // await sleep(2000);
    // const logs = await provider.connection.getParsedTransaction(
    //     tx,
    //     "confirmed"
    // );
    //
    // console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));

    return {
        eventAccount,
    };
};

export { createEvent, updateEvent };
