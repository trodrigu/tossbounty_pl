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
    const example = anchor.workspace.Example as Program<Example>;
    const payer = (provider.wallet as NodeWallet).payer;
    let mintPubkey;
    let fundingAccount;
    let bountyRewardAmount;
    let whitehatTokenAccount;

    before(async () => {
      mintPubkey = await splToken.createMint(
        provider.connection,
        payer,
        provider.wallet.publicKey,
        provider.wallet.publicKey,
        6,
      );
    });

    it("Creates a bounty", async () => {
        await provider.connection.requestAirdrop(payer.publicKey, 10*LAMPORTS_PER_SOL);

        const balance = await provider.connection.getBalance(payer.publicKey);

        bountyRewardAmount = new anchor.BN(10000);

        fundingAccount = await splToken.createAccount(
          provider.connection,
          payer,
          mintPubkey, 
          payer.publicKey, 
        );

        const [bountyPda, bump] = anchor.web3.PublicKey.findProgramAddressSync([
          anchor.utils.bytes.utf8.encode("bounty"),
          payer.publicKey.toBuffer(),
        ], program.programId)

        const ix = await program.methods.createBountyExample("a cool bounty", "Acme", bountyRewardAmount, bump).accounts({
          bounty: bountyPda,
          fundingAccount: fundingAccount, 
          tokenProgram: splToken.TOKEN_PROGRAM_ID,
          exampleProgramId: example.programId,
        });

        const tx = await ix.rpc();
    });

    it("Claims a bounty", async () => {
        const [bountyPda, bump] = anchor.web3.PublicKey.findProgramAddressSync([
          anchor.utils.bytes.utf8.encode("bounty"),
          payer.publicKey.toBuffer(),
        ], program.programId)

        const whitehatKeypair = anchor.web3.Keypair.generate();

        await provider.connection.requestAirdrop(whitehatKeypair.publicKey, 10 * LAMPORTS_PER_SOL);

        await splToken.mintTo(provider.connection, payer, mintPubkey, fundingAccount, payer.publicKey, bountyRewardAmount, [], undefined, splToken.TOKEN_PROGRAM_ID);

        whitehatTokenAccount = await splToken.createAccount(
          provider.connection,
          whitehatKeypair,
          mintPubkey, 
          whitehatKeypair.publicKey, 
        );

        const ix = await program.methods.claimBounty().accounts({
          bounty: bountyPda,
          whitehatTokenAccount: whitehatTokenAccount, 
          fundingAccount: fundingAccount,
        });

        const tx = await ix.rpc();
    });

    it("Pauses the org program associated with the bounty", async () => {
        const [statePda, bump] = anchor.web3.PublicKey.findProgramAddressSync([
          anchor.utils.bytes.utf8.encode("pause"),
          payer.publicKey.toBuffer(),
        ], example.programId)

        await provider.connection.requestAirdrop(statePda, 10*LAMPORTS_PER_SOL);

        const balance = await provider.connection.getBalance(statePda);

        const ix = await program.methods.pauseExample().accounts({
          exampleProgramId: example.programId,
          state: statePda,
        });

        const tx = await ix.rpc();
    });
});

