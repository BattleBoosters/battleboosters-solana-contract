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

    it('Should add a new fight card', async () => {

        const { fight_card_account, event_account } = await createFightCard(
            provider,
            program,
            admin_account,
            program_pda,
            0,
            { mainCard: {} },
            true,
            112
        );
        let fightCardAccountData = await program.account.fightCardData.fetch(
            fight_card_account
        );
        assert.equal(fightCardAccountData.id.eq(new BN(112)), true);
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
        assert.deepEqual(fightCardAccountData.tournament, { mainCard: {} });

        let fetchedEvent = await program.account.eventData.fetch(event_account);
        assert.equal(fetchedEvent.fightCardIdCounter, 1);
    });

    it('Should fail creating fight card unauthorized signer', async () => {
        try {
            await createFightCard(
                provider,
                program,
                unauthorized_account,
                program_pda,
                0,
                { mainCard: {} },
                true,
                112
            );
        } catch (e) {
            assert.include(e.message, 'Unauthorized access attempt');
        }
    });

    it('Should update fight card', async () => {

        const { fight_card_account, event_account } = await updateFightCard(
            provider,
            program,
            admin_account,
            program_pda,
            0,
            { prelims: {} },
            false,
            112,
            0
        );

        const eventAccount = await program.account.eventData.fetch(
            event_account
        );
        let fetchedFightCard = await program.account.fightCardData.fetch(
            fight_card_account
        );
        assert.equal(fetchedFightCard.id.eq(new BN(112)), true);
        assert.equal(fetchedFightCard.fightDuration, null);
        assert.equal(fetchedFightCard.titleFight, false);
        assert.equal(fetchedFightCard.fighterBlue, null);
        assert.equal(fetchedFightCard.fighterRed, null);
        assert.deepEqual(
            fetchedFightCard.eventPubkey.equals(event_account),
            true
        );
        assert.equal(fetchedFightCard.result, null);
        assert.deepEqual(fetchedFightCard.tournament, { prelims: {} });

        let fetchedEvent = await program.account.eventData.fetch(event_account);
        assert.equal(fetchedEvent.fightCardIdCounter, 1);
    });

    it('Should fail updating fight card unauthorized signer', async () => {
        try {
            await updateFightCard(
                provider,
                program,
                unauthorized_account,
                program_pda,
                0,
                { prelims: {} },
                false,
                112,
                0
            );
        } catch (e) {
            assert.include(e.message, 'Unauthorized access attempt');
        }
    });
});
