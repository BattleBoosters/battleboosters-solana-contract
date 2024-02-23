
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
import {PublicKey} from "@solana/web3.js";

// describe("battleboosters", () => {
//         it("check", ()=> {
//             const byteArray = [72,63,203,56,127,67,11,107,12,45,239,229,185,94,245,165,230,227,107,223,233,102,157,202,163,38,205,82,18,58,239,214,106,239,102,176,204,236,205,115,83,16,112,246,128,179,20,153,107,216,170,249,151,84,117,84,147,67,81,141,104,38,246,108]
//             const uint8Array = new Uint8Array(byteArray);
//             const programId = new PublicKey(uint8Array);
//
//             console.log(programId.toString());
//         })
//     }
// );
