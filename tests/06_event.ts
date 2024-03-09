import * as anchor from '@coral-xyz/anchor';
import { BN, Program } from '@coral-xyz/anchor';
import { Battleboosters } from '../target/types/battleboosters';
import { assert } from 'chai';
import account_init from './utils/account_init';
import { createEvent, updateEvent } from './utils/createUpdateEvent';
describe('Create event', () => {
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

    it('Should add a new event', async () => {
        const time_start = 1713045216;
        const time_end = 1711045216;
        const { program_data_before, eventAccount, program_data_after } =
            await createEvent(
                provider,
                program,
                admin_account,
                program_pda,
                time_start,
                time_end
            );

        assert.equal(program_data_before.eventNonce.eq(new BN(0)), true);
        assert.equal(eventAccount.fightCardIdCounter, 0);
        assert.equal(eventAccount.startDate.eq(new BN(time_start)), true);
        assert.equal(eventAccount.endDate.eq(new BN(time_end)), true);
        assert.equal(program_data_after.eventNonce.eq(new BN(1)), true);
        //console.log("Transaction signature", tx);
    });

    it('Should add a second new event', async () => {
        const time_start = 1713045216;
        const time_end = 1711045216;
        const { program_data_before, eventAccount, program_data_after } =
            await createEvent(
                provider,
                program,
                admin_account,
                program_pda,
                time_start,
                time_end
            );
        assert.equal(program_data_before.eventNonce.eq(new BN(1)), true);
        assert.equal(eventAccount.fightCardIdCounter, 0);
        assert.equal(eventAccount.startDate.eq(new BN(time_start)), true);
        assert.equal(eventAccount.endDate.eq(new BN(time_end)), true);
        assert.equal(program_data_after.eventNonce.eq(new BN(2)), true);
        //console.log("Transaction signature", tx);
    });

    it('Should fail adding a new event, unauthorized signer', async () => {
        try {
            const time_start = 1713045216;
            const time_end = 1711045216;
            await createEvent(
                provider,
                program,
                unauthorized_account,
                program_pda,
                time_start,
                time_end
            );
        } catch (e) {
            assert.include(e.message, 'Unauthorized access attempt');
        }
    });

    it('Should update an event', async () => {
        const event_id = 0;
        const new_time_start = 1713045316;
        const new_time_end = 1711045516;
        const { eventAccount } = await updateEvent(
            provider,
            program,
            admin_account,
            program_pda,
            event_id,
            new_time_start,
            new_time_end
        );
        assert.equal(eventAccount.startDate.eq(new BN(new_time_start)), true);
        assert.equal(eventAccount.endDate.eq(new BN(new_time_end)), true);
        assert.equal(eventAccount.fightCardIdCounter, 0);
    });

    it('Should fail updating a new event, unauthorized signer', async () => {
        try {
            const event_id = 0;
            const new_time_start = 1713045316;
            const new_time_end = 1711045516;
            await updateEvent(
                provider,
                program,
                unauthorized_account,
                program_pda,
                event_id,
                new_time_start,
                new_time_end
            );
        } catch (err) {
            assert.include(err.message, 'Unauthorized access attempt');
        }
    });

    it('Should fail updating a new event id not found', async () => {
        try {
            const event_id = 1;
            const new_time_start = 1713045316;
            const new_time_end = 1711045516;
            await updateEvent(
                provider,
                program,
                admin_account,
                program_pda,
                event_id,
                new_time_start,
                new_time_end
            );
        } catch (err) {
            assert.include(err.message, 'Unauthorized access attempt');
        }
    });
});
