import * as anchor from "@coral-xyz/anchor";

const airdropSol = async function (provider, publicKey, amountInSol) {
  const amountInLamports = amountInSol * anchor.web3.LAMPORTS_PER_SOL;
  const signature = await provider.connection.requestAirdrop(
    publicKey,
    amountInLamports
  );
  await provider.connection.confirmTransaction(signature, "confirmed");
};

export default airdropSol;
