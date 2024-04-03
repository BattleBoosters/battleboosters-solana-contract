import * as anchor from '@coral-xyz/anchor';
import { BN, Program } from '@coral-xyz/anchor';
import account_init from './utils/initAccounts';
import { Battleboosters } from '../target/types/battleboosters';
import { updateFightCard } from './utils/createUpdateFightCard';
import { updateEvent } from './utils/createUpdateEvent';
import { sleep } from '@switchboard-xyz/common';

describe('Collect Rewards', () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);
    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;

    const {
        admin_account,
        unauthorized_account,
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

    it('Collect Reward', async () => {
        // Test code here

        const [event_account, event_account_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('event'),
                    new BN(0).toBuffer('le', 8),
                ],
                program.programId
            );

        const [rank_pda, rank_pda_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('rank'),
                    event_account.toBuffer(),
                    new BN(0).toBuffer('le', 8),
                    //admin_account.publicKey.toBuffer()
                    // new BN(0).toBuffer('le', 8),
                ],
                program.programId
            );
        const rank_pda_data = await program.account.rankData.fetch(rank_pda);

        const [player_account_pda, player_account_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('player'),
                    rank_pda_data.playerAccount.toBuffer(),
                    //admin_account.publicKey.toBuffer(),
                ],
                program.programId
            );

        const player_account_pda_data = await program.account.playerData.fetch(
            player_account_pda
        );
        console.log('player_account_pda_data.orderNonce');
        console.log(player_account_pda_data.orderNonce);
        const [mystery_box_pda, mystery_box_pda_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('mysteryBox'),
                    rank_pda_data.playerAccount.toBuffer(),
                    new BN(player_account_pda_data.orderNonce).toBuffer(
                        'le',
                        8
                    ),
                ],
                program.programId
            );

        console.log(event_account);
        console.log(rank_pda);
        console.log(player_account_pda);
        console.log(mystery_box_pda);
        console.log(rarity_pda);
        try {
            let tx = await program.methods
                .collectRewards(new BN(0), new BN(0))
                .accounts({
                    signer: provider.wallet.publicKey,
                    event: event_account,
                    bank: bank_pda,
                    rank: rank_pda,
                    program: program_pda,
                    playerAccount: player_account_pda,
                    mysteryBox: mystery_box_pda,
                    rarity: rarity_pda,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([])
                .rpc();

            await sleep(2000);
            const logs = await provider.connection.getParsedTransaction(
                tx,
                'confirmed'
            );

            console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));

            // let event_data = await program.account.eventData.fetch(event_account);
            // console.log(event_data.randomness);
        } catch (e) {
            console.log('issue');
            console.log(e);
        }
    });
});
