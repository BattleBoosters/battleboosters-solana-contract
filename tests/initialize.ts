import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import { Battleboosters } from "../target/types/battleboosters";
import {assert} from "chai";
import airdropSol from "./utils/airdrop_sol";
import { TOKEN_PROGRAM_ID, AccountLayout, MintLayout } from '@solana/spl-token';
const { SystemProgram, SYSVAR_RENT_PUBKEY } = anchor.web3;
import {mplTokenMetadata, getMplTokenMetadataProgramId} from "@metaplex-foundation/mpl-token-metadata";
import {MPL_TOKEN_METADATA_PROGRAM_ID} from "@metaplex-foundation/mpl-token-metadata";
describe.only("battleboosters", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;

    it("Is initialized!", async () => {
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
        const program_account = anchor.web3.Keypair.generate();

        const metadataEnergyBooster = anchor.web3.Keypair.generate();
        const metadataShieldBooster = anchor.web3.Keypair.generate();
        const metadataPointsBooster = anchor.web3.Keypair.generate();
        const metadataFighter = anchor.web3.Keypair.generate();
        const metadataChampionsPass = anchor.web3.Keypair.generate();
        const masterEditionEnergyBooster = anchor.web3.Keypair.generate();
        const masterEditionShieldBooster = anchor.web3.Keypair.generate();
        const masterEditionPointsBooster = anchor.web3.Keypair.generate();
        const masterEditionFighter = anchor.web3.Keypair.generate();
        const masterEditionChampionsPass = anchor.web3.Keypair.generate();

        console.log(admin_account.publicKey)
        console.log(admin_account.secretKey)
        console.log(program_account.publicKey)

        console.log(metadataEnergyBooster.publicKey)
        console.log(metadataShieldBooster.publicKey)
        console.log(metadataPointsBooster.publicKey)
        console.log(metadataFighter.publicKey)
        console.log(metadataChampionsPass.publicKey)
        console.log(masterEditionEnergyBooster.publicKey)
        console.log(masterEditionShieldBooster.publicKey)
        console.log(masterEditionPointsBooster.publicKey)
        console.log(masterEditionFighter.publicKey)
        console.log(masterEditionChampionsPass.publicKey)




        const [mint_authority_account]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mintAuthority1"),
            ], program.programId);

        const [program_pda]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("program3"),
            ], program.programId);

        const [mintEnergyBooster]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mint"),


            ], program.programId);

        const [mintShieldBooster]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mint"),
                new BN(2).toBuffer("le", 8)
            ], program.programId);
        const [mintPointsBooster]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mint"),
                new BN(3).toBuffer("le", 8)
            ], program.programId);
        const [mintFighter]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mint"),
                new BN(4).toBuffer("le", 8)
            ], program.programId);
        const [mintChampionsPass]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mint"),
                new BN(5).toBuffer("le", 8)
            ], program.programId);


        // Airdrop admin_account
        // await airdropSol(provider, admin_account.publicKey, 1);
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
                        mintEnergyBooster: mintEnergyBooster,
                        // mintShieldBooster: mintShieldBooster,
                        // mintPointsBooster: mintPointsBooster,
                        // mintFighter: mintFighter,
                        // mintChampionsPass: mintChampionsPass,

                        // metadataEnergyBooster: metadataEnergyBooster.publicKey,

                        // metadataShieldBooster: metadataShieldBooster.publicKey,
                        // metadataPointsBooster: metadataPointsBooster.publicKey,
                        // metadataFighter: metadataFighter.publicKey,
                        // metadataChampionsPass: metadataChampionsPass.publicKey,

                        // masterEditionAccountEnergyBooster: masterEditionEnergyBooster.publicKey,
                        // masterEditionAccountShieldBooster: masterEditionShieldBooster.publicKey,
                        // masterEditionAccountPointsBooster: masterEditionPointsBooster.publicKey,
                        // masterEditionAccountFighter: masterEditionFighter.publicKey,
                        // masterEditionAccountChampionsPass: masterEditionChampionsPass.publicKey,

                        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                        systemProgram: anchor.web3.SystemProgram.programId,
                        tokenProgram: TOKEN_PROGRAM_ID,
                        // metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID
                    })
                    .signers([admin_account]) // Include new_account as a signer
                    .rpc();


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



