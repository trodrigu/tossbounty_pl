import * as anchor from "@coral-xyz/anchor";
import { Tossbounty } from "../target/types/tossbounty"; // Adjust according to your actual path
import { createTokenAccount } from "./token-utils"; // Helper utils for SPL operations
import {
  clusterApiUrl,
  Connection,
  Keypair,
  Transaction,
  SystemProgram,
  PublicKey,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import * as splToken from "@solana/spl-token";
import * as bs58 from "bs58";
import * as assert from "assert";

describe("Bounty Program with SPL Token Rewards", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Tossbounty as Program<Tossbounty>;
    const payer = (provider.wallet as NodeWallet).payer;
    const owner = anchor.web3.Keypair.generate();
    const vault = anchor.web3.Keypair.generate();
    const to = anchor.web3.Keypair.generate();

    it("Creates and funds a bounty with SPL tokens", async () => {
        let mintPubkey = await splToken.createMint(
          provider.connection,
          payer,
          provider.wallet.publicKey,
          provider.wallet.publicKey,
          6,
          undefined,
          undefined,
          splToken.TOKEN_PROGRAM_ID
        );

        await provider.connection.requestAirdrop(payer.publicKey, 10*LAMPORTS_PER_SOL);
        await provider.connection.requestAirdrop(to.publicKey, 10*LAMPORTS_PER_SOL);

        let fromAccount = await splToken.createAccount(
          provider.connection,
          payer,
          mintPubkey, 
          payer.publicKey, 
          undefined,
          undefined,
          splToken.TOKEN_PROGRAM_ID
        );

        let toAccount = await splToken.createAccount(
          provider.connection,
          to,
          mintPubkey, 
          to.publicKey,
          undefined,
          undefined,
          splToken.TOKEN_PROGRAM_ID
        );

        const bountyRewardAmount = new anchor.BN(100000);

        await splToken.mintTo(provider.connection, payer, mintPubkey, fromAccount, payer.publicKey, bountyRewardAmount, [], undefined, splToken.TOKEN_PROGRAM_ID);


        const [bountyPda] = await anchor.web3.PublicKey.findProgramAddress([
          anchor.utils.bytes.utf8.encode("bounty"),
          payer.publicKey.toBuffer(),
        ], program.programId)

        const ix = await program.methods.createAndFundBounty("a cool bounty", "Acme", bountyRewardAmount).accounts({
          bounty: bountyPda,
          toTokenAccount: toAccount,
          fromTokenAccount: fromAccount, 
          tokenProgram: splToken.TOKEN_PROGRAM_ID,
        });

        const tx = await ix.rpc();

        const toAccountInfo = await provider.connection.getTokenAccountBalance(toAccount);
        assert.equal(toAccountInfo.value.amount, bountyRewardAmount);
    });
});

