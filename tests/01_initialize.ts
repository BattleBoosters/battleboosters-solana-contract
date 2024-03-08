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

describe.only("Initialize", () => {

    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);

    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;
    let switchboardProgram;

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

    let randomnessService;
    let lastPriceSolUsd;
    it("Initialize", async () => {

        // try {
        //     randomnessService = await RandomnessService.fromProvider(provider);
        // }catch (e) {
        //     console.log(e)
        // }
        // console.log(randomnessService)
        // console.log("Randomness Service OK")

        switchboardProgram = await SwitchboardProgram.load(
            new Connection("https://api.mainnet-beta.solana.com"),
        );

        // Check the latest SOL/USD price
        const aggregatorAccount = new AggregatorAccount(switchboardProgram, new anchor.web3.PublicKey("GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR"));
        lastPriceSolUsd = await aggregatorAccount.fetchLatestValue();



        const programInfo = await provider.connection.getAccountInfo(new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"))
        // if (programInfo === null) {
        //     throw new Error('Program has not been deployed');
        // }
        // if (!programInfo.executable) {
        //     throw new Error('Program is not executable');
        // }

        let admin_account_balance = await provider.connection.getBalance(admin_account.publicKey);
        console.log(admin_account_balance)
         if (admin_account_balance < LAMPORTS_PER_SOL * 2){
             await airdrop_sol(provider, admin_account.publicKey, 10);
         }


        try {

            await program.account.programData.fetch(program_pda);
        } catch (e) {

            const tx = await program.methods.initialize(
                authority_bump,
                bank_bump,
                admin_account.publicKey,
                new BN(1),
                new BN(1),
                5
            )
                .accounts({
                    creator: admin_account.publicKey,
                    program: program_pda,
                    bank: bank_pda,
                    mintAuthority: mint_authority_account,
                    //systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([admin_account]) // Include new_account as a signer
                .rpc();

            const programAccount = await program.account.programData.fetch(program_pda);
            assert.equal(programAccount.eventNonce.eq(new BN(0)),  true);
            assert.equal(programAccount.mintableGameAssetNonce.eq(new BN(0)),  true);
            assert.deepEqual(programAccount.adminPubkey, admin_account.publicKey);
            assert.equal(programAccount.fighterPackPrice.eq(new BN(1)), true)
            assert.equal(programAccount.boosterPrice.eq(new BN(1)), true)
            assert.equal(programAccount.fighterPackAmount, 5)
        }

        try {
            await program.account.rarityData.fetch(rarity_pda);
        } catch (e) {

             try {
                const tx2 = await program.methods.initializeRarity(
                    [
                        {
                            common: {

                                energy: { min: 100, max: 150 },
                                power: { min: 100, max: 150 },
                                lifespan: { min: 100, max: 150 },
                                }
                            },
                        {uncommon: {
                                energy: { min: 150, max: 200 },
                                power: { min: 150, max: 200 },
                                lifespan: { min: 150, max: 200 },
                            }},
                        {rare: {
                                energy: { min: 200, max: 250 },
                                power: { min: 200, max: 250 },
                                lifespan: { min: 200, max: 250 },
                            }},
                        {epic: {
                                energy: { min: 250, max: 300 },
                                power: { min: 250, max: 300 },
                                lifespan: { min: 250, max: 300 },
                            }},
                        {legendary: {
                                energy: { min: 300, max: 350 },
                                power: { min: 300, max: 350 },
                                lifespan: { min: 300, max: 350 },
                            }},
                    ],
                    [

                        {
                            common: {
                                value: { min: 100, max: 150 },
                            }
                        },
                        {
                            uncommon: {
                                value: { min: 150, max: 200 },

                            }
                        },
                        {
                            rare: {
                                value: { min: 200, max: 250 },
                            },
                        },
                        {
                            epic: {
                                value: { min: 250, max: 300 }
                            },
                        },
                        {
                            legendary: {
                                value: { min: 300, max: 350 }
                            },
                        }
                    ],
                    [

                        {
                            common: {
                                value: { min: 100, max: 150 },
                            }
                        },
                        {
                            uncommon: {
                                value: { min: 150, max: 200 },

                            }
                        },
                        {
                            rare: {
                                value: { min: 200, max: 250 },
                            },
                        },
                        {
                            epic: {
                                value: { min: 250, max: 300 }
                            },
                        },
                        {
                            legendary: {
                                value: { min: 300, max: 350 }
                            },
                        }
                    ],
                    [

                        {
                            common: {
                                value: { min: 100, max: 150 },
                            }
                        },
                        {
                            uncommon: {
                                value: { min: 150, max: 200 },

                            }
                        },
                        {
                            rare: {
                                value: { min: 200, max: 250 },
                            },
                        },
                        {
                            epic: {
                                value: { min: 250, max: 300 }
                            },
                        },
                        {
                            legendary: {
                                value: { min: 300, max: 350 }
                            },
                        }
                    ],
                    Buffer.from([60, 25, 10, 4, 1]),
                    Buffer.from([60, 25, 10, 4, 1]),
                )
                    .accounts({
                        creator: admin_account.publicKey,
                        rarity: rarity_pda,
                        //systemProgram: anchor.web3.SystemProgram.programId,
                    })
                    .signers([admin_account]) // Include new_account as a signer
                    .rpc();


                const rarityData = await program.account.rarityData.fetch(rarity_pda);
                assert.isTrue(rarityData.isInitialized);
                assert.deepEqual(rarityData.boosterProbabilities.equals(Buffer.from([60, 25, 10, 4, 1])), true);
                assert.deepEqual(rarityData.fighterProbabilities.equals(Buffer.from([60, 25, 10, 4, 1])), true);
                assert.deepEqual(rarityData.energyBooster, [

                    {
                        common: {
                            value: { min: 100, max: 150 },
                        }
                    },
                    {
                        uncommon: {
                            value: { min: 150, max: 200 },

                        }
                    },
                    {
                        rare: {
                            value: { min: 200, max: 250 },
                        },
                    },
                    {
                        epic: {
                            value: { min: 250, max: 300 }
                        },
                    },
                    {
                        legendary: {
                            value: { min: 300, max: 350 }
                        },
                    }
                ]);
                assert.deepEqual(rarityData.fighter, [
                    {
                        common: {

                            energy: { min: 100, max: 150 },
                            power: { min: 100, max: 150 },
                            lifespan: { min: 100, max: 150 },
                        }
                    },
                    {uncommon: {
                            energy: { min: 150, max: 200 },
                            power: { min: 150, max: 200 },
                            lifespan: { min: 150, max: 200 },
                        }},
                    {rare: {
                            energy: { min: 200, max: 250 },
                            power: { min: 200, max: 250 },
                            lifespan: { min: 200, max: 250 },
                        }},
                    {epic: {
                            energy: { min: 250, max: 300 },
                            power: { min: 250, max: 300 },
                            lifespan: { min: 250, max: 300 },
                        }},
                    {legendary: {
                            energy: { min: 300, max: 350 },
                            power: { min: 300, max: 350 },
                            lifespan: { min: 300, max: 350 },
                        }},
                ]);
             } catch (e) {
                console.log(e)
             }
        }

    })

    it( "Initialize player account", async () => {
        const customOwner = anchor.web3.Keypair.generate();

        // Initialize the player account first
        const [player_inventory_pda, player_account_pda] = await InitializePlayerAccount(provider, customOwner.publicKey, program, program_pda);

        // const playerInventory = await program.account.inventoryData.fetch(player_inventory_pda);
        // assert.isTrue(playerInventory.boosterMintAllowance.eq(new BN(0)))
        // assert.isTrue(playerInventory.fighterMintAllowance.eq(new BN(0)))
        // assert.isTrue(playerInventory.isInitialized);

        const playerAccount = await program.account.playerData.fetch(player_account_pda);
        assert.isTrue(playerAccount.orderNonce.eq(new BN(0)));
        assert.isTrue(playerAccount.playerGameAssetLinkNonce.eq(new BN(0)));
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



