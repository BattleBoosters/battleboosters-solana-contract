import * as anchor from '@coral-xyz/anchor';
import { BN } from '@coral-xyz/anchor';
import { sleep } from '@switchboard-xyz/common';
import { assert } from 'chai';
import { Battleboosters } from '../../target/types/battleboosters';
import PrepareLookupTable from './prepareLookupTable';

// Pseudo-code to illustrate the concept

function findProgramAddressSync(
    provider: anchor.AnchorProvider,
    program: anchor.Program<Battleboosters>,
    asset_nonce,
    asset_link_nonce
) {
    let asset_nonce_pda = null;
    if (asset_nonce !== null) {
        const [mintable_game_asset_pda, mintable_game_asset_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('mintableGameAsset'),
                    new BN(asset_nonce).toBuffer('le', 8),
                ],
                program.programId
            );
        asset_nonce_pda = mintable_game_asset_pda;
    }

    let asset_link_nonce_pda = null;
    if (asset_link_nonce !== null) {
        const [mintable_game_asset_link_pda, mintable_game_asset_link_bump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from('BattleBoosters'),
                    Buffer.from('mintableGameAsset'),
                    new BN(asset_link_nonce).toBuffer('le', 8),
                    provider.wallet.publicKey.toBuffer(),
                    //admin_account.publicKey.toBuffer(),
                ],
                program.programId
            );
        asset_link_nonce_pda = mintable_game_asset_link_pda;
    }

    return [asset_nonce_pda, asset_link_nonce_pda];
}
const joinFightCard = async function (
    provider: anchor.AnchorProvider,
    program: anchor.Program<Battleboosters>,
    admin_account,
    program_pda,
    fighterColorSide,
    event_nonce,
    fight_card_nonce,
    fighter_mintable_asset_nonce,
    fighter_mintable_asset_link_nonce,
    energy_mintable_asset_nonce = null,
    energy_mintable_asset_link_nonce = null,
    shield_mintable_asset_nonce = null,
    shield_mintable_asset_link_nonce = null,
    points_mintable_asset_nonce = null,
    points_mintable_asset_link_nonce = null
) {
    const [event_account, event_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('event'),
                new BN(event_nonce).toBuffer('le', 8),
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
                //admin_account.publicKey.toBuffer()
                // new BN(0).toBuffer('le', 8),
            ],
            program.programId
        );

    const [fight_card_account, fight_card_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('fightCard'),
                event_account.toBuffer(),
                new BN(fight_card_nonce).toBuffer(),
            ],
            program.programId
        );
    const [fight_card_link_account, fight_card_link_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('fightCard'),
                event_account.toBuffer(),
                new BN(fight_card_nonce).toBuffer(),
                provider.wallet.publicKey.toBuffer(),
                //admin_account.publicKey.toBuffer()
            ],
            program.programId
        );
    const [player_account_pda, player_account_bump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from('BattleBoosters'),
                Buffer.from('player'),
                provider.wallet.publicKey.toBuffer(),
                //admin_account.publicKey.toBuffer(),
            ],
            program.programId
        );

    // const player_account_pda_data = await program.account.playerData.fetch(
    //     player_account_pda
    // );

    const [
        fighter_mintable_game_asset_pda,
        fighter_mintable_game_asset_link_pda,
    ] = findProgramAddressSync(
        provider,
        program,
        fighter_mintable_asset_nonce,
        fighter_mintable_asset_link_nonce
    );
    const [
        energy_mintable_game_asset_pda,
        energy_mintable_game_asset_link_pda,
    ] = findProgramAddressSync(
        provider,
        program,
        energy_mintable_asset_nonce,
        energy_mintable_asset_link_nonce
    );
    const [
        shield_mintable_game_asset_pda,
        shield_mintable_game_asset_link_pda,
    ] = findProgramAddressSync(
        provider,
        program,
        shield_mintable_asset_nonce,
        shield_mintable_asset_link_nonce
    );
    const [
        points_mintable_game_asset_pda,
        points_mintable_game_asset_link_pda,
    ] = findProgramAddressSync(
        provider,
        program,
        points_mintable_asset_nonce,
        points_mintable_asset_link_nonce
    );

    const tx = await program.methods
        .joinFightCard(
            fighterColorSide
        )
        .accounts({
            signer: provider.wallet.publicKey,
            event: event_account,
            fighterAsset: fighter_mintable_game_asset_pda,
            fighterLink: fighter_mintable_game_asset_link_pda,
            shieldBoosterAsset: shield_mintable_game_asset_pda,
            pointsBoosterAsset: points_mintable_game_asset_pda,
            shieldBoosterLink: shield_mintable_game_asset_link_pda,
            pointsBoosterLink: points_mintable_game_asset_link_pda,
            fightCard: fight_card_account,
            fightCardLink: fight_card_link_account,
            eventLink: event_link_account,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([])
        .rpc();

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

    return {
        event_account,
        event_link_account,
        fight_card_account,
        fight_card_link_account,
        player_account_pda,
        fighter_mintable_game_asset_pda,
        fighter_mintable_game_asset_link_pda,
        energy_mintable_game_asset_pda,
        energy_mintable_game_asset_link_pda,
    };
};

export { joinFightCard };
