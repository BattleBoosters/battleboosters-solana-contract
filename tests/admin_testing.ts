import {createFightCard, updateFightCard} from './utils/createUpdateFightCard';
import * as anchor from '@coral-xyz/anchor';
import { BN, Program } from '@coral-xyz/anchor';
import account_init from './utils/initAccounts';
import { Battleboosters } from '../target/types/battleboosters';
import {createEvent, updateEvent} from './utils/createUpdateEvent';
import { assert } from 'chai';
import {sleep} from "@switchboard-xyz/common";
import {PublicKey} from "@solana/web3.js";

describe.only('Creator', () => {
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

    it.skip('should create an event', async () => {
        const new_time_start = 1813535498;
        const new_time_end = 1813623964;
        const { eventAccount } = await createEvent(
            provider,
            program,
            admin_account,
            program_pda,
            new_time_start,
            new_time_end,
            { mainCard: {} },
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
                {
                    startRank: new BN(3),
                    endRank: new BN(3),
                    prizeAmount: new BN(20),
                    fighterAmount: 1,
                    boosterAmount: 3,
                    championsPassAmount: 1,
                },
                {
                    startRank: new BN(4),
                    endRank: new BN(4),
                    prizeAmount: new BN(10),
                    fighterAmount: 1,
                    boosterAmount: 1,
                    championsPassAmount: 1,
                },
                {
                    startRank: new BN(5),
                    endRank: new BN(10),
                    prizeAmount: new BN(1),
                    fighterAmount: 1,
                    boosterAmount: 1,
                    championsPassAmount: 1,
                },
                {
                    startRank: new BN(11),
                    endRank: new BN(20),
                    prizeAmount: new BN(0),
                    fighterAmount: 1,
                    boosterAmount: 1,
                    championsPassAmount: 0,
                },
                {
                    startRank: new BN(21),
                    endRank: null,
                    prizeAmount: new BN(0),
                    fighterAmount: 1,
                    boosterAmount: 1,
                    championsPassAmount: 0,
                },
            ]
        );

    });

    it.skip("should add a fightCard", async() => {
        const { fight_card_account, event_account } = await createFightCard(
            provider,
            program,
            admin_account,
            program_pda,
            2,
            true
        );
    })

    it.skip("should create a fighting style", async () => {
        const [fighter_pda] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('fighter'),
                Buffer.from([7]),
            ],
            program.programId
        );

        let tx = await program.methods
            .createFighter(
                { sambo: {} },
                {
                    takedownsAttempted: { points: 10, energy: 1, damage: 50 },
                    takedownsLanded: { points: 4, energy: 1, damage: 50 },
                    takedownsSlam: { points: 5, energy: 1, damage: 50 },
                    sigClinchHeadStrikesAttempted: {
                        points: 1,
                        energy: 1,
                        damage: 50,
                    },
                    sigClinchHeadStrikesLanded: {
                        points: 6,
                        energy: 1,
                        damage: 50,
                    },
                    sigClinchBodyStrikesAttempted: {
                        points: 7,
                        energy: 1,
                        damage: 50,
                    },
                    sigClinchBodyStrikesLanded: {
                        points: 5,
                        energy: 1,
                        damage: 50,
                    },
                    sigClinchLegStrikesAttempted: {
                        points: 6,
                        energy: 1,
                        damage: 50,
                    },
                    sigClinchLegStrikesLanded: {
                        points: 5,
                        energy: 1,
                        damage: 50,
                    },
                    knockdowns: { points: 100, energy: 1, damage: 0 },
                    sigDistanceHeadStrikesAttempted: {
                        points: 56,
                        energy: 1,
                        damage: 50,
                    },
                    sigDistanceHeadStrikesLanded: {
                        points: 6,
                        energy: 1,
                        damage: 50,
                    },
                    sigDistanceBodyStrikesAttempted: {
                        points: 6,
                        energy: 1,
                        damage: 50,
                    },
                    sigDistanceBodyStrikesLanded: {
                        points: 6,
                        energy: 1,
                        damage: 50,
                    },
                    sigDistanceLegStrikesAttempted: {
                        points: 5,
                        energy: 1,
                        damage: 50,
                    },
                    sigDistanceLegStrikesLanded: {
                        points: 2,
                        energy: 1,
                        damage: 50,
                    },
                    reversals: { points: 1, energy: 1, damage: 50 },
                    submissions: { points: 4, energy: 1, damage: 50 },
                    secondsInControl: { points: 6, energy: 1, damage: 50 },
                    sigGroundHeadStrikesAttempted: {
                        points: 1,
                        energy: 1,
                        damage: 50,
                    },
                    sigGroundHeadStrikesLanded: {
                        points: 5,
                        energy: 1,
                        damage: 50,
                    },
                    sigGroundBodyStrikesAttempted: {
                        points: 7,
                        energy: 1,
                        damage: 50,
                    },
                    sigGroundBodyStrikesLanded: {
                        points: 9,
                        energy: 1,
                        damage: 50,
                    },
                    sigGroundLegStrikesAttempted: {
                        points: 12,
                        energy: 1,
                        damage: 50,
                    },
                    sigGroundLegStrikesLanded: {
                        points: 43,
                        energy: 1,
                        damage: 50,
                    },
                    advanceToHalfGuard: {
                        points: 45,
                        energy: 1,
                        damage: 50,
                    },
                    advanceToSlide: {
                        points: 67,
                        energy: 1,
                        damage: 50,
                    },
                    advanceToMount: {
                        points: 56,
                        energy: 1,
                        damage: 50,
                    },
                    advanceToBack: {
                        points: 56,
                        energy: 1,
                        damage: 50,
                    },
                }
            )
            .accounts({
                creator: admin_account.publicKey,
                program: program_pda,
                fighterBase: fighter_pda,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin_account])
            .rpc();
    })

    it.skip("update fight card", async () => {
        let fighterBlue = {
            takedownsAttempted: 2,
            takedownsLanded: 0,
            takedownsSlam: 1,
            sigClinchHeadStrikesAttempted: 3,
            sigClinchHeadStrikesLanded: 0,
            sigClinchBodyStrikesAttempted: 3,
            sigClinchBodyStrikesLanded: 0,
            sigClinchLegStrikesAttempted: 3,
            sigClinchLegStrikesLanded: 0,
            sigGroundHeadStrikesAttempted: 3,
            sigGroundHeadStrikesLanded: 0,
            sigGroundBodyStrikesAttempted: 3,
            sigGroundBodyStrikesLanded: 0,
            sigGroundLegStrikesAttempted: 3,
            sigGroundLegStrikesLanded: 0,
            strikingStrength: {
                knockdowns: 1,
                sigDistanceHeadStrikesAttempted: 0,
                sigDistanceHeadStrikesLanded: 0,
                sigDistanceBodyStrikesAttempted: 1,
                sigDistanceBodyStrikesLanded: 1,
                sigDistanceLegStrikesAttempted: 1,
                sigDistanceLegStrikesLanded: 1,
            },
            grapplingStrength: {
                submissions: 1,
                secondsInControl: 10,
                advanceToHalfGuard: 1000,
                advanceToSlide: 1,
                advanceToMount: 2,
                advanceToBack: 0,
            },
        };
        let fighterRed = {
            takedownsAttempted: 2,
            takedownsLanded: 0,
            takedownsSlam: 1,
            sigClinchHeadStrikesAttempted: 3,
            sigClinchHeadStrikesLanded: 0,
            sigClinchBodyStrikesAttempted: 3,
            sigClinchBodyStrikesLanded: 0,
            sigClinchLegStrikesAttempted: 3,
            sigClinchLegStrikesLanded: 0,
            sigGroundHeadStrikesAttempted: 3,
            sigGroundHeadStrikesLanded: 0,
            sigGroundBodyStrikesAttempted: 3,
            sigGroundBodyStrikesLanded: 0,
            sigGroundLegStrikesAttempted: 3,
            sigGroundLegStrikesLanded: 0,
            strikingStrength: {
                knockdowns: 1,
                sigDistanceHeadStrikesAttempted: 0,
                sigDistanceHeadStrikesLanded: 0,
                sigDistanceBodyStrikesAttempted: 1,
                sigDistanceBodyStrikesLanded: 1,
                sigDistanceLegStrikesAttempted: 1,
                sigDistanceLegStrikesLanded: 1,
            },
            grapplingStrength: {
                submissions: 1,
                secondsInControl: 10,
                advanceToHalfGuard: 1000,
                advanceToSlide: 1,
                advanceToMount: 2,
                advanceToBack: 0,
            },
        };
        let { fight_card_account } = await updateFightCard(
            provider,
            program,
            admin_account,
            program_pda,
            2,
            false,
            0,
            fighterBlue,
            fighterRed,
            new BN(200),
            { koTko: {} },
            { fighterBlue: {} }
        );
    })

    it.skip("should determine ranking points", async () => {

        const [event_account, event_account_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('event'),
                    new BN(2).toBuffer('le', 8),
                ],
                program.programId
            );

        const [fight_card_account, fight_card_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('fightCard'),
                    event_account.toBuffer(),
                    //new BN(fight_card_id).toBuffer(),
                    Buffer.from([0]),
                ],
                program.programId
            );

        const player = new PublicKey("Dov9Td4ZuYbnqrzTTX6G52QvuzZshezyX9pEdZpUsYh8");
        const [fight_card_link_account, fight_card_link_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('fightCard'),
                    event_account.toBuffer(),
                    fight_card_account.toBuffer(),
                    player.toBuffer(),
                    //admin_account.publicKey.toBuffer()
                ],
                program.programId
            );
        console.log(fight_card_link_account.toString())

        const [player_account_pda, player_account_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('player'),
                    player.toBuffer(),
                    //admin_account.publicKey.toBuffer(),
                ],
                program.programId
            );

        console.log(player_account_pda.toString())
        const fight_card_link_data = await program.account.fightCardLinkData.fetch(fight_card_link_account);

        const fighter_mintable_game_asset_pda = fight_card_link_data.fighterUsed
        const fighter_mintable_game_asset_link_pda =  fight_card_link_data.fighterLinkUsed
        const points_mintable_game_asset_link_pda =  fight_card_link_data.pointsBoosterUsed
        const shield_mintable_game_asset_link_pda =  fight_card_link_data.shieldBoosterUsed
        console.log(fighter_mintable_game_asset_pda.toString())
        console.log(fighter_mintable_game_asset_link_pda.toString())

        //const fighter_mintable_game_asset_link_pda = new PublicKey(fight_card_link_data.)
        // const [
        //     fighter_mintable_game_asset_pda,
        //     fighter_mintable_game_asset_bump,
        // ] = anchor.web3.PublicKey.findProgramAddressSync(
        //     [
        //         Buffer.from('BattleBoosters'),
        //         Buffer.from('mintableGameAsset'),
        //         new BN(0).toBuffer('le', 8),
        //     ],
        //     program.programId
        // );

        // const [
        //     fighter_mintable_game_asset_link_pda,
        //     fighter_mintable_game_asset_link_bump,
        // ] = anchor.web3.PublicKey.findProgramAddressSync(
        //     [
        //         Buffer.from('BattleBoosters'),
        //         Buffer.from('mintableGameAsset'),
        //         new BN(0).toBuffer('le', 8),
        //         provider.wallet.publicKey.toBuffer(),
        //         //admin_account.publicKey.toBuffer(),
        //     ],
        //     program.programId
        // );

        const [fighter_base_pda] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('fighter'),
                Buffer.from([7]),
            ],
            program.programId
        );

        const [rank_pda, rank_pda_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('rank'),
                    event_account.toBuffer(),
                    new BN(0).toBuffer('le', 8),
                    //admin_account.publicKey.toBuffer(),
                ],
                program.programId
            );

        try {
            let tx = await program.methods
                .determineRankingPoints({ sambo: {} })
                .accounts({
                    signer: provider.wallet.publicKey,
                    event: event_account,
                    rank: rank_pda,
                    playerAccount: player_account_pda,
                    fightCard: fight_card_account,
                    fightCardLink: fight_card_link_account,
                    fighterAsset: fighter_mintable_game_asset_pda,
                    fighterAssetLink: fighter_mintable_game_asset_link_pda,
                    pointsBoosterAsset: null,
                    shieldBoosterAsset: shield_mintable_game_asset_link_pda,
                    fighterBase: fighter_base_pda,
                })
                .signers([])
                .rpc();

            await sleep(2000);
            const logs = await provider.connection.getParsedTransaction(
                tx,
                'confirmed'
            );

            console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));

            let rank_data = await program.account.rankData.fetch(rank_pda);
            console.log(rank_data.totalPoints.toString());
            console.log(rank_data.isConsumed);
        } catch (e) {
            console.log('issue');
            console.log(e);
        }
    })

    it.skip('Should update an event', async () => {
        const event_id = 2;
        const new_time_start = 1713535498;
        const new_time_end = 1713623964;
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
                {
                    startRank: new BN(3),
                    endRank: new BN(3),
                    prizeAmount: new BN(20),
                    fighterAmount: 1,
                    boosterAmount: 3,
                    championsPassAmount: 1,
                },
                {
                    startRank: new BN(4),
                    endRank: new BN(4),
                    prizeAmount: new BN(10),
                    fighterAmount: 1,
                    boosterAmount: 1,
                    championsPassAmount: 1,
                },
                {
                    startRank: new BN(5),
                    endRank: new BN(10),
                    prizeAmount: new BN(1),
                    fighterAmount: 1,
                    boosterAmount: 1,
                    championsPassAmount: 1,
                },
                {
                    startRank: new BN(11),
                    endRank: new BN(20),
                    prizeAmount: new BN(0),
                    fighterAmount: 1,
                    boosterAmount: 1,
                    championsPassAmount: 0,
                },
                {
                    startRank: new BN(21),
                    endRank: null,
                    prizeAmount: new BN(0),
                    fighterAmount: 1,
                    boosterAmount: 1,
                    championsPassAmount: 0,
                },
            ]
        );
    });

    it('Update ranking', async () => {
        // Test code here

        const [event_account, event_account_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('event'),
                    new BN(2).toBuffer('le', 8),
                ],
                program.programId
            );

        const [rank_pda, rank_pda_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('rank'),
                    event_account.toBuffer(),
                    new BN(0).toBuffer('le', 8),
                    //admin_account.publicKey.toBuffer()
                    // new BN(0).toBuffer('le', 8),
                ],
                program.programId
            );

        try {
            let tx = await program.methods
                .adminUpdateRank(new BN(1))
                .accounts({
                    signer: admin_account.publicKey,
                    event: event_account,
                    rank: rank_pda,
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

            // let event_data = await program.account.eventData.fetch(event_account);
            // console.log(event_data.randomness);
        } catch (e) {
            console.log('issue');
            console.log(e);
        }
    });
});
