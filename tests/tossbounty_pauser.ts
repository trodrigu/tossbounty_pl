import * as anchor from "@coral-xyz/anchor";
import { TossbountyPauser } from "../target/types/tossbounty_pauser";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import * as splToken from "@solana/spl-token";
import assert from "assert";

describe("Pauser program", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.TossbountyPauser as Program<TossbountyPauser>;
    const example = anchor.workspace.Example as Program<Example>;
    const payer = (provider.wallet as NodeWallet).payer;
    let mintPubkey: anchor.web3.PublicKey;
    let fundingAccount: anchor.web3.PublicKey;
    let bountyRewardAmount: anchor.BN;
    let whitehatTokenAccount: anchor.web3.PublicKey;

    before(async () => {
      mintPubkey = await splToken.createMint(
        provider.connection,
        payer,
        provider.wallet.publicKey,
        provider.wallet.publicKey,
        6,
      );
    });

    it("Pauses the org program associated with the bounty", async () => {
        const [statePda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([
          anchor.utils.bytes.utf8.encode("pause"),
          payer.publicKey.toBuffer(),
        ], example.programId)

        const ix = await program.methods.pauseExample().accounts({
          programId: example.programId,
          state: statePda,
        });

        await ix.rpc();
    });
});

