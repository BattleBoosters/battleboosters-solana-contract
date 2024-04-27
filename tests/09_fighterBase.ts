import * as anchor from '@coral-xyz/anchor';
import { BN, Program } from '@coral-xyz/anchor';
import { Battleboosters } from '../target/types/battleboosters';
import { assert } from 'chai';
import account_init from './utils/initAccounts';
import { joinFightCard } from './utils/joinFightCard';
import createMintableGameAsset from './utils/createMintableGameAsset';
import InitializePlayerAccount from './utils/initializePlayerAccount';
import airdropSol from './utils/airdropSol';
import { SystemProgram } from '@solana/web3.js';
import { sleep } from '@switchboard-xyz/common';
describe('fighter', () => {
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

    it('Create a fighter', async () => {
        try {
            const [fighter_pda] = anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('fighterBase'),
                    Buffer.from([2]),
                ],
                program.programId
            );

            let tx = await program.methods
                .createFighter(
                    { taekwondo: {} },
                    {
                        takedownsAttempted: {
                            points: 10,
                            energy: 1,
                            damage: 50,
                        },
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

            console.log(tx);
        } catch (e) {
            console.log(e);
        }
        // await sleep(2000);
        // const logs = await provider.connection.getParsedTransaction(
        //     tx,
        //     'confirmed'
        // );
        //
        // console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));
        //
        // let fighter_pda_data = await program.account.fighterData.fetch(
        //     fighter_pda
        // );
        // console.log('knockdown points');
        // console.log(fighter_pda_data.fightMetrics.knockdowns);
        // console.log(fighter_pda_data.fightMetrics.takedownsAttempted);
        // console.log(fighter_pda_data.fighterType);
    });
});
