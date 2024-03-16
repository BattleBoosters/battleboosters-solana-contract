import * as anchor from '@coral-xyz/anchor';
import { BN } from '@coral-xyz/anchor';
import { sleep } from '@switchboard-xyz/common';
import { assert } from 'chai';
import { Battleboosters } from '../../target/types/battleboosters';
const joinFightCard = async function (
    provider: anchor.AnchorProvider,
    program: anchor.Program<Battleboosters>,
    admin_account,
    program_pda,
) {
    const program_data_before = await program.account.programData.fetch(
        program_pda
    );

    const [event_account, event_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('event'),
                new BN(0).toBuffer('le', 8),
            ],
            program.programId
        );
    const [event_link_account, event_link_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('event'),
                event_account.toBuffer(),
                provider.wallet.publicKey.toBuffer(),
                // new BN(0).toBuffer('le', 8),
            ],
            program.programId
        );

    console.log(event_account)

    const [fight_card_account, fight_card_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('fightCard'),
                event_account.toBuffer(),
                new BN(0).toBuffer(),
            ],
            program.programId
        );
    const [fight_card_link_account, fight_card_link_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('fightCard'),
                event_account.toBuffer(),
                provider.wallet.publicKey.toBuffer(),
                new BN(0).toBuffer(),
            ],
            program.programId
        );
    const [player_account_pda, player_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('player'),
                provider.wallet.publicKey.toBuffer(),
            ],
            program.programId
        );

    const player_account_pda_data = await program.account.playerData.fetch(
        player_account_pda
    );

    const [fighter_mintable_game_asset_pda, fighter_mintable_game_asset_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mintableGameAsset'),
                new BN(0).toBuffer(
                    'le',
                    8
                ),
            ],
            program.programId
        );
    const [fighter_mintable_game_asset_link_pda, fighter_mintable_game_asset_link_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('mintableGameAsset'),
                provider.wallet.publicKey.toBuffer(),
                new BN(0).toBuffer(
                    'le',
                    8
                ),

            ],
            program.programId
        );

    console.log(player_account_pda_data)

    // const [player_game_asset_link_pda, player_game_asset_link_bump] =
    //     anchor.web3.PublicKey.findProgramAddressSync(
    //         [
    //             Buffer.from('BattleBoosters'),
    //             Buffer.from('mintableGameAsset'),
    //             provider.wallet.publicKey.toBuffer(),
    //             new BN(player_game_asset_link_nonce).toBuffer('le', 8),
    //         ],
    //         program.programId
    //     );
    //
    // const player_game_asset_link_pda_data =
    //     await program.account.playerGameAssetLinkData.fetch(
    //         player_game_asset_link_pda
    //     );

    try {
        const tx = await program.methods
            .joinFightCard(
                new BN(0),
                0,
                new BN(0),
                null,
                null,
                null,
                null,
                new BN(0),
                null,
                null,
                null,
                null,
                { fighterBlue: {} }
            )
            .accounts({
                signer: provider.wallet.publicKey,
                program: program_pda,
                event: event_account,
                eventLink: event_link_account,
                fightCard: fight_card_account,
                fightCardLink: fight_card_link_account,
                fighterAsset: fighter_mintable_game_asset_pda,
                fighterLink: fighter_mintable_game_asset_link_pda,
                energyBoosterAsset: null,
                shieldBoosterAsset: null,
                pointsBoosterAsset: null,
                championsPassAsset: null,
                energyBoosterLink: null,
                shieldBoosterLink: null,
                pointsBoosterLink: null,
                championsPassLink: null,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([])
            .rpc();
    }catch (e) {
        console.log(e)
    }


    // const eventAccount = await program.account.eventData.fetch(
    //     event_account_one
    // );
    //
    // const program_data_after = await program.account.programData.fetch(
    //     program_pda
    // );

    // console.log(tx)
    // await sleep(2000);
    // const logs = await provider.connection.getParsedTransaction(
    //     tx,
    //     "confirmed"
    // );
    //
    // console.log(JSON.stringify(logs?.meta?.logMessages, undefined, 2));

    // return {
    //     program_data_before,
    //     eventAccount,
    //     program_data_after,
    // };
};

export {
    joinFightCard
}
