import {updateFightCard} from "./utils/createUpdateFightCard";
import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import account_init from "./utils/initAccounts";
import { Battleboosters } from '../target/types/battleboosters';
import {updateEvent} from "./utils/createUpdateEvent";
import {assert} from "chai";

describe.only('Collect Rewards', () => {
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


    it('Should update an event', async () => {
        const event_id = 0;
        const new_time_start = 1713535498;
        const new_time_end = 1813535498;
        const { eventAccount } = await updateEvent(
            provider,
            program,
            admin_account,
            program_pda,
            event_id,
            new_time_start,
            new_time_end,
            { prelims: {} },
            [
                {
                    startRank: new BN(1),
                    endRank: new BN(1),
                    prizeAmount: new BN(100),
                    fighterAmount: 1,
                    boosterAmount: 5,
                    championsPassAmount: 1,
                },
                {
                    startRank: new BN(2),
                    endRank: new BN(2),
                    prizeAmount: new BN(50),
                    fighterAmount: 1,
                    boosterAmount: 5,
                    championsPassAmount: 1,
                },
            ]
        );
        assert.equal(eventAccount.startDate.eq(new BN(new_time_start)), true);
        assert.equal(eventAccount.endDate.eq(new BN(new_time_end)), true);
        assert.equal(eventAccount.fightCardNonce, 0);
    });

})