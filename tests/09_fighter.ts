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
describe('fighter', () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);
    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;
});
