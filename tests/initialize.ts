import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import { Battleboosters } from "../target/types/battleboosters";
import {assert} from "chai";
import airdropSol from "./utils/airdrop_sol";
import { TOKEN_PROGRAM_ID, AccountLayout, MintLayout } from '@solana/spl-token';
const { SystemProgram, SYSVAR_RENT_PUBKEY } = anchor.web3;
import {mplTokenMetadata, getMplTokenMetadataProgramId} from "@metaplex-foundation/mpl-token-metadata";
import {MPL_TOKEN_METADATA_PROGRAM_ID} from "@metaplex-foundation/mpl-token-metadata";
import {PublicKey, Transaction} from "@solana/web3.js";
import {before} from "mocha";
import airdrop_sol from "./utils/airdrop_sol";
import { sleep } from "@switchboard-xyz/common";

describe.only("battleboosters", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;

    const metadata_pubkey = new anchor.web3.PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID);
    const admin_account = anchor.web3.Keypair.fromSecretKey( new Uint8Array([
            223,  59, 101, 153, 143,  21,  27,  11, 169, 175,  70,
            197,  18, 124,  44,  79, 218,  51, 196, 199, 144, 211,
            97,  87,  75, 138,  62, 180, 106, 250, 127, 172,   6,
            144, 226, 141, 181, 189,  96,  98, 164, 204, 232, 161,
            130, 182,  19, 162,  30, 200, 230, 194,  32,  45,  49,
            175, 101, 113,  85, 206, 140,   5, 206, 107
        ]),
    )
    const [bank_pda, bank_bump]  = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("BattleBoosters"),
            Buffer.from("bank"),
        ], program.programId);

    const [program_pda, program_bump]  = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("BattleBoosters"),
            Buffer.from("program"),
        ], program.programId);

    const [mint_authority_account, authority_bump]  = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("BattleBoosters"),
            Buffer.from("mintAuthority"),
        ], program.programId);




    before("Initialize", async () => {

        const programInfo = await provider.connection.getAccountInfo(new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"))
        if (programInfo === null) {
            throw new Error('Program has not been deployed');
        }
        if (!programInfo.executable) {
            throw new Error('Program is not executable');
        }

        await airdrop_sol(provider, admin_account.publicKey, 10);

        const tx = await program.methods.initialize(
            authority_bump,
            bank_bump,
            admin_account.publicKey,
            new BN((100 * anchor.web3.LAMPORTS_PER_SOL)),
            new BN((1 * anchor.web3.LAMPORTS_PER_SOL)),
            5
        )
            .accounts({
                creator: admin_account.publicKey,
                program: program_pda,
                bank: bank_pda,
                mintAuthority: mint_authority_account,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin_account]) // Include new_account as a signer
            .rpc();

        const programAccount = await program.account.programData.fetch(program_pda);
        assert.equal(programAccount.eventCounter.eq(new BN(0)),  true);
        assert.deepEqual(programAccount.adminPubkey, admin_account.publicKey);
        assert.equal(programAccount.fighterPackPrice.eq(new BN((100 * anchor.web3.LAMPORTS_PER_SOL))), true)
        assert.equal(programAccount.boosterPrice.eq(new BN((1 * anchor.web3.LAMPORTS_PER_SOL))), true)
        assert.equal(programAccount.fighterPackAmount, 5)
    })

    it.only("Interacts with the mocked oracle", async () => {

        const [user_bank_pda, user_bank_bump]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("bank"),
                provider.wallet.publicKey.toBuffer()
            ], program.programId);

        const priceFeedAccount = new anchor.web3.PublicKey("GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR");

        try {
            const tx = await program.methods.purchaseNfts(
                user_bank_bump,
                [
                    {
                        nftType: { booster: {} }, // Use the variant name as key for enum
                        quantity: new anchor.BN(1),
                    },
                    {
                        nftType: { fighterPack: {} }, // Use the variant name as key for enum
                        quantity: new anchor.BN(2),
                    }
                ]
            )
                .accounts({
                    signer: provider.wallet.publicKey,
                    program: program_pda,
                    bank: user_bank_pda,
                    priceFeed: priceFeedAccount
                })
                .signers([]) // Include new_account as a signer
                .rpc();
            // wait for RPC
            await sleep(2000);
            const logs = await provider.connection.getParsedTransaction(
                tx,
                "confirmed"
            );

            console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));

        }catch (e) {
            console.log(e)
        }



    });

    it("Create NFT collection" ,async () => {

        const [minter]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mint"),
              Buffer.from([0])
            ], program.programId);



        const [metadata]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadata_pubkey.toBuffer(),
                minter.toBuffer()
            ], metadata_pubkey);

        const [masterEdition]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadata_pubkey.toBuffer(),
                minter.toBuffer(),
                Buffer.from("edition"),
            ], metadata_pubkey);


        const tx = await program.methods.createNftCollection(
            { energy:{} },
            "Energy Booster",
            "https://battleboosters.com/metadata",
            500 // 5% royalty
        )
            .accounts({
                creator: admin_account.publicKey,
                program: program_pda,
                mintAuthority: mint_authority_account,
                minter: minter,
                metadata: metadata,
                masterEdition: masterEdition,
                sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID
            })
            .signers([admin_account]) // Include new_account as a signer
            .rpc();
    })
    //
    // it ("test", async () => {
    //
    //
    //     const solUsdId = new anchor.web3.PublicKey('J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix');
    //
    //     const tx = await program.methods.purchaseNfts(
    //
    //     )
    //         .accounts({
    //             signer: admin_account.publicKey,
    //             priceFeed:solUsdId
    //
    //         })
    //         .signers([admin_account]) // Include new_account as a signer
    //         .rpc();
    //
    // })


});



