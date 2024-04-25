import * as anchor from '@coral-xyz/anchor';
import { BN, Program } from '@coral-xyz/anchor';
import account_init from './utils/initAccounts';
import { Battleboosters } from '../target/types/battleboosters';
import { updateFightCard } from './utils/createUpdateFightCard';
import { updateEvent } from './utils/createUpdateEvent';
import { sleep } from '@switchboard-xyz/common';

describe('Determine ranking points', () => {
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

    before(async () => {
        // Force end the event and update it
        try {
            await updateEvent(
                provider,
                program,
                admin_account,
                program_pda,
                0,
                0,
                0,
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
                ]
            );

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
                0,
                false,
                0,
                fighterBlue,
                fighterRed,
                new BN(200),
                { koTko: {} },
                { fighterBlue: {} }
            );
            let fight_card_data = await program.account.fightCardData.fetch(
                fight_card_account
            );
            console.log('fight_card_data.fighterBlue');
            console.log(fight_card_data.fighterBlue);
            console.log(fight_card_data.result);
            console.log(fight_card_data.winner);
        } catch (e) {
            console.log(e);
        }
    });

    it('should return correct ranking points', async () => {
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

        const [fight_card_link_account, fight_card_link_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('fightCard'),
                    event_account.toBuffer(),
                    new BN(0).toBuffer(),
                    provider.wallet.publicKey.toBuffer(),
                    //admin_account.publicKey.toBuffer()
                ],
                program.programId
            );

        const [player_account_pda, player_account_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('player'),
                    provider.wallet.publicKey.toBuffer(),
                    //admin_account.publicKey.toBuffer(),
                ],
                program.programId
            );

        const [
            fighter_mintable_game_asset_pda,
            fighter_mintable_game_asset_bump,
        ] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mintableGameAsset'),
                new BN(0).toBuffer('le', 8),
            ],
            program.programId
        );

        const [
            fighter_mintable_game_asset_link_pda,
            fighter_mintable_game_asset_link_bump,
        ] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mintableGameAsset'),
                new BN(0).toBuffer('le', 8),
                provider.wallet.publicKey.toBuffer(),
                //admin_account.publicKey.toBuffer(),
            ],
            program.programId
        );

        const [fighter_pda] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('fighter'),
                Buffer.from([6]),
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
                .determineRankingPoints({ brazilianJiuJitsu: {} })
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
                    shieldBoosterAsset: null,
                    fighterBase: fighter_pda,
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
    });
});
