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
function createMetrics(
    takedownsAttemptedPoints,
    takedownsAttemptedDamage,
    takedownsLandedPoints,
    takedownsLandedDamage,
    takedownsSlamPoints,
    takedownsSlamDamage,
    sigClinchHeadStrikesAttemptedPoints,
    sigClinchHeadStrikesAttemptedDamage,
    sigClinchHeadStrikesLandedPoints,
    sigClinchHeadStrikesLandedDamage,
    sigClinchBodyStrikesAttemptedPoints,
    sigClinchBodyStrikesAttemptedDamage,
    sigClinchBodyStrikesLandedPoints,
    sigClinchBodyStrikesLandedDamage,
    sigClinchLegStrikesAttemptedPoints,
    sigClinchLegStrikesAttemptedDamage,
    sigClinchLegStrikesLandedPoints,
    sigClinchLegStrikesLandedDamage,
    knockdownsPoints,
    knockdownsDamage,
    sigDistanceHeadStrikesAttemptedPoints,
    sigDistanceHeadStrikesAttemptedDamage,
    sigDistanceHeadStrikesLandedPoints,
    sigDistanceHeadStrikesLandedDamage,
    sigDistanceBodyStrikesAttemptedPoints,
    sigDistanceBodyStrikesAttemptedDamage,
    sigDistanceBodyStrikesLandedPoints,
    sigDistanceBodyStrikesLandedDamage,
    sigDistanceLegStrikesAttemptedPoints,
    sigDistanceLegStrikesAttemptedDamage,
    sigDistanceLegStrikesLandedPoints,
    sigDistanceLegStrikesLandedDamage,
    reversalsPoints,
    reversalsDamage,
    submissionsPoints,
    submissionsDamage,
    secondsInControlPoints,
    secondsInControlDamage,
    sigGroundHeadStrikesAttemptedPoints,
    sigGroundHeadStrikesAttemptedDamage,
    sigGroundHeadStrikesLandedPoints,
    sigGroundHeadStrikesLandedDamage,
    sigGroundBodyStrikesAttemptedPoints,
    sigGroundBodyStrikesAttemptedDamage,
    sigGroundBodyStrikesLandedPoints,
    sigGroundBodyStrikesLandedDamage,
    sigGroundLegStrikesAttemptedPoints,
    sigGroundLegStrikesAttemptedDamage,
    sigGroundLegStrikesLandedPoints,
    sigGroundLegStrikesLandedDamage,
    advanceToHalfGuardPoints,
    advanceToHalfGuardDamage,
    advanceToSidePoints,
    advanceToSideDamage,
    advanceToMountPoints,
    advanceToMountDamage,
    advanceToBackPoints,
    advanceToBackDamage
) {
    let metrics = {
        takedownsAttempted: {
            points: takedownsAttemptedPoints,
            damage: takedownsAttemptedDamage,
        },
        takedownsLanded: {
            points: takedownsLandedPoints,
            damage: takedownsLandedDamage,
        },
        takedownsSlam: {
            points: takedownsSlamPoints,
            damage: takedownsSlamDamage,
        },
        sigClinchHeadStrikesAttempted: {
            points: sigClinchHeadStrikesAttemptedPoints,
            damage: sigClinchHeadStrikesAttemptedDamage,
        },
        sigClinchHeadStrikesLanded: {
            points: sigClinchHeadStrikesLandedPoints,
            damage: sigClinchHeadStrikesLandedDamage,
        },
        sigClinchBodyStrikesAttempted: {
            points: sigClinchBodyStrikesAttemptedPoints,
            damage: sigClinchBodyStrikesAttemptedDamage,
        },
        sigClinchBodyStrikesLanded: {
            points: sigClinchBodyStrikesLandedPoints,
            damage: sigClinchBodyStrikesLandedDamage,
        },
        sigClinchLegStrikesAttempted: {
            points: sigClinchLegStrikesAttemptedPoints,
            damage: sigClinchLegStrikesAttemptedDamage,
        },
        sigClinchLegStrikesLanded: {
            points: sigClinchLegStrikesLandedPoints,
            damage: sigClinchLegStrikesLandedDamage,
        },
        knockDowns: { points: knockdownsPoints, damage: knockdownsDamage },
        sigDistanceHeadStrikesAttempted: {
            points: sigDistanceHeadStrikesAttemptedPoints,
            damage: sigDistanceHeadStrikesAttemptedDamage,
        },
        sigDistanceHeadStrikesLanded: {
            points: sigDistanceHeadStrikesLandedPoints,
            damage: sigDistanceHeadStrikesLandedDamage,
        },
        sigDistanceBodyStrikesAttempted: {
            points: sigDistanceBodyStrikesAttemptedPoints,
            damage: sigDistanceBodyStrikesAttemptedDamage,
        },
        sigDistanceBodyStrikesLanded: {
            points: sigDistanceBodyStrikesLandedPoints,
            damage: sigDistanceBodyStrikesLandedDamage,
        },
        sigDistanceLegStrikesAttempted: {
            points: sigDistanceLegStrikesAttemptedPoints,
            damage: sigDistanceLegStrikesAttemptedDamage,
        },
        sigDistanceLegStrikesLanded: {
            points: sigDistanceLegStrikesLandedPoints,
            damage: sigDistanceLegStrikesLandedDamage,
        },
        reversals: { points: reversalsPoints, damage: reversalsDamage },
        submissions: { points: submissionsPoints, damage: submissionsDamage },
        secondsInControl: {
            points: secondsInControlPoints,
            damage: secondsInControlDamage,
        },
        sigGroundHeadStrikesAttempted: {
            points: sigGroundHeadStrikesAttemptedPoints,
            damage: sigGroundHeadStrikesAttemptedDamage,
        },
        sigGroundHeadStrikesLanded: {
            points: sigGroundHeadStrikesLandedPoints,
            damage: sigGroundHeadStrikesLandedDamage,
        },
        sigGroundBodyStrikesAttempted: {
            points: sigGroundBodyStrikesAttemptedPoints,
            damage: sigGroundBodyStrikesAttemptedDamage,
        },
        sigGroundBodyStrikesLanded: {
            points: sigGroundBodyStrikesLandedPoints,
            damage: sigGroundBodyStrikesLandedDamage,
        },
        sigGroundLegStrikesAttempted: {
            points: sigGroundLegStrikesAttemptedPoints,
            damage: sigGroundLegStrikesAttemptedDamage,
        },
        sigGroundLegStrikesLanded: {
            points: sigGroundLegStrikesLandedPoints,
            damage: sigGroundLegStrikesLandedDamage,
        },
        advanceToHalfGuard: {
            points: advanceToHalfGuardPoints,
            damage: advanceToHalfGuardDamage,
        },
        advanceToSide: {
            points: advanceToSidePoints,
            damage: advanceToSideDamage,
        },
        advanceToMount: {
            points: advanceToMountPoints,
            damage: advanceToMountDamage,
        },
        advanceToBack: {
            points: advanceToBackPoints,
            damage: advanceToBackDamage,
        },
    };
    return metrics;
}

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
            let fighterPDAs = [];
            for (let i = 0; i < 8; i++) {
                const [fighterPDA] =
                    anchor.web3.PublicKey.findProgramAddressSync(
                        [
                            Buffer.from('BattleBoosters'),
                            Buffer.from('fighterBase'),
                            Buffer.from([i]), // Using the loop counter to create unique PDAs
                        ],
                        program.programId
                    );
                fighterPDAs.push(fighterPDA);
            }

            // Boxing
            let metrics_boxing = createMetrics(
                2, // takedownsAttemptedPoints
                0, // takedownsAttemptedDamage
                3, // takedownsLandedPoints
                3, // takedownsLandedDamage
                1, // takedownsSlamPoints
                1, // takedownsSlamDamage
                4, // sigClinchHeadStrikesAttemptedPoints
                0, // sigClinchHeadStrikesAttemptedDamage
                4, // sigClinchHeadStrikesLandedPoints
                4, // sigClinchHeadStrikesLandedDamage
                2, // sigClinchBodyStrikesAttemptedPoints
                0, // sigClinchBodyStrikesAttemptedDamage
                2, // sigClinchBodyStrikesLandedPoints
                2, // sigClinchBodyStrikesLandedDamage
                1, // sigClinchLegStrikesAttemptedPoints
                0, // sigClinchLegStrikesAttemptedDamage
                1, // sigClinchLegStrikesLandedPoints
                1, // sigClinchLegStrikesLandedDamage
                4, // knockdownsPoints
                3, // knockdownsDamage
                4, // sigDistanceHeadStrikesAttemptedPoints
                0, // sigDistanceHeadStrikesAttemptedDamage
                4, // sigDistanceHeadStrikesLandedPoints
                4, // sigDistanceHeadStrikesLandedDamage
                3, // sigDistanceBodyStrikesAttemptedPoints
                0, // sigDistanceBodyStrikesAttemptedDamage
                3, // sigDistanceBodyStrikesLandedPoints
                3, // sigDistanceBodyStrikesLandedDamage
                1, // sigDistanceLegStrikesAttemptedPoints
                0, // sigDistanceLegStrikesAttemptedDamage
                1, // sigDistanceLegStrikesLandedPoints
                1, // sigDistanceLegStrikesLandedDamage
                1, // reversalsPoints
                1, // reversalsDamage
                2, // submissionsPoints
                2, // submissionsDamage
                4, // secondsInControlPoints
                4, // secondsInControlDamage
                1, // sigGroundHeadStrikesAttemptedPoints
                0, // sigGroundHeadStrikesAttemptedDamage
                1, // sigGroundHeadStrikesLandedPoints
                1, // sigGroundHeadStrikesLandedDamage
                1, // sigGroundBodyStrikesAttemptedPoints
                0, // sigGroundBodyStrikesAttemptedDamage
                1, // sigGroundBodyStrikesLandedPoints
                1, // sigGroundBodyStrikesLandedDamage
                1, // sigGroundLegStrikesAttemptedPoints
                0, // sigGroundLegStrikesAttemptedDamage
                1, // sigGroundLegStrikesLandedPoints
                1, // sigGroundLegStrikesLandedDamage
                1, // advanceToHalfGuardPoints
                1, // advanceToHalfGuardDamage
                1, // advanceToSlidePoints
                1, // advanceToSlideDamage
                1, // advanceToMountPoints
                1, // advanceToMountDamage
                1, // advanceToBackPoints
                1 // advanceToBackDamage
            );

            // Muay Thai
            let metrics_muaythai = createMetrics(
                3, // takedownsAttemptedPoints
                0, // takedownsAttemptedDamage
                4, // takedownsLandedPoints
                4, // takedownsLandedDamage
                2, // takedownsSlamPoints
                2, // takedownsSlamDamage
                4, // sigClinchHeadStrikesAttemptedPoints
                0, // sigClinchHeadStrikesAttemptedDamage
                4, // sigClinchHeadStrikesLandedPoints
                4, // sigClinchHeadStrikesLandedDamage
                3, // sigClinchBodyStrikesAttemptedPoints
                0, // sigClinchBodyStrikesAttemptedDamage
                3, // sigClinchBodyStrikesLandedPoints
                3, // sigClinchBodyStrikesLandedDamage
                2, // sigClinchLegStrikesAttemptedPoints
                0, // sigClinchLegStrikesAttemptedDamage
                2, // sigClinchLegStrikesLandedPoints
                2, // sigClinchLegStrikesLandedDamage
                4, // knockdownsPoints
                4, // knockdownsDamage
                4, // sigDistanceHeadStrikesAttemptedPoints
                0, // sigDistanceHeadStrikesAttemptedDamage
                4, // sigDistanceHeadStrikesLandedPoints
                4, // sigDistanceHeadStrikesLandedDamage
                4, // sigDistanceBodyStrikesAttemptedPoints
                0, // sigDistanceBodyStrikesAttemptedDamage
                4, // sigDistanceBodyStrikesLandedPoints
                4, // sigDistanceBodyStrikesLandedDamage
                2, // sigDistanceLegStrikesAttemptedPoints
                0, // sigDistanceLegStrikesAttemptedDamage
                2, // sigDistanceLegStrikesLandedPoints
                2, // sigDistanceLegStrikesLandedDamage
                1, // reversalsPoints
                1, // reversalsDamage
                2, // submissionsPoints
                2, // submissionsDamage
                4, // secondsInControlPoints
                4, // secondsInControlDamage
                1, // sigGroundHeadStrikesAttemptedPoints
                0, // sigGroundHeadStrikesAttemptedDamage
                1, // sigGroundHeadStrikesLandedPoints
                1, // sigGroundHeadStrikesLandedDamage
                1, // sigGroundBodyStrikesAttemptedPoints
                0, // sigGroundBodyStrikesAttemptedDamage
                1, // sigGroundBodyStrikesLandedPoints
                1, // sigGroundBodyStrikesLandedDamage
                1, // sigGroundLegStrikesAttemptedPoints
                0, // sigGroundLegStrikesAttemptedDamage
                1, // sigGroundLegStrikesLandedPoints
                1, // sigGroundLegStrikesLandedDamage
                1, // advanceToHalfGuardPoints
                1, // advanceToHalfGuardDamage
                1, // advanceToSlidePoints
                1, // advanceToSlideDamage
                1, // advanceToMountPoints
                1, // advanceToMountDamage
                1, // advanceToBackPoints
                1 // advanceToBackDamage
            );

            // Taekwondo
            let metrics_taekwondo = createMetrics(
                2, // takedownsAttemptedPoints
                0, // takedownsAttemptedDamage
                3, // takedownsLandedPoints
                3, // takedownsLandedDamage
                1, // takedownsSlamPoints
                1, // takedownsSlamDamage
                4, // sigClinchHeadStrikesAttemptedPoints
                0, // sigClinchHeadStrikesAttemptedDamage
                4, // sigClinchHeadStrikesLandedPoints
                4, // sigClinchHeadStrikesLandedDamage
                2, // sigClinchBodyStrikesAttemptedPoints
                0, // sigClinchBodyStrikesAttemptedDamage
                2, // sigClinchBodyStrikesLandedPoints
                2, // sigClinchBodyStrikesLandedDamage
                1, // sigClinchLegStrikesAttemptedPoints
                0, // sigClinchLegStrikesAttemptedDamage
                1, // sigClinchLegStrikesLandedPoints
                1, // sigClinchLegStrikesLandedDamage
                3, // knockdownsPoints
                3, // knockdownsDamage
                4, // sigDistanceHeadStrikesAttemptedPoints
                0, // sigDistanceHeadStrikesAttemptedDamage
                4, // sigDistanceHeadStrikesLandedPoints
                4, // sigDistanceHeadStrikesLandedDamage
                3, // sigDistanceBodyStrikesAttemptedPoints
                0, // sigDistanceBodyStrikesAttemptedDamage
                3, // sigDistanceBodyStrikesLandedPoints
                3, // sigDistanceBodyStrikesLandedDamage
                2, // sigDistanceLegStrikesAttemptedPoints
                0, // sigDistanceLegStrikesAttemptedDamage
                2, // sigDistanceLegStrikesLandedPoints
                2, // sigDistanceLegStrikesLandedDamage
                1, // reversalsPoints
                1, // reversalsDamage
                1, // submissionsPoints
                1, // submissionsDamage
                3, // secondsInControlPoints
                3, // secondsInControlDamage
                1, // sigGroundHeadStrikesAttemptedPoints
                0, // sigGroundHeadStrikesAttemptedDamage
                1, // sigGroundHeadStrikesLandedPoints
                1, // sigGroundHeadStrikesLandedDamage
                1, // sigGroundBodyStrikesAttemptedPoints
                0, // sigGroundBodyStrikesAttemptedDamage
                1, // sigGroundBodyStrikesLandedPoints
                1, // sigGroundBodyStrikesLandedDamage
                1, // sigGroundLegStrikesAttemptedPoints
                0, // sigGroundLegStrikesAttemptedDamage
                1, // sigGroundLegStrikesLandedPoints
                1, // sigGroundLegStrikesLandedDamage
                1, // advanceToHalfGuardPoints
                1, // advanceToHalfGuardDamage
                1, // advanceToSlidePoints
                1, // advanceToSlideDamage
                1, // advanceToMountPoints
                1, // advanceToMountDamage
                1, // advanceToBackPoints
                1 // advanceToBackDamage
            );

            // Karate
            let metrics_karate = createMetrics(
                2, // takedownsAttemptedPoints
                0, // takedownsAttemptedDamage
                3, // takedownsLandedPoints
                3, // takedownsLandedDamage
                1, // takedownsSlamPoints
                1, // takedownsSlamDamage
                4, // sigClinchHeadStrikesAttemptedPoints
                0, // sigClinchHeadStrikesAttemptedDamage
                4, // sigClinchHeadStrikesLandedPoints
                4, // sigClinchHeadStrikesLandedDamage
                2, // sigClinchBodyStrikesAttemptedPoints
                0, // sigClinchBodyStrikesAttemptedDamage
                2, // sigClinchBodyStrikesLandedPoints
                2, // sigClinchBodyStrikesLandedDamage
                1, // sigClinchLegStrikesAttemptedPoints
                0, // sigClinchLegStrikesAttemptedDamage
                1, // sigClinchLegStrikesLandedPoints
                1, // sigClinchLegStrikesLandedDamage
                3, // knockdownsPoints
                3, // knockdownsDamage
                4, // sigDistanceHeadStrikesAttemptedPoints
                0, // sigDistanceHeadStrikesAttemptedDamage
                4, // sigDistanceHeadStrikesLandedPoints
                4, // sigDistanceHeadStrikesLandedDamage
                3, // sigDistanceBodyStrikesAttemptedPoints
                0, // sigDistanceBodyStrikesAttemptedDamage
                3, // sigDistanceBodyStrikesLandedPoints
                3, // sigDistanceBodyStrikesLandedDamage
                2, // sigDistanceLegStrikesAttemptedPoints
                0, // sigDistanceLegStrikesAttemptedDamage
                2, // sigDistanceLegStrikesLandedPoints
                2, // sigDistanceLegStrikesLandedDamage
                1, // reversalsPoints
                1, // reversalsDamage
                1, // submissionsPoints
                1, // submissionsDamage
                3, // secondsInControlPoints
                3, // secondsInControlDamage
                1, // sigGroundHeadStrikesAttemptedPoints
                0, // sigGroundHeadStrikesAttemptedDamage
                1, // sigGroundHeadStrikesLandedPoints
                1, // sigGroundHeadStrikesLandedDamage
                1, // sigGroundBodyStrikesAttemptedPoints
                0, // sigGroundBodyStrikesAttemptedDamage
                1, // sigGroundBodyStrikesLandedPoints
                1, // sigGroundBodyStrikesLandedDamage
                1, // sigGroundLegStrikesAttemptedPoints
                0, // sigGroundLegStrikesAttemptedDamage
                1, // sigGroundLegStrikesLandedPoints
                1, // sigGroundLegStrikesLandedDamage
                1, // advanceToHalfGuardPoints
                1, // advanceToHalfGuardDamage
                1, // advanceToSlidePoints
                1, // advanceToSlideDamage
                1, // advanceToMountPoints
                1, // advanceToMountDamage
                1, // advanceToBackPoints
                1 // advanceToBackDamage
            );

            // Judo
            let metrics_judo = createMetrics(
                4, // takedownsAttemptedPoints
                0, // takedownsAttemptedDamage
                5, // takedownsLandedPoints
                5, // takedownsLandedDamage
                2, // takedownsSlamPoints
                2, // takedownsSlamDamage
                2, // sigClinchHeadStrikesAttemptedPoints
                0, // sigClinchHeadStrikesAttemptedDamage
                2, // sigClinchHeadStrikesLandedPoints
                2, // sigClinchHeadStrikesLandedDamage
                1, // sigClinchBodyStrikesAttemptedPoints
                0, // sigClinchBodyStrikesAttemptedDamage
                1, // sigClinchBodyStrikesLandedPoints
                1, // sigClinchBodyStrikesLandedDamage
                1, // sigClinchLegStrikesAttemptedPoints
                0, // sigClinchLegStrikesAttemptedDamage
                1, // sigClinchLegStrikesLandedPoints
                1, // sigClinchLegStrikesLandedDamage
                3, // knockdownsPoints
                3, // knockdownsDamage
                3, // sigDistanceHeadStrikesAttemptedPoints
                0, // sigDistanceHeadStrikesAttemptedDamage
                3, // sigDistanceHeadStrikesLandedPoints
                3, // sigDistanceHeadStrikesLandedDamage
                2, // sigDistanceBodyStrikesAttemptedPoints
                0, // sigDistanceBodyStrikesAttemptedDamage
                2, // sigDistanceBodyStrikesLandedPoints
                2, // sigDistanceBodyStrikesLandedDamage
                1, // sigDistanceLegStrikesAttemptedPoints
                0, // sigDistanceLegStrikesAttemptedDamage
                1, // sigDistanceLegStrikesLandedPoints
                1, // sigDistanceLegStrikesLandedDamage
                2, // reversalsPoints
                2, // reversalsDamage
                3, // submissionsPoints
                3, // submissionsDamage
                5, // secondsInControlPoints
                5, // secondsInControlDamage
                1, // sigGroundHeadStrikesAttemptedPoints
                0, // sigGroundHeadStrikesAttemptedDamage
                1, // sigGroundHeadStrikesLandedPoints
                1, // sigGroundHeadStrikesLandedDamage
                1, // sigGroundBodyStrikesAttemptedPoints
                0, // sigGroundBodyStrikesAttemptedDamage
                1, // sigGroundBodyStrikesLandedPoints
                1, // sigGroundBodyStrikesLandedDamage
                1, // sigGroundLegStrikesAttemptedPoints
                0, // sigGroundLegStrikesAttemptedDamage
                1, // sigGroundLegStrikesLandedPoints
                1, // sigGroundLegStrikesLandedDamage
                1, // advanceToHalfGuardPoints
                1, // advanceToHalfGuardDamage
                1, // advanceToSlidePoints
                1, // advanceToSlideDamage
                1, // advanceToMountPoints
                1, // advanceToMountDamage
                2, // advanceToBackPoints
                2 // advanceToBackDamage
            );

            // Wrestling
            let metrics_wrestling = createMetrics(
                5, // takedownsAttemptedPoints
                0, // takedownsAttemptedDamage
                5, // takedownsLandedPoints
                5, // takedownsLandedDamage
                3, // takedownsSlamPoints
                3, // takedownsSlamDamage
                1, // sigClinchHeadStrikesAttemptedPoints
                0, // sigClinchHeadStrikesAttemptedDamage
                1, // sigClinchHeadStrikesLandedPoints
                1, // sigClinchHeadStrikesLandedDamage
                1, // sigClinchBodyStrikesAttemptedPoints
                0, // sigClinchBodyStrikesAttemptedDamage
                1, // sigClinchBodyStrikesLandedPoints
                1, // sigClinchBodyStrikesLandedDamage
                1, // sigClinchLegStrikesAttemptedPoints
                0, // sigClinchLegStrikesAttemptedDamage
                1, // sigClinchLegStrikesLandedPoints
                1, // sigClinchLegStrikesLandedDamage
                2, // knockdownsPoints
                2, // knockdownsDamage
                2, // sigDistanceHeadStrikesAttemptedPoints
                0, // sigDistanceHeadStrikesAttemptedDamage
                2, // sigDistanceHeadStrikesLandedPoints
                2, // sigDistanceHeadStrikesLandedDamage
                1, // sigDistanceBodyStrikesAttemptedPoints
                0, // sigDistanceBodyStrikesAttemptedDamage
                1, // sigDistanceBodyStrikesLandedPoints
                1, // sigDistanceBodyStrikesLandedDamage
                1, // sigDistanceLegStrikesAttemptedPoints
                0, // sigDistanceLegStrikesAttemptedDamage
                1, // sigDistanceLegStrikesLandedPoints
                1, // sigDistanceLegStrikesLandedDamage
                3, // reversalsPoints
                3, // reversalsDamage
                4, // submissionsPoints
                4, // submissionsDamage
                5, // secondsInControlPoints
                5, // secondsInControlDamage
                1, // sigGroundHeadStrikesAttemptedPoints
                0, // sigGroundHeadStrikesAttemptedDamage
                1, // sigGroundHeadStrikesLandedPoints
                1, // sigGroundHeadStrikesLandedDamage
                1, // sigGroundBodyStrikesAttemptedPoints
                0, // sigGroundBodyStrikesAttemptedDamage
                1, // sigGroundBodyStrikesLandedPoints
                1, // sigGroundBodyStrikesLandedDamage
                1, // sigGroundLegStrikesAttemptedPoints
                0, // sigGroundLegStrikesAttemptedDamage
                1, // sigGroundLegStrikesLandedPoints
                1, // sigGroundLegStrikesLandedDamage
                1, // advanceToHalfGuardPoints
                1, // advanceToHalfGuardDamage
                1, // advanceToSlidePoints
                1, // advanceToSlideDamage
                1, // advanceToMountPoints
                1, // advanceToMountDamage
                2, // advanceToBackPoints
                2 // advanceToBackDamage
            );

            // BJJ
            let metrics_bjj = createMetrics(
                1, // takedownsAttemptedPoints
                0, // takedownsAttemptedDamage
                2, // takedownsLandedPoints
                2, // takedownsLandedDamage
                1, // takedownsSlamPoints
                1, // takedownsSlamDamage
                3, // sigClinchHeadStrikesAttemptedPoints
                0, // sigClinchHeadStrikesAttemptedDamage
                3, // sigClinchHeadStrikesLandedPoints
                3, // sigClinchHeadStrikesLandedDamage
                2, // sigClinchBodyStrikesAttemptedPoints
                0, // sigClinchBodyStrikesAttemptedDamage
                2, // sigClinchBodyStrikesLandedPoints
                2, // sigClinchBodyStrikesLandedDamage
                1, // sigClinchLegStrikesAttemptedPoints
                0, // sigClinchLegStrikesAttemptedDamage
                1, // sigClinchLegStrikesLandedPoints
                1, // sigClinchLegStrikesLandedDamage
                2, // knockdownsPoints
                2, // knockdownsDamage
                2, // sigDistanceHeadStrikesAttemptedPoints
                0, // sigDistanceHeadStrikesAttemptedDamage
                2, // sigDistanceHeadStrikesLandedPoints
                2, // sigDistanceHeadStrikesLandedDamage
                1, // sigDistanceBodyStrikesAttemptedPoints
                0, // sigDistanceBodyStrikesAttemptedDamage
                1, // sigDistanceBodyStrikesLandedPoints
                1, // sigDistanceBodyStrikesLandedDamage
                1, // sigDistanceLegStrikesAttemptedPoints
                0, // sigDistanceLegStrikesAttemptedDamage
                1, // sigDistanceLegStrikesLandedPoints
                1, // sigDistanceLegStrikesLandedDamage
                4, // reversalsPoints
                4, // reversalsDamage
                5, // submissionsPoints
                5, // submissionsDamage
                5, // secondsInControlPoints
                5, // secondsInControlDamage
                2, // sigGroundHeadStrikesAttemptedPoints
                0, // sigGroundHeadStrikesAttemptedDamage
                2, // sigGroundHeadStrikesLandedPoints
                2, // sigGroundHeadStrikesLandedDamage
                3, // sigGroundBodyStrikesAttemptedPoints
                0, // sigGroundBodyStrikesAttemptedDamage
                3, // sigGroundBodyStrikesLandedPoints
                3, // sigGroundBodyStrikesLandedDamage
                1, // sigGroundLegStrikesAttemptedPoints
                0, // sigGroundLegStrikesAttemptedDamage
                1, // sigGroundLegStrikesLandedPoints
                1, // sigGroundLegStrikesLandedDamage
                2, // advanceToHalfGuardPoints
                2, // advanceToHalfGuardDamage
                2, // advanceToSlidePoints
                2, // advanceToSlideDamage
                2, // advanceToMountPoints
                2, // advanceToMountDamage
                3, // advanceToBackPoints
                3 // advanceToBackDamage
            );

            // Sambo
            let metrics_sambo = createMetrics(
                3, // takedownsAttemptedPoints
                0, // takedownsAttemptedDamage
                4, // takedownsLandedPoints
                4, // takedownsLandedDamage
                2, // takedownsSlamPoints
                2, // takedownsSlamDamage
                3, // sigClinchHeadStrikesAttemptedPoints
                0, // sigClinchHeadStrikesAttemptedDamage
                3, // sigClinchHeadStrikesLandedPoints
                3, // sigClinchHeadStrikesLandedDamage
                2, // sigClinchBodyStrikesAttemptedPoints
                0, // sigClinchBodyStrikesAttemptedDamage
                2, // sigClinchBodyStrikesLandedPoints
                2, // sigClinchBodyStrikesLandedDamage
                1, // sigClinchLegStrikesAttemptedPoints
                0, // sigClinchLegStrikesAttemptedDamage
                1, // sigClinchLegStrikesLandedPoints
                1, // sigClinchLegStrikesLandedDamage
                3, // knockdownsPoints
                3, // knockdownsDamage
                3, // sigDistanceHeadStrikesAttemptedPoints
                0, // sigDistanceHeadStrikesAttemptedDamage
                3, // sigDistanceHeadStrikesLandedPoints
                3, // sigDistanceHeadStrikesLandedDamage
                2, // sigDistanceBodyStrikesAttemptedPoints
                0, // sigDistanceBodyStrikesAttemptedDamage
                2, // sigDistanceBodyStrikesLandedPoints
                2, // sigDistanceBodyStrikesLandedDamage
                1, // sigDistanceLegStrikesAttemptedPoints
                0, // sigDistanceLegStrikesAttemptedDamage
                1, // sigDistanceLegStrikesLandedPoints
                1, // sigDistanceLegStrikesLandedDamage
                2, // reversalsPoints
                2, // reversalsDamage
                3, // submissionsPoints
                3, // submissionsDamage
                4, // secondsInControlPoints
                4, // secondsInControlDamage
                2, // sigGroundHeadStrikesAttemptedPoints
                0, // sigGroundHeadStrikesAttemptedDamage
                2, // sigGroundHeadStrikesLandedPoints
                2, // sigGroundHeadStrikesLandedDamage
                3, // sigGroundBodyStrikesAttemptedPoints
                0, // sigGroundBodyStrikesAttemptedDamage
                3, // sigGroundBodyStrikesLandedPoints
                3, // sigGroundBodyStrikesLandedDamage
                1, // sigGroundLegStrikesAttemptedPoints
                0, // sigGroundLegStrikesAttemptedDamage
                1, // sigGroundLegStrikesLandedPoints
                1, // sigGroundLegStrikesLandedDamage
                1, // advanceToHalfGuardPoints
                1, // advanceToHalfGuardDamage
                1, // advanceToSidePoints
                1, // advanceToSideDamage
                1, // advanceToMountPoints
                1, // advanceToMountDamage
                2, // advanceToBackPoints
                2 // advanceToBackDamage,
            );

            await program.methods
                .createFighter({ boxing: {} }, metrics_boxing)
                .accounts({
                    creator: admin_account.publicKey,
                    program: program_pda,
                    fighterBase: fighterPDAs[0],
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([admin_account])
                .rpc();

            await program.methods
                .createFighter({ muayThai: {} }, metrics_muaythai)
                .accounts({
                    creator: admin_account.publicKey,
                    program: program_pda,
                    fighterBase: fighterPDAs[1],
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([admin_account])
                .rpc();

            await program.methods
                .createFighter({ taekwondo: {} }, metrics_taekwondo)
                .accounts({
                    creator: admin_account.publicKey,
                    program: program_pda,
                    fighterBase: fighterPDAs[2],
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([admin_account])
                .rpc();

            await program.methods
                .createFighter({ karate: {} }, metrics_karate)
                .accounts({
                    creator: admin_account.publicKey,
                    program: program_pda,
                    fighterBase: fighterPDAs[3],
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([admin_account])
                .rpc();

            await program.methods
                .createFighter({ judo: {} }, metrics_judo)
                .accounts({
                    creator: admin_account.publicKey,
                    program: program_pda,
                    fighterBase: fighterPDAs[4],
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([admin_account])
                .rpc();

            await program.methods
                .createFighter({ wrestling: {} }, metrics_wrestling)
                .accounts({
                    creator: admin_account.publicKey,
                    program: program_pda,
                    fighterBase: fighterPDAs[5],
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([admin_account])
                .rpc();

            await program.methods
                .createFighter({ brazilianJiuJitsu: {} }, metrics_bjj)
                .accounts({
                    creator: admin_account.publicKey,
                    program: program_pda,
                    fighterBase: fighterPDAs[6],
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([admin_account])
                .rpc();

            await program.methods
                .createFighter({ sambo: {} }, metrics_sambo)
                .accounts({
                    creator: admin_account.publicKey,
                    program: program_pda,
                    fighterBase: fighterPDAs[7],
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([admin_account])
                .rpc();

            // let metrics = createMetrics(1)
            // let tx = await program.methods
            //     .createFighter(
            //         { taekwondo: {} },
            //         metrics
            //     )
            //     .accounts({
            //         creator: admin_account.publicKey,
            //         program: program_pda,
            //         fighterBase: fighter_pda,
            //         systemProgram: anchor.web3.SystemProgram.programId,
            //     })
            //     .signers([admin_account])
            //     .rpc();
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
