
import * as anchor from "@coral-xyz/anchor";
import {BN, Program, web3} from "@coral-xyz/anchor";
import { Battleboosters } from "../target/types/battleboosters";


import {
    mplTokenMetadata,
    createProgrammableNft,
    MPL_TOKEN_METADATA_PROGRAM_ID,
    createMplTokenMetadataProgram,
    createNft, fetchDigitalAsset, createV1, TokenStandard
} from '@metaplex-foundation/mpl-token-metadata'
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import {generateSigner, keypairIdentity, percentAmount} from "@metaplex-foundation/umi";
import airdropSol from "./utils/airdrop_sol";

describe("battleboosters", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;

    // Use the RPC endpoint of your choice.
    const umi = createUmi('https://solana-devnet.g.alchemy.com/v2/gBl6MYuZ9TWoXXTPbY9-z0E0Trf0UPW4')
        .use(mplTokenMetadata())

    it("Is initialized!", async () => {

        const metadata = {
            name: "Solana Gold",
            symbol: "GOLDSOL",
            uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
        };


        const admin_account = anchor.web3.Keypair.fromSecretKey( new Uint8Array([
                223,  59, 101, 153, 143,  21,  27,  11, 169, 175,  70,
                197,  18, 124,  44,  79, 218,  51, 196, 199, 144, 211,
                97,  87,  75, 138,  62, 180, 106, 250, 127, 172,   6,
                144, 226, 141, 181, 189,  96,  98, 164, 204, 232, 161,
                130, 182,  19, 162,  30, 200, 230, 194,  32,  45,  49,
                175, 101, 113,  85, 206, 140,   5, 206, 107
            ]),
        )

        const mint = createSignerFromKeypair( admin_account, admin_account)



        umi.use(keypairIdentity(mint))
        await createV1(umi, {
            mint,
            authority: mint,
            name: 'My NFT',
            uri: metadata.uri,
            sellerFeeBasisPoints: percentAmount(5.5),
            tokenStandard: TokenStandard.ProgrammableNonFungible,
            isCollection: true,
            isMutable: true
        }).sendAndConfirm(umi)

    })}
);
