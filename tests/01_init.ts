import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import { Battleboosters } from '../target/types/battleboosters';
import {assert} from "chai";

import InitializePlayerAccount from '../tests_utils/utils/initializePlayerAccount';
import account_init from '../tests_utils/utils/initAccounts';
import airdrop_sol from "../tests_utils/utils/airdropSol";
import {before} from "mocha";

describe("init", () => {
    // Configure the client to use the local cluster.
    //anchor.setProvider(anchor.AnchorProvider.env());
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
        await airdrop_sol(provider, admin_account.publicKey, 10);
        await airdrop_sol(provider, unauthorized_account.publicKey, 10);
        await airdrop_sol(provider, provider.wallet.publicKey, 10);
    });

    it("Is initialized!", async () => {
        const fighter_price = new BN(20_000_000);
        const booster_price =  new BN(250_000)
        // define an obj before using
        let initAccounts = {
            creator: admin_account.publicKey,
            program: program_pda,
            bank: bank_pda,
            mintAuthority: mint_authority_account,
            systemProgram: anchor.web3.SystemProgram.programId,
        };

        const tx = await program.methods
            .initialize(
                authority_bump,
                bank_bump,
                admin_account.publicKey,
                fighter_price,
                booster_price,
                { dev: {} }
            ).accounts(initAccounts)
            .signers([admin_account]) // Include new_account as a signer
            .rpc();

        const programAccount = await program.account.programData.fetch(
            program_pda
        );
        assert.equal(programAccount.eventNonce.eq(new BN(0)), true);
        assert.equal(
            programAccount.mintableGameAssetNonce.eq(new BN(0)),
            true
        );
        assert.deepEqual(
            programAccount.adminPubkey,
            admin_account.publicKey
        );
        assert.equal(programAccount.fighterPrice.eq(fighter_price), true);
        assert.equal(programAccount.boosterPrice.eq(booster_price), true);
    });
});