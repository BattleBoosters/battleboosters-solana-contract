import * as anchor from '@coral-xyz/anchor';
import { BN, Program } from '@coral-xyz/anchor';
import { Battleboosters } from '../target/types/battleboosters';
import { assert } from 'chai';
import account_init from './utils/initAccounts';
import {
    createFightCard,
    updateFightCard,
} from './utils/createUpdateFightCard';
import * as events from 'events';
describe('Create fight card', () => {
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

    it('Should add a main card fight card', async () => {
        const { fight_card_account, event_account } = await createFightCard(
            provider,
            program,
            admin_account,
            program_pda,
            0,
            true
        );
        let fightCardAccountData = await program.account.fightCardData.fetch(
            fight_card_account
        );

        assert.equal(
            fightCardAccountData.eventNonceTracker.eq(new BN(0)),
            true
        );
        assert.equal(fightCardAccountData.fightDuration, null);
        assert.equal(fightCardAccountData.titleFight, true);
        assert.equal(fightCardAccountData.fighterBlue, null);
        assert.equal(fightCardAccountData.fighterRed, null);
        assert.deepEqual(
            fightCardAccountData.eventPubkey.equals(event_account),
            true
        );
        assert.equal(fightCardAccountData.result, null);

        let fetchedEvent = await program.account.eventData.fetch(event_account);
        assert.equal(fetchedEvent.fightCardNonce, 1);
    });

    it('Should fail creating fight card unauthorized signer', async () => {
        try {
            await createFightCard(
                provider,
                program,
                unauthorized_account,
                program_pda,
                0,
                true
            );
        } catch (e) {
            assert.include(e.message, 'Unauthorized access attempt');
        }
    });

    it('Should update fight card from main card to prelims', async () => {
        const { fight_card_account, event_account } = await updateFightCard(
            provider,
            program,
            admin_account,
            program_pda,
            0,
            false,
            0,
            null,
            null,
            null,
            null,
            null
        );

        const eventAccount = await program.account.eventData.fetch(
            event_account
        );
        let fetchedFightCard = await program.account.fightCardData.fetch(
            fight_card_account
        );

        assert.equal(fetchedFightCard.fightDuration, null);
        assert.equal(fetchedFightCard.titleFight, false);
        assert.equal(fetchedFightCard.fighterBlue, null);
        assert.equal(fetchedFightCard.fighterRed, null);
        assert.deepEqual(
            fetchedFightCard.eventPubkey.equals(event_account),
            true
        );
        assert.equal(fetchedFightCard.result, null);
        let fetchedEvent = await program.account.eventData.fetch(event_account);
        assert.equal(fetchedEvent.fightCardNonce, 1);
    });

    it('Should fail updating fight card unauthorized signer', async () => {
        try {
            await updateFightCard(
                provider,
                program,
                unauthorized_account,
                program_pda,
                0,
                false,
                0,
                null,
                null,
                null,
                null,
                null
            );
        } catch (e) {
            assert.include(e.message, 'Unauthorized access attempt');
        }
    });

    it('Should add a early prelims fight card', async () => {
        const { fight_card_account, event_account } = await createFightCard(
            provider,
            program,
            admin_account,
            program_pda,
            0,
            false
        );
        let fightCardAccountData = await program.account.fightCardData.fetch(
            fight_card_account
        );
        assert.equal(
            fightCardAccountData.eventNonceTracker.eq(new BN(0)),
            true
        );
        assert.equal(fightCardAccountData.fightDuration, null);
        assert.equal(fightCardAccountData.titleFight, false);
        assert.equal(fightCardAccountData.fighterBlue, null);
        assert.equal(fightCardAccountData.fighterRed, null);
        assert.deepEqual(
            fightCardAccountData.eventPubkey.equals(event_account),
            true
        );
        assert.equal(fightCardAccountData.result, null);

        let fetchedEvent = await program.account.eventData.fetch(event_account);
        assert.equal(fetchedEvent.fightCardNonce, 2);
    });

    it('Should add a main card fight card', async () => {
        const { fight_card_account, event_account } = await createFightCard(
            provider,
            program,
            admin_account,
            program_pda,
            0,
            false
        );
        let fightCardAccountData = await program.account.fightCardData.fetch(
            fight_card_account
        );
        assert.equal(
            fightCardAccountData.eventNonceTracker.eq(new BN(0)),
            true
        );
        assert.equal(fightCardAccountData.fightDuration, null);
        assert.equal(fightCardAccountData.titleFight, false);
        assert.equal(fightCardAccountData.fighterBlue, null);
        assert.equal(fightCardAccountData.fighterRed, null);
        assert.deepEqual(
            fightCardAccountData.eventPubkey.equals(event_account),
            true
        );
        assert.equal(fightCardAccountData.result, null);

        let fetchedEvent = await program.account.eventData.fetch(event_account);
        assert.equal(fetchedEvent.fightCardNonce, 3);
    });
});
