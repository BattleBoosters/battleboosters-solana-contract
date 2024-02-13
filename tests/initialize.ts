import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import { Battleboosters } from "../target/types/battleboosters";
import {assert} from "chai";
import airdropSol from "./utils/airdrop_sol";
import { TOKEN_PROGRAM_ID, AccountLayout, MintLayout } from '@solana/spl-token';
const { SystemProgram, SYSVAR_RENT_PUBKEY } = anchor.web3;
import {mplTokenMetadata, getMplTokenMetadataProgramId} from "@metaplex-foundation/mpl-token-metadata";
import {MPL_TOKEN_METADATA_PROGRAM_ID} from "@metaplex-foundation/mpl-token-metadata";
import {PublicKey} from "@solana/web3.js";
describe.only("battleboosters", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;

    it("Is initialized!", async () => {

        const programInfo = await provider.connection.getAccountInfo(new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"))
        if (programInfo === null) {
            throw new Error('Program has not been deployed');
        }
        if (!programInfo.executable) {
            throw new Error('Program is not executable');
        }


        const admin_account = anchor.web3.Keypair.fromSecretKey( new Uint8Array([
            223,  59, 101, 153, 143,  21,  27,  11, 169, 175,  70,
                197,  18, 124,  44,  79, 218,  51, 196, 199, 144, 211,
                97,  87,  75, 138,  62, 180, 106, 250, 127, 172,   6,
                144, 226, 141, 181, 189,  96,  98, 164, 204, 232, 161,
                130, 182,  19, 162,  30, 200, 230, 194,  32,  45,  49,
                175, 101, 113,  85, 206, 140,   5, 206, 107
            ]),
        )

        // const admin_account = anchor.web3.Keypair.generate();
        // const program_account = anchor.web3.Keypair.generate();



        const energyMinter = anchor.web3.Keypair.generate();
        const shieldMinter = anchor.web3.Keypair.generate();
        const pointsMinter = anchor.web3.Keypair.generate();
        const fighterMinter = anchor.web3.Keypair.generate();
        const championsPassMinter = anchor.web3.Keypair.generate();


        const [mint_authority_account, authority_bump]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mintAuthority2"),
            ], program.programId);


        const [program_pda]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("program4"),
            ], program.programId);

        const [mintEnergyBooster]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mint1"),
                new BN(1).toBuffer()
            ], program.programId);

        const [mintShieldBooster]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mint1"),
                new BN(2).toBuffer()
            ], program.programId);
        const [mintPointsBooster]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mint1"),
                new BN(3).toBuffer()
            ], program.programId);
        const [mintFighter]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mint1"),
                new BN(4).toBuffer()
            ], program.programId);
        const [mintChampionsPass]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mint1"),
                new BN(5).toBuffer()
            ], program.programId);



        const [mintEnergyBooster2]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mint1")
            ], program.programId);

        const metadata_pubkey = new anchor.web3.PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID);

        const [metadataEnergyBooster]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadata_pubkey.toBuffer(),
                // mintEnergyBooster.toBuffer()
                mintEnergyBooster2.toBuffer()
            ], metadata_pubkey);


        const [metadataShieldBooster]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadata_pubkey.toBuffer(),
                // mintEnergyBooster.toBuffer()
                shieldMinter.publicKey.toBuffer()
            ], metadata_pubkey);
        const [metadataPointsBooster]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadata_pubkey.toBuffer(),
                // mintEnergyBooster.toBuffer()
                pointsMinter.publicKey.toBuffer()
            ], metadata_pubkey);
        const [metadataFighter]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadata_pubkey.toBuffer(),
                // mintEnergyBooster.toBuffer()
                fighterMinter.publicKey.toBuffer()
            ], metadata_pubkey);
        const [metadataChampionsPass]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadata_pubkey.toBuffer(),
                // mintEnergyBooster.toBuffer()
                championsPassMinter.publicKey.toBuffer()
            ], metadata_pubkey);




        const [masterEditionAccountEnergyBooster]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadata_pubkey.toBuffer(),
                mintEnergyBooster2.toBuffer(),
                Buffer.from("edition"),
            ], metadata_pubkey);
        const [masterEditionAccountShieldBooster]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadata_pubkey.toBuffer(),
                shieldMinter.publicKey.toBuffer(),
                Buffer.from("edition"),
            ], metadata_pubkey);
        const [masterEditionAccountPointsBooster]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadata_pubkey.toBuffer(),
                pointsMinter.publicKey.toBuffer(),
                Buffer.from("edition"),
            ], metadata_pubkey);
        const [masterEditionAccountFighter]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadata_pubkey.toBuffer(),
                fighterMinter.publicKey.toBuffer(),
                Buffer.from("edition"),
            ], metadata_pubkey);
        const [masterEditionAccountChampionsPass]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadata_pubkey.toBuffer(),
                championsPassMinter.publicKey.toBuffer(),
                Buffer.from("edition"),
            ], metadata_pubkey);




        // Airdrop admin_account
        await airdropSol(provider, admin_account.publicKey, 1);
        // // Airdrop program_account
        // await airdropSol(provider, program_account.publicKey, 1);
        // // Airdrop mintEnergyBooster
        // await airdropSol(provider, mintEnergyBooster.publicKey, 1);
        // // Airdrop mintShieldBooster
        // await airdropSol(provider, mintShieldBooster.publicKey, 1);
        // // Airdrop mintPointsBooster
        // await airdropSol(provider, mintPointsBooster.publicKey, 1);
        // // Airdrop mintFighter
        // await airdropSol(provider, mintFighter.publicKey, 1);
        // // Airdrop mintChampionsPass
        // await airdropSol(provider, mintChampionsPass.publicKey, 1);


        try {
            //Fetch the account details of the payment sender
            const programAccount = await program.account.programData.fetch(program_pda);

            console.log("PDA Account Data:", programAccount);
            assert.equal(programAccount.eventCounter.eq(new BN(0)),  true);
            assert.deepEqual(programAccount.adminPubkey, admin_account.publicKey);
            // assert.deepEqual(Buffer.from(senderAccount.rarityProbabilities), Buffer.from([1, 2, 3, 4 , 5]))
            assert.equal(programAccount.fighterPackPrice.eq(new BN((100 * anchor.web3.LAMPORTS_PER_SOL))), true)
            assert.equal(programAccount.boosterEnergyPrice.eq(new BN((1 * anchor.web3.LAMPORTS_PER_SOL))), true)
            assert.equal(programAccount.boosterShieldPrice.eq(new BN((1 * anchor.web3.LAMPORTS_PER_SOL))), true)
            assert.equal(programAccount.boosterPointsPrice.eq(new BN((1 * anchor.web3.LAMPORTS_PER_SOL))), true)
            assert.equal(programAccount.fighterPackAmount, 5)
        } catch (e) {
            try{
                const tx = await program.methods.initialize(
                    authority_bump,
                    admin_account.publicKey,
                    new BN((100 * anchor.web3.LAMPORTS_PER_SOL)),
                    new BN((1 * anchor.web3.LAMPORTS_PER_SOL)),
                    new BN((1 * anchor.web3.LAMPORTS_PER_SOL)),
                    new BN((1 * anchor.web3.LAMPORTS_PER_SOL)),
                    5
                )
                    .accounts({
                        creator: admin_account.publicKey,
                        program: program_pda,
                        mintAuthority: mint_authority_account,
                        energyMinter: mintEnergyBooster2,
                        // shieldMinter: shieldMinter.publicKey,
                        // pointsMinter: pointsMinter.publicKey,
                        // fighterMinter: fighterMinter.publicKey,
                        // championsPassMinter: championsPassMinter.publicKey,

                        //mintEnergyBooster: mintEnergyBooster,
                        // mintShieldBooster: mintShieldBooster,
                        // mintPointsBooster: mintPointsBooster,
                        // mintFighter: mintFighter,
                        // mintChampionsPass: mintChampionsPass,

                        metadataEnergyBooster: metadataEnergyBooster,
                        // metadataShieldBooster: metadataShieldBooster,
                        // metadataPointsBooster: metadataPointsBooster,
                        // metadataChampionsPass: metadataChampionsPass,
                        // metadataFighter: metadataFighter,

                        masterEditionAccountEnergyBooster: masterEditionAccountEnergyBooster,
                        // masterEditionAccountShieldBooster: masterEditionAccountShieldBooster,
                        // masterEditionAccountPointsBooster: masterEditionAccountPointsBooster,
                        // masterEditionAccountFighter: masterEditionAccountFighter,
                        // masterEditionAccountChampionsPass: masterEditionAccountChampionsPass,

                        sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
                        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                        systemProgram: anchor.web3.SystemProgram.programId,
                        tokenProgram: TOKEN_PROGRAM_ID,
                        metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID
                    })
                    .signers([admin_account]) // Include new_account as a signer
                    .rpc();


                // const tx2 = await program.methods.initializeEnergyBooster(
                //
                // )
                //     .accounts({
                //         creator: admin_account.publicKey,
                //
                //         mintAuthority: mint_authority_account,
                //         mintEnergyBooster: mintEnergyBooster,
                //         // mintShieldBooster: mintShieldBooster,
                //         // mintPointsBooster: mintPointsBooster,
                //         // mintFighter: mintFighter,
                //         // mintChampionsPass: mintChampionsPass,
                //
                //         // metadataEnergyBooster: metadataEnergyBooster.publicKey,
                //
                //         // metadataShieldBooster: metadataShieldBooster.publicKey,
                //         // metadataPointsBooster: metadataPointsBooster.publicKey,
                //         // metadataFighter: metadataFighter.publicKey,
                //         // metadataChampionsPass: metadataChampionsPass.publicKey,
                //
                //         // masterEditionAccountEnergyBooster: masterEditionEnergyBooster.publicKey,
                //         // masterEditionAccountShieldBooster: masterEditionShieldBooster.publicKey,
                //         // masterEditionAccountPointsBooster: masterEditionPointsBooster.publicKey,
                //         // masterEditionAccountFighter: masterEditionFighter.publicKey,
                //         // masterEditionAccountChampionsPass: masterEditionChampionsPass.publicKey,
                //
                //         rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                //         systemProgram: anchor.web3.SystemProgram.programId,
                //         tokenProgram: TOKEN_PROGRAM_ID,
                //         // metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID
                //     })
                //     .signers([admin_account]) // Include new_account as a signer
                //     .rpc();


                //Fetch the account details of the payment sender
                // const programAccount = await program.account.programData.fetch(program_pda);
                //
                //
                // console.log("ok test")
                // console.log("PDA Account Data:", programAccount);
                // assert.equal(programAccount.eventCounter.eq(new BN(0)),  true);
                // assert.deepEqual(programAccount.adminPubkey, admin_account.publicKey);
                // // assert.deepEqual(Buffer.from(senderAccount.rarityProbabilities), Buffer.from([1, 2, 3, 4 , 5]))
                // assert.equal(programAccount.fighterPackPrice.eq(new BN((100 * anchor.web3.LAMPORTS_PER_SOL))), true)
                // assert.equal(programAccount.boosterEnergyPrice.eq(new BN((1 * anchor.web3.LAMPORTS_PER_SOL))), true)
                // assert.equal(programAccount.boosterShieldPrice.eq(new BN((1 * anchor.web3.LAMPORTS_PER_SOL))), true)
                // assert.equal(programAccount.boosterPointsPrice.eq(new BN((1 * anchor.web3.LAMPORTS_PER_SOL))), true)
                // assert.equal(programAccount.fighterPackAmount, 5)


                // const accountInfo = await provider.connection.getAccountInfo(mintEnergyBooster.publicKey);
                // const accountData = MintLayout.decode(accountInfo.data);
                // assert.deepEqual(accountData.mintAuthority, mint_authority_account);
            }catch (e) {
                console.log(e)
            }
        }

        //console.log("Transaction signature", tx);
    });
});



