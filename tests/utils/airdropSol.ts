import * as anchor from '@coral-xyz/anchor';
import { LAMPORTS_PER_SOL } from '@solana/web3.js';

const airdropSol = async function (provider, publicKey, amountInSol) {
    let account_balance = await provider.connection.getBalance(publicKey);
    if (account_balance < LAMPORTS_PER_SOL * 0.0001) {
        const amountInLamports = amountInSol * anchor.web3.LAMPORTS_PER_SOL;
        const signature = await provider.connection.requestAirdrop(
            publicKey,
            amountInLamports
        );
        await provider.connection.confirmTransaction(signature, 'confirmed');
    }
};

export default airdropSol;
