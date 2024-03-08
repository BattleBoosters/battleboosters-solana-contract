import * as anchor from "@coral-xyz/anchor";
import {BN, Program, web3} from "@coral-xyz/anchor";
import { Battleboosters } from "../target/types/battleboosters";
import {assert} from "chai";
import airdropSol from "./utils/airdrop_sol";
import {TOKEN_PROGRAM_ID, AccountLayout, MintLayout, ASSOCIATED_TOKEN_PROGRAM_ID} from '@solana/spl-token';
const { SystemProgram, SYSVAR_RENT_PUBKEY } = anchor.web3;
import {mplTokenMetadata, getMplTokenMetadataProgramId} from "@metaplex-foundation/mpl-token-metadata";
import {MPL_TOKEN_METADATA_PROGRAM_ID} from "@metaplex-foundation/mpl-token-metadata";
import {Connection, LAMPORTS_PER_SOL, PublicKey, Transaction} from "@solana/web3.js";
import {before} from "mocha";
import airdrop_sol from "./utils/airdrop_sol";
import { sleep } from "@switchboard-xyz/common";
import {AggregatorAccount, SwitchboardProgram} from "@switchboard-xyz/solana.js";
import InitializePlayerAccount from "./utils/initialize_player_account";
import { RandomnessService } from "@switchboard-xyz/solana-randomness-service";
import * as buffer from "buffer";
import account_init from "./utils/account_init";

describe.only("Random Mintable Asset", () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);
    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;

    const {
        admin_account,
        metadata_pubkey,
        bank_pda,
        bank_bump,
        program_pda,
        program_bump,
        rarity_pda,
        rarity_bump,
        mint_authority_account,
        authority_bump
    } = account_init(program);

    it("Open nft pack randomly", async () => {

        await InitializePlayerAccount(provider, provider.wallet.publicKey, program, program_pda);


        const program_pda_data = await program.account.programData.fetch(program_pda);
        const [mintable_game_asset_pda, mintable_game_asset_bump]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mintableGameAsset"),
                new BN(program_pda_data.mintableGameAssetNonce).toBuffer("le", 8)
            ], program.programId);

        const [player_account_pda, player_account_bump]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("player"),
                provider.wallet.publicKey.toBuffer()
            ], program.programId);


        const player_account_pda_data = await program.account.playerData.fetch(player_account_pda);

        const [player_game_asset_link_pda, player_game_asset_link_bump]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("mintableGameAsset"),
                provider.wallet.publicKey.toBuffer(),
                new BN(player_account_pda_data.playerGameAssetLinkNonce).toBuffer("le", 8)
            ], program.programId);

        const [collector_pack_pda, collector_pack_bump]  = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("BattleBoosters"),
                Buffer.from("collector"),
                provider.wallet.publicKey.toBuffer(),
                new BN(player_account_pda_data.orderNonce).toBuffer("le", 8)
            ], program.programId);

        const tx = await program.methods.testGiftCollectorPack().accounts({
            signer: provider.wallet.publicKey,
            recipient: provider.wallet.publicKey,
            program: program_pda,
            playerAccount: player_account_pda,
            collectorPack: collector_pack_pda,

        }).signers([]).rpc()

        const collector_pack_pda_data = await program.account.collectorPack.fetch(collector_pack_pda);
        assert.isTrue(collector_pack_pda_data.boosterMintAllowance.eq(new BN(3)));
        assert.isTrue(collector_pack_pda_data.fighterMintAllowance.eq(new BN(2)));

        try {
            const tx2 = await program.methods.generateRandomMintableGameAsset(
                new BN(player_account_pda_data.playerGameAssetLinkNonce),
                {
                    nftType: { fighterPack: {} }, // Use the variant name as key for enum
                }
            ).accounts({
                signer: provider.wallet.publicKey,
                program: program_pda,
                playerAccount: player_account_pda,
                collectorPack: collector_pack_pda,
                rarity: rarity_pda,
                playerGameAssetLink: player_game_asset_link_pda,
                mintableGameAsset: mintable_game_asset_pda,
            }).signers([]).rpc()

            console.log(tx2)
            await sleep(2000);
            const logs = await provider.connection.getParsedTransaction(
                tx2,
                "confirmed"
            );

            console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));


            const pre_mint_pda_data = await program.account.mintableGameAssetData.fetch(mintable_game_asset_pda);

            const program_pda_data_2 = await program.account.programData.fetch(program_pda);
            const [mintable_game_asset_pda_2, mintable_game_asset_bump_2]  = anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from("BattleBoosters"),
                    Buffer.from("mintableGameAsset"),
                    new BN(program_pda_data_2.mintableGameAssetNonce).toBuffer("le", 8)
                ], program.programId);

            const [player_account_pda_2, player_account_bump_2]  = anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from("BattleBoosters"),
                    Buffer.from("player"),
                    provider.wallet.publicKey.toBuffer()
                ], program.programId);


            const player_account_pda_data_2 = await program.account.playerData.fetch(player_account_pda);

            const [player_game_asset_link_pda_2, player_game_asset_link_bump_2]  = anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from("BattleBoosters"),
                    Buffer.from("mintableGameAsset"),
                    provider.wallet.publicKey.toBuffer(),
                    new BN(player_account_pda_data_2.playerGameAssetLinkNonce).toBuffer("le", 8)
                ], program.programId);

            const [collector_pack_pda_2, collector_pack_bump_2]  = anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from("BattleBoosters"),
                    Buffer.from("collector"),
                    provider.wallet.publicKey.toBuffer(),
                    new BN(player_account_pda_data_2.orderNonce).toBuffer("le", 8)
                ], program.programId);

            const tx3 = await program.methods.generateRandomMintableGameAsset(
                new BN(player_account_pda_data_2.playerGameAssetLinkNonce),
                {
                    nftType: { fighterPack: {} }, // Use the variant name as key for enum
                }
            ).accounts({
                signer: provider.wallet.publicKey,
                program: program_pda,
                playerAccount: player_account_pda_2,
                collectorPack: collector_pack_pda_2,
                rarity: rarity_pda,
                playerGameAssetLink: player_game_asset_link_pda_2,
                mintableGameAsset: mintable_game_asset_pda_2,
            }).signers([]).rpc()

            console.log(tx3)
            await sleep(2000);
            const logs2 = await provider.connection.getParsedTransaction(
                tx3,
                "confirmed"
            );

            console.log(JSON.stringify(logs2?.meta?.logMessages, undefined, 2));


        }catch (e) {
            console.log(e)
        }

    })
})
