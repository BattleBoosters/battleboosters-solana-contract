import * as anchor from "@coral-xyz/anchor";
import { BN, Program, web3 } from "@coral-xyz/anchor";
import { Battleboosters } from "../target/types/battleboosters";
import { assert } from "chai";
import airdropSol from "./utils/airdrop_sol";
import {
  TOKEN_PROGRAM_ID,
  AccountLayout,
  MintLayout,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
const { SystemProgram, SYSVAR_RENT_PUBKEY } = anchor.web3;
import {
  mplTokenMetadata,
  getMplTokenMetadataProgramId,
} from "@metaplex-foundation/mpl-token-metadata";
import { MPL_TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
} from "@solana/web3.js";
import { before } from "mocha";
import airdrop_sol from "./utils/airdrop_sol";
import { sleep } from "@switchboard-xyz/common";
import {
  AggregatorAccount,
  SwitchboardProgram,
} from "@switchboard-xyz/solana.js";
import InitializePlayerAccount from "./utils/initialize_player_account";
import { RandomnessService } from "@switchboard-xyz/solana-randomness-service";
import * as buffer from "buffer";
import account_init from "./utils/account_init";

describe("Purchase", () => {
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
    authority_bump,
  } = account_init(program);

  let switchboardProgram;
  let randomnessService;
  let lastPriceSolUsd;
  before(async () => {
    // try {
    //     randomnessService = await RandomnessService.fromProvider(provider);
    // }catch (e) {
    //     console.log(e)
    // }
    // console.log(randomnessService)
    // console.log("Randomness Service OK")

    switchboardProgram = await SwitchboardProgram.load(
      new Connection("https://api.mainnet-beta.solana.com")
    );

    // Check the latest SOL/USD price
    const aggregatorAccount = new AggregatorAccount(
      switchboardProgram,
      new anchor.web3.PublicKey("GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR")
    );
    lastPriceSolUsd = await aggregatorAccount.fetchLatestValue();
  });
  /**
     Player Purchase in game NFT assets
     **/

  it("Purchase successfully in-game assets for signer", async () => {
    // Start watching for the settled event before triggering the request
    const requestKeypair = anchor.web3.Keypair.generate();

    console.log("watching...");

    //Start watching for the settled event before triggering the request
    const settledRandomnessEventPromise = randomnessService.awaitSettledEvent(
      requestKeypair.publicKey
    );

    console.log("watched...");

    console.log("await Randomness Service");

    const [bank_pda, bank_bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("BattleBoosters"), Buffer.from("bank")],
      program.programId
    );

    const [user_bank_pda, user_bank_bump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("BattleBoosters"),
          Buffer.from("bank"),
          provider.wallet.publicKey.toBuffer(),
        ],
        program.programId
      );

    const [player_inventory_pda, player_inventory_bump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("BattleBoosters"),
          Buffer.from("inventory"),
          provider.wallet.publicKey.toBuffer(),
        ],
        program.programId
      );

    const [player_account_pda, player_account_bump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("BattleBoosters"),
          Buffer.from("player"),
          provider.wallet.publicKey.toBuffer(),
        ],
        program.programId
      );

    const playerAccountData = await program.account.playerData.fetch(
      player_account_pda
    );
    // MY_APP_PREFIX, COLLECTOR, recipient.key().as_ref(), player_account.order_nonce.to_le_bytes().as_ref()
    const [collector_pack_pda, collector_pack_bump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("BattleBoosters"),
          Buffer.from("collector"),
          provider.wallet.publicKey.toBuffer(),
          new BN(playerAccountData.orderNonce).toBuffer("le", 8),
        ],
        program.programId
      );

    const priceFeedAccount = new anchor.web3.PublicKey(
      "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR"
    );

    let programPDA = await program.account.programData.fetch(program_pda);

    const boosterQty = new anchor.BN(1);
    const fighterQty = new anchor.BN(2);
    const boosterPrice = programPDA.boosterPrice;
    const fighterPrice = programPDA.fighterPackPrice;

    const total = boosterQty
      .mul(boosterPrice)
      .add(fighterQty.mul(fighterPrice));
    const safeAmount =
      total.add(new BN(1)).toNumber() * (1 / lastPriceSolUsd.toNumber());

    const amountToSend = new anchor.BN(
      anchor.web3.LAMPORTS_PER_SOL * safeAmount
    ); // For example, 1 SOL

    // Create a transaction to transfer SOL from the signer to the bank_escrow PDA
    const transferTx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: user_bank_pda,
        lamports: amountToSend.toNumber(),
      })
    );

    // Sign and send the transaction
    await provider.sendAndConfirm(transferTx, []);
    const accountData = await provider.connection.getAccountInfo(user_bank_pda);
    const rentExemptionAmount =
      await provider.connection.getMinimumBalanceForRentExemption(
        accountData.data.length
      );

    try {
      // Initialize the player account first
      await InitializePlayerAccount(
        provider,
        provider.wallet.publicKey,
        program,
        program_pda
      );
      console.log("tx start");
      console.log(randomnessService.accounts.state);

      const tx = await program.methods
        .purchaseNfts(user_bank_bump, [
          {
            nftType: { booster: {} }, // Use the variant name as key for enum
            quantity: boosterQty,
          },
          {
            nftType: { fighterPack: {} }, // Use the variant name as key for enum
            quantity: fighterQty,
          },
        ])
        .accounts({
          signer: provider.wallet.publicKey,
          recipient: provider.wallet.publicKey,
          program: program_pda,
          playerAccount: player_account_pda,
          collectorPack: collector_pack_pda,
          bankEscrow: user_bank_pda,
          bank: bank_pda,
          priceFeed: priceFeedAccount,
          randomnessService: randomnessService.programId,
          randomnessRequest: requestKeypair.publicKey,
          randomnessEscrow: anchor.utils.token.associatedAddress({
            mint: randomnessService.accounts.mint,
            owner: requestKeypair.publicKey,
          }),
          randomnessState: randomnessService.accounts.state,
          randomnessMint: randomnessService.accounts.mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([requestKeypair]) // Include new_account as a signer
        .rpc();

      console.log("tx await");
      console.log(`[TX] requestRandomness: ${tx}`);
      //Await the response from the Switchboard Service

      const [settledRandomnessEvent, settledSlot] =
        await settledRandomnessEventPromise;
      console.log(settledRandomnessEventPromise);
      console.log(
        `[EVENT] SimpleRandomnessV1SettledEvent\n${JSON.stringify(
          {
            ...settledRandomnessEvent,

            // why is anchor.BN so annoying with hex strings?
            requestSlot: settledRandomnessEvent.requestSlot.toNumber(),
            settledSlot: settledRandomnessEvent.settledSlot.toNumber(),
            randomness: `[${new Uint8Array(
              settledRandomnessEvent.randomness
            )}]`,
          },
          undefined,
          2
        )}`
      );

      assert.equal(
        settledRandomnessEvent.user.toBase58(),
        provider.wallet.publicKey.toBase58(),
        "User should be the same as the provider wallet"
      );
      assert.equal(
        settledRandomnessEvent.request.toBase58(),
        requestKeypair.publicKey.toBase58(),
        "Request should be the same as the provided request keypair"
      );
      assert.equal(
        settledRandomnessEvent.isSuccess,
        true,
        "Request did not complete successfully"
      );

      const latency = settledRandomnessEvent.settledSlot
        .sub(settledRandomnessEvent.requestSlot)
        .toNumber();
      console.log(
        `\nRandomness: [${new Uint8Array(
          settledRandomnessEvent.randomness
        )}]\nRequest completed in ${latency} slots!\n`
      );

      // wait for RPC
      await sleep(2000);
      const logs = await provider.connection.getParsedTransaction(
        tx,
        "confirmed"
      );

      console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));

      const playerInventoryAccountAfter =
        await program.account.inventoryData.fetch(player_inventory_pda);
      assert.isTrue(
        playerInventoryAccountAfter.boosterMintAllowance.eq(boosterQty)
      );
      assert.isTrue(
        playerInventoryAccountAfter.fighterMintAllowance.eq(fighterQty)
      );
      assert.isTrue(playerInventoryAccountAfter.isInitialized);

      // Test if bank PDA received the correct SOL amount
      const bankPdaBalance = await provider.connection.getBalance(bank_pda);
      assert.equal(
        bankPdaBalance,
        amountToSend.toNumber() - rentExemptionAmount
      );
      // Test the user PDA is rent exempt
      const userBankPdaBalance = await provider.connection.getBalance(
        user_bank_pda
      );
      assert.equal(userBankPdaBalance, rentExemptionAmount);
    } catch (e) {
      console.log(e);
    }
  });

  it("Purchase successfully in-game assets for another recipient", async () => {
    const newRecipient = anchor.web3.Keypair.generate();

    const [bank_pda, bank_bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("BattleBoosters"), Buffer.from("bank")],
      program.programId
    );

    const [user_bank_pda, user_bank_bump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("BattleBoosters"),
          Buffer.from("bank"),
          provider.wallet.publicKey.toBuffer(),
        ],
        program.programId
      );

    const [player_inventory_pda, player_inventory_bump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("BattleBoosters"),
          Buffer.from("inventory"),
          newRecipient.publicKey.toBuffer(),
        ],
        program.programId
      );

    const priceFeedAccount = new anchor.web3.PublicKey(
      "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR"
    );

    const programPDA = await program.account.programData.fetch(program_pda);

    const boosterQty = new anchor.BN(1);
    const fighterQty = new anchor.BN(2);
    const boosterPrice = programPDA.boosterPrice;
    const fighterPrice = programPDA.fighterPackPrice;

    const total = boosterQty
      .mul(boosterPrice)
      .add(fighterQty.mul(fighterPrice));
    const safeAmount =
      total.add(new BN(1)).toNumber() * (1 / lastPriceSolUsd.toNumber());

    const amountToSend = new anchor.BN(
      anchor.web3.LAMPORTS_PER_SOL * safeAmount
    ); // For example, 1 SOL

    // Create a transaction to transfer SOL from the signer to the bank_escrow PDA
    const transferTx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: user_bank_pda,
        lamports: amountToSend.toNumber(),
      })
    );

    // Sign and send the transaction
    await provider.sendAndConfirm(transferTx, []);

    try {
      // Initialize the player account first
      await InitializePlayerAccount(
        provider,
        newRecipient.publicKey,
        program,
        program_pda
      );

      const tx = await program.methods
        .purchaseNfts(user_bank_bump, [
          {
            nftType: { booster: {} }, // Use the variant name as key for enum
            quantity: boosterQty,
          },
          {
            nftType: { fighterPack: {} }, // Use the variant name as key for enum
            quantity: fighterQty,
          },
        ])
        .accounts({
          signer: provider.wallet.publicKey,
          recipient: newRecipient.publicKey,
          program: program_pda,
          //playerInventory: player_inventory_pda,
          bankEscrow: user_bank_pda,
          bank: bank_pda,
          priceFeed: priceFeedAccount,
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

      const playerInventoryAccountAfter =
        await program.account.inventoryData.fetch(player_inventory_pda);
      assert.isTrue(
        playerInventoryAccountAfter.boosterMintAllowance.eq(boosterQty)
      );
      assert.isTrue(
        playerInventoryAccountAfter.fighterMintAllowance.eq(fighterQty)
      );
      assert.isTrue(playerInventoryAccountAfter.isInitialized);
    } catch (e) {
      console.log(e);
    }
  });

  it("Purchase error insuficient amount in purchase request", async () => {
    const newRecipient = anchor.web3.Keypair.generate();

    const [bank_pda, bank_bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("BattleBoosters"), Buffer.from("bank")],
      program.programId
    );

    const [user_bank_pda, user_bank_bump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("BattleBoosters"),
          Buffer.from("bank"),
          provider.wallet.publicKey.toBuffer(),
        ],
        program.programId
      );

    const [player_inventory_pda, player_inventory_bump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("BattleBoosters"),
          Buffer.from("inventory"),
          newRecipient.publicKey.toBuffer(),
        ],
        program.programId
      );

    const priceFeedAccount = new anchor.web3.PublicKey(
      "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR"
    );

    const programPDA = await program.account.programData.fetch(program_pda);

    const boosterQty = new anchor.BN(0);
    const fighterQty = new anchor.BN(0);
    const boosterPrice = programPDA.boosterPrice;
    const fighterPrice = programPDA.fighterPackPrice;

    const total = boosterQty
      .mul(boosterPrice)
      .add(fighterQty.mul(fighterPrice));
    const safeAmount =
      total.add(new BN(1)).toNumber() * (1 / lastPriceSolUsd.toNumber());

    const amountToSend = new anchor.BN(
      anchor.web3.LAMPORTS_PER_SOL * safeAmount
    ); // For example, 1 SOL

    // Create a transaction to transfer SOL from the signer to the bank_escrow PDA
    const transferTx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: user_bank_pda,
        lamports: amountToSend.toNumber(),
      })
    );

    // Sign and send the transaction
    await provider.sendAndConfirm(transferTx, []);

    try {
      // Initialize the player account first
      await InitializePlayerAccount(
        provider,
        newRecipient.publicKey,
        program,
        program_pda
      );

      const tx = await program.methods
        .purchaseNfts(user_bank_bump, [
          {
            nftType: { booster: {} }, // Use the variant name as key for enum
            quantity: boosterQty,
          },
          {
            nftType: { fighterPack: {} }, // Use the variant name as key for enum
            quantity: fighterQty,
          },
        ])
        .accounts({
          signer: provider.wallet.publicKey,
          recipient: newRecipient.publicKey,
          program: program_pda,
          // playerInventory: player_inventory_pda,
          bankEscrow: user_bank_pda,
          bank: bank_pda,
          priceFeed: priceFeedAccount,
        })
        .signers([]) // Include new_account as a signer
        .rpc();
    } catch (e) {
      assert.include(e.message, "Insufficient amount in purchase request.");
    }
  });

  it("Purchase unsuccessfully not enough money", async () => {
    const newRecipient = anchor.web3.Keypair.generate();

    const [bank_pda, bank_bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("BattleBoosters"), Buffer.from("bank")],
      program.programId
    );

    const [user_bank_pda, user_bank_bump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("BattleBoosters"),
          Buffer.from("bank"),
          provider.wallet.publicKey.toBuffer(),
        ],
        program.programId
      );

    const [player_inventory_pda, player_inventory_bump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("BattleBoosters"),
          Buffer.from("inventory"),
          newRecipient.publicKey.toBuffer(),
        ],
        program.programId
      );

    const priceFeedAccount = new anchor.web3.PublicKey(
      "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR"
    );

    const programPDA = await program.account.programData.fetch(program_pda);

    const boosterQty = new anchor.BN(1);
    const fighterQty = new anchor.BN(2);
    const boosterPrice = programPDA.boosterPrice;
    const fighterPrice = programPDA.fighterPackPrice;

    const total = boosterQty
      .mul(boosterPrice)
      .add(fighterQty.mul(fighterPrice));
    const safeAmount =
      total.sub(new BN(10)).toNumber() * (1 / lastPriceSolUsd.toNumber());

    const amountToSend = new anchor.BN(
      anchor.web3.LAMPORTS_PER_SOL * safeAmount
    ); // For example, 1 SOL

    // Create a transaction to transfer SOL from the signer to the bank_escrow PDA
    const transferTx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: user_bank_pda,
        lamports: amountToSend.toNumber(),
      })
    );

    // Sign and send the transaction
    await provider.sendAndConfirm(transferTx, []);

    try {
      // Initialize the player account first
      await InitializePlayerAccount(
        provider,
        newRecipient.publicKey,
        program,
        program_pda
      );

      const tx = await program.methods
        .purchaseNfts(user_bank_bump, [
          {
            nftType: { booster: {} }, // Use the variant name as key for enum
            quantity: boosterQty,
          },
          {
            nftType: { fighterPack: {} }, // Use the variant name as key for enum
            quantity: fighterQty,
          },
        ])
        .accounts({
          signer: provider.wallet.publicKey,
          recipient: newRecipient.publicKey,
          program: program_pda,
          //playerInventory: player_inventory_pda,
          bankEscrow: user_bank_pda,
          bank: bank_pda,
          priceFeed: priceFeedAccount,
        })
        .signers([]) // Include new_account as a signer
        .rpc();
    } catch (e) {
      assert.include(e.message, "Insufficient funds.");
    }
  });
});
