import * as anchor from '@coral-xyz/anchor';
import { BN, Program } from '@coral-xyz/anchor';
import { Battleboosters } from '../target/types/battleboosters';
import { assert } from 'chai';
describe('Create fight card', () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Battleboosters as Program<Battleboosters>;
    const admin_account = anchor.web3.Keypair.generate();
    const program_account = anchor.web3.Keypair.generate();
    const random_account = anchor.web3.Keypair.generate();
    let event_account;
    let fight_card_account;

    before('Initialize', async () => {
        let rarity = {
            common: {
                powerMin: 10,
                powerMax: 100,
                lifespanMin: 10,
                lifespanMax: 100,
                energyMin: 10,
                energyMax: 100,
            },
        };

        await program.methods
            .initialize(
                admin_account.publicKey,
                rarity,
                Buffer.from([1, 2, 3, 4, 5]),
                new BN(100 * anchor.web3.LAMPORTS_PER_SOL),
                new BN(1 * anchor.web3.LAMPORTS_PER_SOL)
            )
            .accounts({
                creator: provider.wallet.publicKey,
                program: program_account.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([program_account])
            .rpc();

        // Airdrop random_account
        const airdrop_random_account = await provider.connection.requestAirdrop(
            random_account.publicKey,
            100 * anchor.web3.LAMPORTS_PER_SOL
        );
        const latestBlockHash = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: airdrop_random_account,
        });
        // Airdrop random_account
        const airdrop_admin_account = await provider.connection.requestAirdrop(
            admin_account.publicKey,
            100 * anchor.web3.LAMPORTS_PER_SOL
        );
        const latestBlockHash2 = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction({
            blockhash: latestBlockHash2.blockhash,
            lastValidBlockHeight: latestBlockHash2.lastValidBlockHeight,
            signature: airdrop_admin_account,
        });

        let fetchedProgram = await program.account.programData.fetch(
            program_account.publicKey
        );
        const [event_account_one] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('event'),
                    new BN(fetchedProgram.eventCounter).toBuffer('le', 8),
                ],
                program.programId
            );

        event_account = event_account_one;

        await program.methods
            .createNewEvent(new BN(1713045216), new BN(1711045216))
            .accounts({
                creator: admin_account.publicKey,
                program: program_account.publicKey,
                event: event_account_one,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin_account])
            .rpc();
    });

    it('Should add a new fight card', async () => {
        let eventAccountData = await program.account.eventData.fetch(
            event_account
        );
        const [fight_card_account_one, event_account_one_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('fightCard'),
                    new BN(eventAccountData.fightCardIdCounter).toBuffer(),
                ],
                program.programId
            );

        fight_card_account = fight_card_account_one;

        const fightCardData = {
            id: new BN(112),
            eventPubkey: event_account,
            tournament: { mainCard: {} },
            titleFight: true,
            fightStatsFighter1: null,
            fightStatsFighter2: null,
            fightDuration: null,
            result: null,
            winner: null,
        };

        const tx = await program.methods
            .createNewFightCard(fightCardData)
            .accounts({
                creator: admin_account.publicKey,
                program: program_account.publicKey,
                event: event_account,
                fightCard: fight_card_account_one,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin_account])
            .rpc();

        let fetchedFightCard = await program.account.fightCardData.fetch(
            fight_card_account_one
        );
        assert.equal(fetchedFightCard.id.eq(new BN(112)), true);
        assert.equal(fetchedFightCard.fightDuration, null);
        assert.equal(fetchedFightCard.titleFight, true);
        assert.equal(fetchedFightCard.fightStatsFighter1, null);
        assert.equal(fetchedFightCard.fightStatsFighter2, null);
        assert.deepEqual(
            fetchedFightCard.eventPubkey.equals(event_account),
            true
        );
        assert.equal(fetchedFightCard.result, null);
        assert.deepEqual(fetchedFightCard.tournament, { mainCard: {} });

        let fetchedEvent = await program.account.eventData.fetch(event_account);
        assert.equal(fetchedEvent.fightCardIdCounter, 1);
    });

    it('Should update fight card', async () => {
        /* TODO:
                Test all the Fight Card data
         */

        const fightCardData = {
            id: new BN(112),
            eventPubkey: event_account,
            tournament: { mainCard: {} },
            titleFight: false,
            fightStatsFighter1: null,
            fightStatsFighter2: null,
            fightDuration: null,
            result: null,
            winner: null,
        };

        const tx = await program.methods
            .updateFightCard(0, fightCardData)
            .accounts({
                creator: admin_account.publicKey,
                program: program_account.publicKey,
                event: event_account,
                fightCard: fight_card_account,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin_account])
            .rpc();

        let fetchedFightCard = await program.account.fightCardData.fetch(
            fight_card_account
        );
        assert.equal(fetchedFightCard.id.eq(new BN(112)), true);
        assert.equal(fetchedFightCard.fightDuration, null);
        assert.equal(fetchedFightCard.titleFight, false);
        assert.equal(fetchedFightCard.fightStatsFighter1, null);
        assert.equal(fetchedFightCard.fightStatsFighter2, null);
        assert.deepEqual(
            fetchedFightCard.eventPubkey.equals(event_account),
            true
        );
        assert.equal(fetchedFightCard.result, null);
        assert.deepEqual(fetchedFightCard.tournament, { mainCard: {} });

        let fetchedEvent = await program.account.eventData.fetch(event_account);
        assert.equal(fetchedEvent.fightCardIdCounter, 1);
    });
});
