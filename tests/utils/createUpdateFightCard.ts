import * as anchor from '@coral-xyz/anchor';
import { BN } from '@coral-xyz/anchor';
import { sleep } from '@switchboard-xyz/common';
import { assert } from 'chai';

const createFightCard = async function (
    provider,
    program,
    admin_account,
    program_pda,
    event_id,
    variant,
    is_title_fight,
    id_reference_off_chain
) {
    const [event_account, event_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('event'),
                new BN(event_id).toBuffer('le', 8),
            ],
            program.programId
        );
    console.log(event_account);
    let eventAccountData = await program.account.eventData.fetch(event_account);
    const [fight_card_account, fight_card_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('fightCard'),
                new BN(eventAccountData.fightCardIdCounter).toBuffer(),
            ],
            program.programId
        );

    const fightCardData = {
        id: new BN(id_reference_off_chain),
        eventPubkey: event_account,
        eventNonce: new BN(event_id),
        tournament: variant,
        titleFight: is_title_fight,
        fightStatsFighter1: null,
        fightStatsFighter2: null,
        fightDuration: null,
        result: null,
        winner: null,
    };

    try {
        const tx = await program.methods
            .createNewFightCard(fightCardData)
            .accounts({
                creator: admin_account.publicKey,
                program: program_pda,
                event: event_account,
                fightCard: fight_card_account,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin_account])
            .rpc();
    } catch (e) {
        console.log(e);
    }

    // console.log(tx)
    // await sleep(2000);
    // const logs = await provider.connection.getParsedTransaction(
    //     tx,
    //     "confirmed"
    // );
    //
    // console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));

    return {
        event_account,
        fight_card_account,
    };
};

const updateFightCard = async function (
    provider,
    program,
    admin_account,
    program_pda,
    event_id,
    variant,
    is_title_fight,
    id_reference_off_chain,
    fight_card_id
) {
    const [event_account, event_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('event'),
                new BN(event_id).toBuffer('le', 8),
            ],
            program.programId
        );

    const [fight_card_account, fight_card_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('fightCard'),
                new BN(fight_card_id).toBuffer(),
            ],
            program.programId
        );
    const fightCardData = {
        id: new BN(id_reference_off_chain),
        eventPubkey: event_account,
        eventNonce: new BN(event_id),
        tournament: variant,
        titleFight: is_title_fight,
        fightStatsFighter1: null,
        fightStatsFighter2: null,
        fightDuration: null,
        result: null,
        winner: null,
    };

    try {
        const tx = await program.methods
            .updateFightCard(fight_card_id, fightCardData)
            .accounts({
                creator: admin_account.publicKey,
                program: program_pda,
                event: event_account,
                fightCard: fight_card_account,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin_account])
            .rpc();
    } catch (e) {
        console.log(e);
    }

    // console.log(tx)
    // await sleep(2000);
    // const logs = await provider.connection.getParsedTransaction(
    //     tx,
    //     "confirmed"
    // );
    //
    // console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));

    return {
        event_account,
        fight_card_account,
    };
};

export { createFightCard, updateFightCard };
